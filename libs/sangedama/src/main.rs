use std::str::FromStr;

use tokio::select;
use tracing::{info, Level};
use crate::p2p::{
    admin_peer::{create_server, PeerAdminConfig},
    peer::{create_client, PeerConfig},
};

mod p2p;
#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).unwrap();
    info!("Starting sangedama");

    let port = 4587;
    let (server, server_peer_id) = create_server(PeerAdminConfig::new(Some(port))).await;
    let client = create_client(PeerConfig::new(Some(port), format!("/ip4/192.168.1.100/tcp/{port}").parse().unwrap(), server_peer_id)).await;

    select! {
        _ = server => {}
        _ = client => {}
    }
}