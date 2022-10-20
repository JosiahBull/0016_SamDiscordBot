use serenity::builder::CreateEmbed;

use crate::{google_api::maps::GoogleMapsData, state::AppState};

pub const DESTINATIONS: &[[&str; 2]] = &[
    ["UoA", "University of Auckland"],
    [
        "Massey",
        "Massey University East Precinct Albany Expressway, SH17, Albany, Auckland 0632",
    ],
    [
        "Zerojet",
        "5 Te Apunga Place, Mount Wellington, Auckland 1060",
    ],
    ["Crown", "65 Hugo Johnston Drive, Penrose, Auckland, 1061"],
];

pub const PHRASES: &[&str] = &[
    "Auckland's getting congested again!",
    "Auckland costs an arm and a leg.",
    "Crash, bang! Whallop!",
    "But it's still cheaper than Southern Legoland.",
    "If only we all lived in places with more engineers.",
    "Urgh. Truckboys have zero manners.",
    "I'm going to go to bed now.",
    "You know, they really should widen or bypass the motorway.",
    "Powered by broken hopes and dreams",
    "Hey, are you Deaf? 47db is full throttle!",
    "For tours of shame, rates are by negotiation.",
    "Running on 3.4 potato cores, at 800mhz.",
    "Whoa, 6x. That's some user dodging skill right there.",
    "Be advised. This bot is not authorized for medical advice.",
    "I take no responsibility for victims of extreme awesomeness.",
    "This bot brought to you by the letter M.",
    "WARNING: Pulsing so hard, it could combust.",
    "WARNING. Warning. WARNING!",
    "WARNING : Asthma sufferers may experience an attack.",
    "(╯°□°）╯︵ ┻━┻",
    "╰( ͡° ͜ʖ ͡° )つ──☆*:・ﾟ",
    "Are you a ghost? Because you're transparency.",
    "Warning: may contain caffeine",
    "Some assembly required (Warning: not supplied)",
    "Is this your card? Because I would like to play it.",
    "Not intended for people of low self-esteem.",
    "Try not to press too many buttons. Someone may die.",
    "No, this is not clickbait.",
    "Even a circle has three sides sometimes. Try holding it down and twiddling your fingers for 2 seconds.",
    "Attracting computer thieves since 2017.",
    "Woot! Get your Woot on.",
    "Keeping your logo in the cloud since '15",
    "D*d players",
    "...",
    "You're wrong. I like coffee. Sorry.",
    "You're right. I like tea.",
    "What evuh evuh evuh ever happened to... *snort*",
    "Dude, have you been drinking?",
    "I don't want to get out of bed!",
    "I think I could sell you a rock.",
    "I don't push buttons for the big boss, I push buttons for the fat bosses.",
    "WARNING WARNING WARNING WARNING",
    "Dischord. That's my diss",
    "This product does less than nothing.",
    "Improvements to this product will be remedied once I'm in a better mood.",
    "When finishing this product, I feel like I've done a negative amount of work.",
    "Cool story, Ben.",
    "I have one word for you...",
];

// funny comments, Powered by...
pub const POWERED_BY: &[&str] = &[
    "your mother",
    "western technology",
    "good friends",
    "lots of coffee",
    "sad stories",
    "lost children",
    "water filled canisters",
    "burning desire",
    "my hatred of your oppressors",
    "real stories",
    "fictional proffesor from fictional universities",
    "urban travellers",
    "water coolers",
    "fluffy bunny rabbits",
    "old keyboards",
    "powerguy92",
    "20,000 jager bombs fully taped to my car",
    "my old math teacher",
    "pastries and other sweets",
    "freshly squeezed orange juice",
    "working computers",
    "the sun",
    "slurm",
    "what you shall nae know",
    "your tears",
];

pub async fn load_maps_data_to_embed(
    address: String,
    state: &AppState,
) -> Result<CreateEmbed, Box<dyn std::error::Error + Send + Sync + 'static>> {
    // create a oneshot channel to await the response
    let (tx, rx) = tokio::sync::oneshot::channel();

    // make a global request for the address
    state
        .maps_api()
        .add_to_queue(address, DESTINATIONS, tx)
        .await;

    // wait for the oneshot channel to return (maximum of 20 seconds)
    let data: GoogleMapsData =
        tokio::time::timeout(std::time::Duration::from_secs(20), rx).await???;

    let mut embed = CreateEmbed::default();

    let embed = embed
        .title(&data.origin_addresses[0])
        .footer(|f| {
            f.text(PHRASES[rand::random::<usize>() % PHRASES.len()])
                .icon_url("https://cdn.iconscout.com/icon/free/png-256/google-map-461800.png")
        })
        .color(0x4285F4);

    for row in data.rows.iter() {
        for (i, element) in row.elements.iter().enumerate() {
            embed.field(
                DESTINATIONS[i][0],
                format!("{} ({})", element.distance.text, element.duration.text),
                true,
            );
        }
    }

    Ok(embed.to_owned())
}
