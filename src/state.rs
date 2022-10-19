use std::sync::{Arc, RwLock};

use serenity::prelude::TypeMapKey;

use crate::google_api::maps::GoogleMapsApiHandle;

/// A connection to the database, representing the stored "state" of the app
pub struct AppState {
    pub google_api: Arc<RwLock<Option<GoogleMapsApiHandle>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            google_api: Arc::new(RwLock::new(None)),
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
        }
    }
}

impl TypeMapKey for AppState {
    type Value = AppState;
}
