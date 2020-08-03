//Shhh, I know this is bad practice but I'm lazy.
process.on('uncaughtException', function(err) {
    console.log('Caught exception: ' + err);
});

const Discord = require('discord.js');
const client = new Discord.Client();
const path = require('path');
const fs = require('fs');
const Jimp = require('jimp');
const compress_images = require('compress-images');
const util = require('util');
const axios = require('axios');
const uuid = require('uuid4');
const startTime = Date.now();
const dotenv = require('dotenv');
dotenv.config();
//Global config
const triggerChar = '!';
const notify = false;
const sizeLimit = 3e5; //Limiit max image size to 300kb after compression before we just send online link.
const stringLengthLimit = 1900; //char limit for strings
const swearList = ['fuck', 'shit', 'bitch', 'cunt', 'nigga', 'nigger', 'fucker', 'bastard', 'effing', 'slut', 'piss'];

//Help Config (currently requires manual update upon changes).
const help = [
    {
        command: 'water',
        title: 'Drink Water',
        description: 'enforce hydration.'
    },
    {
        command: 'ping',
        title: 'Ping!',
        description: 'Pong!'
    },
    {
        command: 'addTodo <title of todo>',
        title: 'Add Todo',
        description: 'add a todo to the list. Also accepts \`todo\`'
    },
    {
        command: 'removeTodo <index of todo>',
        title: 'Remove Todo',
        description: 'remove a todo from the list.'
    },
    {
        command: 'removeAll',
        title: 'Remove All Todos',
        description: 'remove all todos. Does not remove completed todos.'
    },
    {
        command: 'listTodo',
        title: 'List Todos',
        description: 'list active todos.'
    },
    {
        command: 'done <index>',
        title: 'Mark Todo Done',
        description: 'mark the todo as completed.'
    },
    {
        command: 'undone <index>',
        title: 'Mark Todo Incompleted',
        description: 'mark the todo as incomplete.'
    },
    {
        command: 'listdone',
        title: 'List Completed Todos',
        description: 'show the list of completed todos.'
    },
    {
        command: 'say',
        title: 'Speak For You',
        description: 'will say whatever you want, and delete your original message.'
    },
    {
        command: 'addImage <newCommand> <imageURL>',
        title: 'Add Custom Image',
        description: 'add a custom command which responds with an image when triggered.'
    },
    {
        command: 'listImage',
        title: 'List All Custom Images',
        description: 'list all custom image commands.'
    },
    {
        command: 'removeImage <index>',
        title: 'Remove Custom Image',
        description: 'remove a custom image from database.'
    },
    {
        command: 'uptime',
        title: 'Bot Up Time',
        description: 'how long Tom has been online for.'
    },
    {
        command: 'bfcount',
        title: 'Sam Boyfriends',
        description: `number of sam's boyfriends.`
    },
    {
        command: 'hide',
        title: 'Hide Messages',
        description: `hide recent messages with a large block of text.`
    }
]

//Constructors
const TodoItem = function(title = '', author, dateAdded = Date.now()) {
    //Check things are valid
    if (typeof title !== 'string') throw new Error('Error creating new Item, title is not of type string.');
    if (typeof dateAdded !== 'number') throw new Error('Error creating new Item, date is not of type number.');
    if (typeof author !== 'string') throw new Error('Error creating new Item, author is not of type string.')
    //Set things
    this.title = title;
    this.dateAdded = dateAdded;
    this.author = author;
    this.done = false;
    this.completedDate = undefined;
};

const TodoDataStore = function (initPath) {
    const savePath = initPath;
    
    this.data = {
        todos: [],
        completedTodos: []
    };

    this.saveFile = async () => await fs.writeFileSync(path.join(__dirname, savePath), JSON.stringify(this.data), {encoding: 'utf-8'});
    this.loadFile = async () => this.data = await JSON.parse(fs.readFileSync(path.join(__dirname, savePath)));

    this.addItem = (item) => {
        if (!(item instanceof TodoItem)) throw new Error('Error adding item to todo list. item not instance of Item');
        this.data.todos.push(item);
        this.saveFile();
        return item;
    };

    this.removeItem = (index) => {
        if (typeof index !== 'number') throw new Error('Error removing item from todo list, index not a number.');
        let removedItem = this.data.todos[index];
        this.data.todos.splice(index, 1);
        this.saveFile();
        return removedItem;
    };

    this.clearAll = () => {
        this.data.todos = [];
        this.saveFile();
    };

    this.changeDoneStatus = (index, newStatus) => {
        if (typeof index !== 'number') throw new Error('Error marking item on todo list as done, index not a number.');
        let todoEditting;
        if (newStatus) { //If true, then we are marking this todo item as completed.
            todoEditting = this.data.todos[index];
            todoEditting.done = true;
            todoEditting.completedDate = Date.now(); //If changing to done, then we set the date of completion.
            this.data.completedTodos.push(todoEditting); //Add to completed todos.
            this.removeItem(index); //Remove from original list.
        } else { //Remove completed todo.
            todoEditting = this.data.completedTodos[index];
            todoEditting.done = false;
            this.data.todos.push(todoEditting);
            this.data.completedTodos.splice(index, 1);
        }
        this.saveFile();
        return todoEditting;
    };

    this.loadFile();
};

const ImageLoader = function(command, url, author, dateAdded = Date.now()) {
    this.command = command;
    this.onlineUrl = url;
    this.author = author;
    this.dateAdded = dateAdded;
    this.ext = getExt(this.onlineUrl);
    this.name = uuid();
    this.offlineUrl;
    this.size;
    // this.checkType = async () => {
    //     if (this.ext === '.png') {
    //         console.log('Image was png, converting to jpg.');
    //         let outpath = path.join(__dirname, '/images/downloading', `${this.name}.jpg`);
    //         this.ext = '.jpg';
    //         let checkPhase = await Jimp.read(this.offlineUrl);
    //         await checkPhase.writeAsync(outpath);
    //         console.log('Deleting original png.');
    //         await fs.unlinkSync(this.offlineUrl);
    //         this.offlineUrl = outpath;
    //     }
    // }
    // this.compress = async () => {
    //     console.log('Compressing file.')
    //     INPUT_path_to_your_images = this.offlineUrl.toString().replace(/\\/g, '/');
    //     OUTPUT_path = 'images/'
    //     await util.promisify(compress_images)(INPUT_path_to_your_images, OUTPUT_path, {compress_force: false, statistic: true, autoupdate: true}, false,
    //         {jpg: {engine: 'mozjpeg', command: ['-quality', '30']}},
    //         {png: {engine: 'pngquant', command: ['--quality=20-50']}},
    //         {svg: {engine: 'svgo', command: '--multipass'}},
    //         {gif: {engine: 'gifsicle', command: ['--colors', '64', '--use-col=web']}});
    // }
    // //Download image and get a local url.
    // this.downloadImage = async () => {
    //     console.log(`Downloading image from link: ${this.onlineUrl}, which has extension ${this.ext}\n`);
    //     const writer = fs.createWriteStream(path.join(__dirname, '/images/downloading', this.name + this.ext));
    //     this.offlineUrl = path.join(__dirname, '/images/downloading', this.name + this.ext);
    //     let response = await axios({
    //         url: this.onlineUrl,
    //         method: 'GET',
    //         responseType: 'stream'
    //     });
    //     response.data.pipe(writer);
    //     await new Promise((resolve, reject) => {
    //         writer.on('finish', resolve)
    //         writer.on('error', reject)
    //     })
    //     await this.checkType();
    //     await this.compress();
    //     console.log('Deleting uncompressed jpg.');
    //     await fs.unlinkSync(this.offlineUrl); //Remove old file.
    //     this.offlineUrl = path.join(__dirname, 'images/', this.name + '.jpg'); //Set url to new path.
    //     this.size = await fs.statSync(this.offlineUrl).size; //Get size of image.
    // };
};

const persistanceDataStore = function(initPath) {
    const savePath = initPath;
    this.data = {
        bfcount: 15000
    }
    this.saveFile = async () => await fs.writeFileSync(path.join(__dirname, savePath), JSON.stringify(this.data), {encoding: 'utf-8'});
    this.loadFIle = async () => this.data = await JSON.parse(fs.readFileSync(path.join(__dirname, savePath)));
};

const ImageReplyDataStore = function(initPath) {
    const savePath = initPath;
    this.data = {
        images: {},
        commands: []
    }

    async function checkType(image) {
        if (image.ext === '.png') {
            console.log('Image was png, converting to jpg.');
            let outpath = path.join(__dirname, '/images/downloading', `${image.name}.jpg`);
            image.ext = '.jpg';
            let checkPhase = await Jimp.read(image.offlineUrl);
            await checkPhase.writeAsync(outpath);
            console.log('Deleting original png.');
            await fs.unlinkSync(image.offlineUrl);
            image.offlineUrl = outpath;
        }
        return image;
    }

    async function compress(image) {
        const noCompressExts = ['.mp4'];
        if (noCompressExts.includes(image.ext)) return image;
        console.log('Compressing file.')
        INPUT_path_to_your_images = image.offlineUrl.toString().replace(/\\/g, '/');
        OUTPUT_path = 'images/'
        await util.promisify(compress_images)(INPUT_path_to_your_images, OUTPUT_path, {compress_force: false, statistic: true, autoupdate: true}, false,
            {jpg: {engine: 'mozjpeg', command: ['-quality', '30']}},
            {png: {engine: 'pngquant', command: ['--quality=20-50']}},
            {svg: {engine: 'svgo', command: '--multipass'}},
            {gif: {engine: 'giflossy', command: ['--lossy=80']}});
        console.log('Deleting uncompressed file.');
        await fs.unlinkSync(image.offlineUrl); //Remove old file.
        image.offlineUrl = path.join(__dirname, 'images/', image.name + image.ext); //Set url to new path.
        return image;
    }

    async function downloadImage(image) {
        console.log(`Downloading image from link: ${image.onlineUrl}, which has extension ${image.ext}\n`);
        const writer = fs.createWriteStream(path.join(__dirname, '/images/downloading', image.name + image.ext));
        image.offlineUrl = path.join(__dirname, '/images/downloading', image.name + image.ext);
        let response = await axios({
            url: image.onlineUrl,
            method: 'GET',
            responseType: 'stream'
        });
        response.data.pipe(writer);
        await new Promise((resolve, reject) => {
            writer.on('finish', resolve)
            writer.on('error', reject)
        })
        image = await checkType(image); //If png convert to jpg.
        image = await compress(image);
        image.size = await fs.statSync(image.offlineUrl).size; //Get size of image.
        return image;
    }

    this.saveFile = async () => await fs.writeFileSync(path.join(__dirname, savePath), JSON.stringify(this.data), {encoding: 'utf-8'});
    this.loadFile = async () => this.data = await JSON.parse(fs.readFileSync(path.join(__dirname, savePath)));
    this.addImage = async (image) => {
        if (!(image instanceof ImageLoader)) throw new Error('Error adding image to reply data store. \'image\' not instance of ImageLoader.');
        if (this.data.commands.includes(image.command)) throw new Error('Error adding image to reply data store. Command already exists, no duplicates allowed.');
        image = await downloadImage(image);
        if (image.size > sizeLimit) {
            console.log('Unable to compress image to a satisfactory amount. Will send online link instead of uploading image.')
            //Message should also be sent to the user in discord to inform them of this info.
        }
        this.data.images[image.command] = image;
        this.data.commands.push(image.command);
        await this.saveFile();
    };

    this.removeImage = async (index) => {
        if (typeof index !== 'number') throw new Error('Error removing image from data store, as index is not a number.');
        if (index - 1 > this.data.commands.length) throw new Error('Error removing image from data store. Index provided is too large.');
        const removedCommand = this.data.commands[index];
        const removedImage = this.data.images[removedCommand];
        await fs.unlinkSync(removedImage.offlineUrl); //Delete image from disk.
        this.data.commands.splice(index, 1); //Remove command.
        delete this.data.images[removedCommand]; //Remove image from store.
        await this.saveFile();
        return removedImage;
    };

    this.loadFile();
};

//Helper Functions
function getDiscordTagFromId(userId) {
    const user = client.users.cache.get(userId); // Getting the user by ID.
    return (user) ? user.tag : 'User not found.';
}

function formatDate(ms) {
    let date = new Date(ms);
    return `${date.getDate()}/${date.getMonth()+1}/${date.getFullYear()}`;
}

function formatTime(ms) {
    let date = new Date(ms);
    return `${date.getHours()}:${date.getMinutes()}`;
}

function todoListCreateResponse(scopedData) {
    //returns an array of message strings which can then be combined to form a full message later on.
    let messageStrings = [];
    if (scopedData.length > 0) {
        messageStrings.push('Todo List:\n');
        scopedData.forEach((todoItem, index) => {
            let { title, author, dateAdded, done, completedDate } = todoItem;
            let newItem = `${index}). ${title} created by: ${getDiscordTagFromId(author)} on ${formatDate(dateAdded)} at ${formatTime(dateAdded)}${(done) ? '. Completed on ' + formatDate(completedDate) + ' at ' + formatTime(completedDate) : ''}\n`;
            messageStrings.push(newItem);
        });
    } else {
        messageStrings.push("I'm sorry, there are no items on this list.");
    }
    return messageStrings;
} 

function getExt(url) {
    url = url.split('?')[0];
    url = url.split('/')[url.split('/').length - 1];
    return url.includes('.') ? url.substring(url.lastIndexOf('.')) : '';
}

function chunkString(str, length) { //Copied from: https://stackoverflow.com/questions/7033639/split-large-string-in-n-size-chunks-in-javascript/10915724
    return str.match(new RegExp(`/(.|[\r\n]){1,${length}}/g`));
}

function avoidMessageSizeOverload(stringArray) {
    if (!Array.isArray(stringArray)) stringArray = [stringArray]; //Ensure array.
    //First we need to do a check on each string in the string array. If any string exceeds the maximum amount for a single message, then split it into two s
    stringArray = stringArray.reduce((culm, curr) => {
        if (typeof curr !== 'string') throw new Error('Error trying to avoid message overload. Non-string object encountered.');
        if (curr.length > stringLengthLimit) {
            culm = [...culm, ...chunkString(curr, stringLengthLimit)];
        } else {
            culm.push(curr);
        }
        return culm;
    }, []); 
    //Then we want to combine strings into message 'chunks' which can be sent at under 2K m.
    let messageArray;
    messageArray = stringArray.reduce((culm, curr) => {
        let currIndex = culm.length -1;
        if (culm[currIndex].length + curr.length >= stringLengthLimit) {
            //String will be too long and exceed char limit.
            culm.push(curr);
        } else {
            //Current string can be appended no problems.
            culm[currIndex] += curr; //Add current string to existing last message string.
        }
        return culm;
    }, ['']);
    return messageArray;
};

function timeSince(date) { //Copied from: https://stackoverflow.com/questions/3177836/how-to-format-time-since-xxx-e-g-4-minutes-ago-similar-to-stack-exchange-site

    var seconds = Math.floor((new Date() - date) / 1000);

    var interval = Math.floor(seconds / 31536000);

    if (interval > 1) {
      return interval + " years";
    }
    interval = Math.floor(seconds / 2592000);
    if (interval > 1) {
      return interval + " months";
    }
    interval = Math.floor(seconds / 86400);
    if (interval > 1) {
      return interval + " days";
    }
    interval = Math.floor(seconds / 3600);
    if (interval > 1) {
      return interval + " hours";
    }
    interval = Math.floor(seconds / 60);
    if (interval > 1) {
      return interval + " minutes";
    }
    return Math.floor(seconds) + " seconds";
  }

//Global Vars
let todoStore = new TodoDataStore('todoData.json');
let imageStore = new ImageReplyDataStore('imageData.json');
let configStore = new persistanceDataStore('generalData.json');

//Listeners
client.on('ready', () => {
    console.log(`Logged in as ${client.user.tag}!`);
    client.user.setActivity(`getting ${triggerChar}help`);
});

client.on('message', message => {
    let { channel, id, content, author } = message;
    if (message.author.bot) return;
    //Check for swears
    for (let i = 0; i < swearList.length; i++) {
        if (message.content.toLowerCase().includes(swearList[i])) {
            console.log(`${message.author.tag} tried to use profanity.`);
            message.react('734648902032162868');
        }
      }
    //Normal Command Phase
    if (content.substring(0, triggerChar.length) === triggerChar) {
        let args = content.toLowerCase().substring(1).split(' ');
        switch (args[0]) {
            case 'ping':
                client.channels.resolve(channel.id).send('pong');
                break;
            case 'help':
                let messageStrings = ['Greetings! The current commands are as follows:\n'];
                help.forEach((helpItem, index) => {
                    let { title, command, description } = helpItem;
                    messageStrings.push((`${index}). '${title}' can be triggered with \`${triggerChar}${command}\` and will ${description}\n`));
                });
                let messageArray = avoidMessageSizeOverload(messageStrings);
                messageArray.forEach(messageContent => {
                    message.channel.send(`${messageContent}`); //Avoid overload by sending multiple messages when an overload is possible.
                });
                message.react('👍'); //React to say we have acknowledged the request. 
                break;
            case 'water':
                message.channel.send(null, {files: ['./images/stayHydrated.jpg']});
                break;
            case 'todo':
            case 'todoadd':
            case 'addtodo': {
                let title = content.slice((`${triggerChar}${args[0]} `.length)); //Remove the first chunk of the string and then set the rest as title.
                if (title.length === 0) {
                    message.channel.send('Oops, looks like you forgot to tell me what to add!');
                    break;
                }
                if (title.length > stringLengthLimit) {
                    message.channel.send('Oops, you have too many chars in your todo!');
                    break;
                }
                let newItem = new TodoItem(title, message.member.id);
                todoStore.addItem(newItem);
                if (notify) message.channel.send(`Added '${title}' to the todo list!`);
                message.react('👍'); //React to say we have acknowledged the request.
                break;
            }
            case 'todoremove':
            case 'removetodo': {
                if (args[1] === undefined || args[1] === null) {
                    message.channel.send('Oops, looks like you forgot to tell me what to remove!');
                    break;
                }
                let removeIndex = Number(args[1]);
                let removed = todoStore.removeItem(removeIndex);
                if (notify) message.channel.send(`'${removed.title}' was removed from the todo list.`);
                message.react('👍'); //React to say we have acknowledged the request.
                break;
            }
            case 'removeall': {
                todoStore.clearAll();
                if (notify) message.channel.send(`All todos cleared.`);
                message.react('👍'); //React to say we have acknowledged the request.
                break;
            }
            case 'todos':
            case 'listtodos':
            case 'todolist':
            case 'todoslist':
            case 'listtodo': {
                let scopedData = todoStore.data.todos;
                let stringArray = todoListCreateResponse(scopedData);
                let messageArray = avoidMessageSizeOverload(stringArray);
                messageArray.forEach(messageContent => {
                    message.channel.send(`\`\`\`${messageContent}\`\`\``); //Avoid overload by sending multiple messages when an overload is possible.
                });
                break;
            }
            case 'done': {
                if (args[1] === undefined || args[1] === null) {
                    message.channel.send('Oops, looks like you forgot to tell me what to mark as completed!');
                    break;
                }
                let editIndex = Number(args[1]);
                let completed = todoStore.changeDoneStatus(editIndex, true);
                if (notify) message.channel.send(`'${completed.title}' has been marked as done!`);
                message.react('👍'); //React to say we have acknowledged the request.
                break;
            }
            case 'undone': {
                if (args[1] === undefined || args[1] === null) {
                    message.channel.send('Oops, looks like you forgot to tell me what to mark as uncompleted!');
                    break;
                }
                let editIndex = Number(args[1]);
                let undone = todoStore.changeDoneStatus(editIndex, false);
                if (notify) message.channel.send(`'${undone.title}' has been marked as uncompleted.`)
                message.react('👍'); //React to say we have acknowledged the request.
                break;
            }
            case 'donelist':
            case 'listdone': {
                let scopedData = todoStore.data.completedTodos;
                let stringArray = todoListCreateResponse(scopedData);
                let messageArray = avoidMessageSizeOverload(stringArray);
                messageArray.forEach(messageContent => {
                    message.channel.send(`\`\`\`${messageContent}\`\`\``); //Avoid overload by sending multiple messages when an overload is possible.
                });
                break;
            }
            case 'say': {
                let messageContent = content.slice((`${triggerChar}${args[0]} `.length));
                if (messageContent.length === 0) {
                    message.channel.send('Oops, looks like you forgot to tell me what to add!');
                    break;
                }
                message.delete({timeout: 5}); //Supposed to delete message
                message.channel.send(messageContent);
                break;
            }
            case 'imageadd':
            case 'addimage': {
                if (args.length < 2 && message.attachments.size === 0) {
                    message.channel.send('Oops! You have not included all of the required parameters.');
                    break;
                }
                let url;
                if (args.length > 2) {
                    let urlStartIndex = (`${triggerChar}${args[0]} ${args[1]} `.length);
                    url = content.slice(urlStartIndex, urlStartIndex + args[2].length);
                } else {
                    // console.log(message.attachments.entries().next().value)
                    url = (message.attachments.entries().next().value)[1].url;
                }
                let command = args[1];
                let author = message.member.id;
                if (imageStore.data.commands.includes(command)) {
                    message.channel.send('Oops! That command is already taken. Consider using a different command - or remove the existing command first.');
                    break;
                }
                if (command.length > 100) {
                    message.channel.send('Oops, your command is too long!');
                    break;
                }
                let newImage = new ImageLoader(command, url, author);
                let addedImage = imageStore.addImage(newImage);
                if (notify) message.channel.send(`Image with command \`${addedImage.command}\` has been added to the list!`);
                message.react('👍');
                break;
            }
            case 'imagelist':
            case 'imageslist':
            case 'listimages':
            case 'listimage': {
                let scopedData = Object.values(imageStore.data.images);
                let messageStrings = [];
                if (scopedData.length > 0) {
                    messageStrings.push('Image Commands:\n');
                    scopedData.forEach((imageItem, index) => {
                        let { command, author, dateAdded } = imageItem;
                        let newItem = `${index}). Triggered with \`${command}\` added by ${getDiscordTagFromId(author)} on ${formatDate(dateAdded)} at ${formatTime(dateAdded)}\n`;
                        messageStrings.push(newItem);
                    });
                } else {
                    messageStrings.push("I'm sorry, there are no items on this list.");
                }
                let messageArray = avoidMessageSizeOverload(messageStrings);
                messageArray.forEach(messageContent => {
                    message.channel.send(`\`\`\`${messageContent}\`\`\``); //Avoid overload by sending multiple messages when an overload is possible.
                });
                break;
            }
            case 'imageremove':
            case 'removeimage': {
                if (args.length < 2) {
                    message.channel.send('Oops! Looks like you forgot to tell me which image to remove.');
                    break;
                }
                const removeIndex = Number(args[1]);
                const removedImage = imageStore.removeImage(removeIndex);
                if (notify) message.channel.send(`Image with command \`${removedImage.command}\` was removed from the list.`);
                message.react('👍');
                break;
            }
            case 'uptime': {
                message.channel.send(`I have been online for ${timeSince(startTime)}.`);
                message.react('👍');
                break;
            }
            case 'datingtime': {
                const msStartedDating = 1594758600000;
                message.channel.send(`We started dating on 15/07/2020 at roughly 8:30am.\nThis means we have been dating for ${timeSince(msStartedDating)}.\n\nGuess what?`);
                setTimeout(() => {
                    message.channel.send(`I still love you. <3`);
                }, 4000);
                break;
            }
            case 'bfcount': {
                configStore.data.bfcount += Math.floor((Math.random() * 100000)); //Update file with new count.
                configStore.saveFile(); //Save file back to disk.
                message.channel.send(`Sam currently has ${configStore.data.bfcount} boyfriends, that's crazy!`);
                break;
            }
            case 'hide': {
                message.channel.send(`​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​​\n​​\n​​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​​\n​​\n​​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n`);
                break;
            }
            default:
                //If we get to here the command has not been found in the regular table.
                if (!imageStore.data.commands.includes(args[0])) { //If the command is not in the table, then we just use the regular error response.
                    console.log(`I was called with: ${args}, but was unable to process the request.`);
                    if (notify) message.channel.send(`I was called with: ${args}, but was unable to process the request.`);
                    break;
                }
                //Dynamic Command Time
                let customImage = imageStore.data.images[args[0]];
                if (customImage.size >= sizeLimit) {
                    message.channel.send(customImage.onlineUrl);
                } else {
                    message.channel.send(null, {files: [customImage.offlineUrl]});
                }
        }
    }
});

client.login(process.env.BOTSECRET);
