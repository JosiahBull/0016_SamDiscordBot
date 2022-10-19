use std::time::Instant;

use reqwest::Client;
use serde::Deserialize;
use tokio::sync::mpsc::{Receiver, Sender};

const API_URL: &str = "https://maps.googleapis.com/maps/api/distancematrix/json";

pub type GoogleMapApiResponse = Result<GoogleMapsData, GoogleMapError>;

#[derive(Debug)]
pub enum GoogleMapError {
    NetworkError,
    APILimitReached,
    InvalidAddress,
    // Unknown(String),
}

impl std::fmt::Display for GoogleMapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::APILimitReached => write!(f, "API limit reached"),
            Self::InvalidAddress => write!(f, "Invalid address"),
            Self::NetworkError => write!(f, "Network error"),
            // Self::Unknown(s) => write!(f, "Unknown error: {}", s),
        }
    }
}

impl std::error::Error for GoogleMapError {}

#[derive(Deserialize, Debug, Clone)]
pub struct GoogleMapsData {
    pub destination_addresses: Vec<String>,
    pub origin_addresses: Vec<String>,
    pub rows: Vec<GoogleMapsRow>,
    pub status: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GoogleMapsRow {
    pub elements: Vec<GoogleMapsElement>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GoogleMapsElement {
    pub distance: GoogleMapsDistance,
    pub duration: GoogleMapsDuration,
    pub status: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GoogleMapsDistance {
    pub text: String,
    pub value: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GoogleMapsDuration {
    pub text: String,
    pub value: u32,
}

#[derive(Debug)]
struct GoogleMapsRequest {
    origin: String, //XXX: would be nice to have this borrowed too
    destinations: &'static [[&'static str; 2]],
    sender: tokio::sync::oneshot::Sender<GoogleMapApiResponse>,
}

#[derive(Debug)]
pub struct MapsApiBuilder {
    key: Option<String>,
}

impl MapsApiBuilder {
    pub fn new() -> Self {
        Self { key: None }
    }

    pub fn key(mut self, key: String) -> Self {
        self.key = Some(key);
        self
    }

    pub fn build(self) -> GoogleMapsApi {
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        GoogleMapsApi {
            client: Client::new(),
            key: self.key.unwrap(),
            internal_receiver: rx,
            internal_sender: tx,
            timeout: None,
        }
    }
}

#[derive(Debug)]
pub struct GoogleMapsApi {
    client: Client,
    key: String,
    internal_receiver: Receiver<GoogleMapsRequest>,
    internal_sender: Sender<GoogleMapsRequest>,
    timeout: Option<Instant>,
}

impl GoogleMapsApi {
    pub fn builder() -> MapsApiBuilder {
        MapsApiBuilder::new()
    }

    async fn get_distance(
        &mut self,
        origin: &str,
        destinations: &[[&str; 2]],
    ) -> GoogleMapApiResponse {
        //TODO create a check to see if this request has previously been processed

        if let Some(timeout) = self.timeout {
            if timeout > Instant::now() {
                return Err(GoogleMapError::APILimitReached);
            }
            self.timeout = None;
        }

        let url: String = {
            let mut url = String::from(API_URL);
            url.push_str("?units=metric");
            url.push_str("&key=");
            url.push_str(&self.key);
            url.push_str("&origins=");
            url.push_str(origin);
            url.push_str("&destinations=");
            url.push_str(
                &destinations
                    .iter()
                    .map(|x| x[1])
                    .collect::<Vec<&str>>()
                    .join("|"),
            );
            url
        };

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|_| GoogleMapError::NetworkError)?;

        // validate status
        if !response.status().is_success() {
            // check if we are rate limited
            if response.status() == 403 {
                self.timeout = Some(Instant::now() + std::time::Duration::from_secs(60 * 60));
                return Err(GoogleMapError::APILimitReached);
            }

            return Err(GoogleMapError::NetworkError);
        }

        let data: GoogleMapsData = response
            .json()
            .await
            .map_err(|_| GoogleMapError::InvalidAddress)?;

        Ok(data)
    }

    pub async fn run(&mut self) {
        loop {
            let request = self.internal_receiver.recv().await.unwrap();
            let response = self
                .get_distance(&request.origin, request.destinations)
                .await;
            request.sender.send(response).unwrap();
        }
    }

    pub fn handle(&self) -> GoogleMapsApiHandle {
        GoogleMapsApiHandle {
            internal_sender: self.internal_sender.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GoogleMapsApiHandle {
    internal_sender: Sender<GoogleMapsRequest>,
}

impl GoogleMapsApiHandle {
    pub async fn add_to_queue(
        &self,
        origin: String,
        destinations: &'static [[&'static str; 2]],
        return_channel: tokio::sync::oneshot::Sender<GoogleMapApiResponse>,
    ) {
        self.internal_sender
            .send(GoogleMapsRequest {
                origin,
                destinations,
                sender: return_channel,
            })
            .await
            .unwrap();
    }
}
