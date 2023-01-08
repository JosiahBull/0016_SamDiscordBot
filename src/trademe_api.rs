use fantoccini::{Client, ClientBuilder};
use log::error;
use serde::Deserialize;
use serde_json::json;
use tokio::sync::mpsc::{Receiver, Sender};

pub type TrademeApiResponse = Result<TrademeApiData, TrademeError>;

#[derive(Debug)]
pub enum TrademeError {
    NetworkError,
    InvalidAddress,
}

impl std::fmt::Display for TrademeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidAddress => write!(f, "Invalid address"),
            Self::NetworkError => write!(f, "Network error"),
        }
    }
}

impl std::error::Error for TrademeError {}

#[derive(Deserialize, Debug, Clone)]
pub struct TrademeApiData {
    pub address: String,
    pub price: u64,
}

#[derive(Debug)]
struct TrademeApiRequest {
    listing_url: String,
    sender: tokio::sync::oneshot::Sender<TrademeApiResponse>,
}

#[derive(Debug)]
pub struct TrademeApiBuilder {
    gecko_driver_url: Option<String>,
}

impl TrademeApiBuilder {
    pub fn gecko_driver_url(mut self, url: String) -> Self {
        self.gecko_driver_url = Some(url);
        self
    }

    pub async fn build(self) -> TrademeApi {
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        // use fantoccini to get the listing details
        let mut caps = serde_json::Map::new();
        caps.insert("browserName".to_string(), "firefox".into());
        caps.insert(
            "moz:firefoxOptions".to_string(),
            json!({"args": ["-headless"], "prefs": {"permissions.default.image": 2}}),
        );

        let mut attempts = 0;
        let client;
        loop {
            let temp_client = ClientBuilder::native()
                .capabilities(caps.clone())
                .connect(self.gecko_driver_url.as_ref().unwrap())
                .await;

            if let Ok(c) = temp_client {
                client = c;
                break;
            }

            if attempts > 5 {
                panic!("Failed to connect to geckodriver");
            }

            // use docker command to restart the `geckodriver` container
            tokio::process::Command::new("docker")
                .arg("restart")
                .arg("geckodriver")
                .output()
                .await
                .expect("Failed to restart geckodriver container");

            tokio::time::sleep(std::time::Duration::from_secs(5)).await;

            error!("Failed to connect to geckodriver. Retrying...");

            attempts += 1;
        }

        TrademeApi {
            client,
            internal_receiver: rx,
            internal_sender: tx,
        }
    }
}

#[derive(Debug)]
pub struct TrademeApi {
    client: Client,
    internal_receiver: Receiver<TrademeApiRequest>,
    internal_sender: Sender<TrademeApiRequest>,
}

impl TrademeApi {
    pub fn builder() -> TrademeApiBuilder {
        TrademeApiBuilder {
            gecko_driver_url: None,
        }
    }

    async fn get_listing_details(&mut self, listing_url: String) -> TrademeApiResponse {
        if let Err(e) = self.client.goto(&listing_url).await {
            eprintln!("goto() error. {:?}", e);
            return Err(TrademeError::NetworkError);
        }

        // load .tm-property-listing-body__location as address
        let address = self
            .client
            .find(fantoccini::Locator::Css(
                ".tm-property-listing-body__location",
            ))
            .await
            .map_err(|e| {
                eprintln!("Error finding address: {:?}", e);
                TrademeError::InvalidAddress
            })?
            .text()
            .await
            .map_err(|e| {
                eprintln!("Error extracting address: {:?}", e);
                TrademeError::InvalidAddress
            })?;

        // load .tm-property-listing-body__price > strong:nth-child(1) as the price
        let price = self
            .client
            .find(fantoccini::Locator::Css(
                ".tm-property-listing-body__price > strong:nth-child(1)",
            ))
            .await
            .map_err(|e| {
                eprintln!("Error finding price: {:?}", e);
                TrademeError::InvalidAddress
            })?
            .text()
            .await
            .map_err(|e| {
                eprintln!("Error extracting price: {:?}", e);
                TrademeError::InvalidAddress
            })?
            .replace('$', "")
            .replace(" per week", "")
            .parse::<u64>()
            .map_err(|e| {
                eprintln!("Error parsing price: {:?}", e);
                TrademeError::InvalidAddress
            })?;

        Ok(TrademeApiData { address, price })
    }

    pub async fn run(&mut self) {
        while let Some(request) = self.internal_receiver.recv().await {
            let response = self.get_listing_details(request.listing_url).await;
            request.sender.send(response).unwrap();
        }
    }

    pub fn handle(&self) -> TrademeApiHandle {
        TrademeApiHandle {
            internal_sender: self.internal_sender.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrademeApiHandle {
    internal_sender: Sender<TrademeApiRequest>,
}

impl TrademeApiHandle {
    pub async fn add_to_queue(
        &self,
        listing_url: String,
        return_channel: tokio::sync::oneshot::Sender<TrademeApiResponse>,
    ) {
        self.internal_sender
            .send(TrademeApiRequest {
                listing_url,
                sender: return_channel,
            })
            .await
            .unwrap();
    }
}
