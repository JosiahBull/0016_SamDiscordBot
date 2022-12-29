use std::sync::{Arc, RwLock};

use serenity::prelude::TypeMapKey;

use crate::{google_api::maps::GoogleMapsApiHandle, trademe_api::TrademeApiHandle};

pub const HEAD_TENANT_ACC_NUMBER: &str = "***REMOVED***"; // TODO: load from env

pub struct Flatmate<'a> {
    pub discord_id: u64,
    pub name: &'a str,
    pub display_name: &'a str,
}
// TODO: load this from env
pub const FLATMATES: &[Flatmate<'static>] = &[
    Flatmate {
        discord_id: ***REMOVED***,
        name: "***REMOVED***",
        display_name: "***REMOVED***",
    },
    Flatmate {
        discord_id: ***REMOVED***,
        name: "***REMOVED***",
        display_name: "***REMOVED***",
    },
    Flatmate {
        discord_id: ***REMOVED***,
        name: "***REMOVED***",
        display_name: "***REMOVED***",
    },
    Flatmate {
        discord_id: ***REMOVED***,
        name: "***REMOVED***",
        display_name: "***REMOVED***",
    },
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

/// A connection to the database, representing the stored "state" of the app
pub struct AppState {
    pub google_api: Arc<RwLock<Option<GoogleMapsApiHandle>>>,
    pub trademe_api: Arc<RwLock<Option<TrademeApiHandle>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            google_api: Arc::new(RwLock::new(None)),
            trademe_api: Arc::new(RwLock::new(None)),
        }
    }

    pub fn set_google_api(&mut self, google_api: GoogleMapsApiHandle) {
        let mut google_api_guard = self.google_api.write().unwrap();
        *google_api_guard = Some(google_api);
    }

    pub fn maps_api(&self) -> GoogleMapsApiHandle {
        let google_api = self.google_api.read().unwrap();
        google_api.as_ref().unwrap().clone()
    }

    pub fn set_tradme_api(&mut self, trademe_api: TrademeApiHandle) {
        let mut trademe_api_guard = self.trademe_api.write().unwrap();
        *trademe_api_guard = Some(trademe_api);
    }

    pub fn trademe_api(&self) -> TrademeApiHandle {
        let trademe_api = self.trademe_api.read().unwrap();
        trademe_api.as_ref().unwrap().clone()
    }
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState").finish()
    }
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            google_api: self.google_api.clone(),
            trademe_api: self.trademe_api.clone(),
        }
    }
}

impl TypeMapKey for AppState {
    type Value = AppState;
}
