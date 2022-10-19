use std::time::Instant;

use reqwest::Client;
use serde::Deserialize;
use tokio::sync::mpsc::{Receiver, Sender};

const API_URL: &str = "";

pub type TrademeApiResponse = Result<TrademeApiData, TrademeError>;

#[derive(Debug)]
pub enum TrademeError {
    NetworkError,
    APILimitReached,
    InvalidAddress,
}

impl std::fmt::Display for TrademeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::APILimitReached => write!(f, "API limit reached"),
            Self::InvalidAddress => write!(f, "Invalid address"),
            Self::NetworkError => write!(f, "Network error"),
        }
    }
}

impl std::error::Error for TrademeError {}

#[derive(Deserialize, Debug, Clone)]
pub struct TrademeApiData {
    category: Option<String>,
    title: Option<String>,
    subtitle: Option<String>,
    description: Option<Vec<String>>,
    start_price: u64,
    reserve_price: u64,
    buy_now_price: u64,
    // duration:
}

struct TrademeApiRequest {
    listing_id: u64,
    sender: tokio::sync::oneshot::Sender<TrademeApiResponse>,
}

#[derive(Debug)]
pub struct TrademeApiBuilder {}

impl TrademeApiBuilder {
    pub fn build(self) -> TrademeApi {
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        TrademeApi {
            client: Client::new(),
            internal_receiver: rx,
            internal_sender: tx,
            timeout: None,
        }
    }
}

#[derive(Debug)]
pub struct TrademeApi {
    client: Client,
    internal_receiver: Receiver<TrademeApiRequest>,
    internal_sender: Sender<TrademeApiRequest>,
    timeout: Option<Instant>,
}

impl TrademeApi {
    pub fn builder() -> TrademeApiBuilder {
        TrademeApiBuilder {}
    }

    async fn get_listing_details(&mut self, listing_id: u64) -> TrademeApiResponse {}

    pub async fn run(&mut self) {}

    pub fn handle(&self) -> TrademeApiHandle {
        TrademeApiHandle {
            internal_sender: self.internal_sender.clone(),
        }
    }
}

pub struct TrademeApiHandle {
    internal_sender: Sender<TrademeApiRequest>,
}

impl TrademeApiHandle {
    pub async fn add_to_queue(
        &self,
        listing_id: u64,
        return_channel: tokio::sync::oneshot::Sender<TrademeApiResponse>,
    ) {
        self.internal_sender.send()
    }
}
