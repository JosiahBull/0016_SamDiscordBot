use serenity::builder::{CreateEmbed, CreateEmbedFooter};

use crate::{
    google_api::maps::GoogleMapsData,
    state::{AppState, PHRASES},
};

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

    let embed = CreateEmbed::default();

    let mut embed = embed
        .title(&data.origin_addresses[0])
        .footer(
            CreateEmbedFooter::new(PHRASES[rand::random::<usize>() % PHRASES.len()])
                .icon_url("https://cdn.iconscout.com/icon/free/png-256/google-map-461800.png"),
        )
        .color(0x4285F4);

    for row in data.rows.iter() {
        for (i, element) in row.elements.iter().enumerate() {
            embed = embed.field(
                DESTINATIONS[i][0],
                format!("{} ({})", element.distance.text, element.duration.text),
                true,
            );
        }
    }

    Ok(embed)
}
