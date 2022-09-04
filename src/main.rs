mod commands;

use dotenv::dotenv;
use std::fmt::Debug;

/// A connection to the database, representing the stored "state" of the app
pub struct AppState {

}

impl Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState").finish()
    }
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {  }
    }
}


pub async fn run_discord_server() -> Result<(), Box<dyn std::error::Error>> {


    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN")?;




    Ok(())
}
