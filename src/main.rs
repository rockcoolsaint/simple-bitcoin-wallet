use std::env;
// #[macro_use]
// extern crate dotenv;

use axum::{http::{response, StatusCode}, response::{Html, IntoResponse}, routing::get, Json, Router};
use bdk::{bitcoin::{address, Network}, blockchain::ElectrumBlockchain, database::{self, MemoryDatabase, SqliteDatabase}, electrum_client::Client, wallet::AddressIndex, SyncOptions, Wallet};
use dotenv::dotenv;
use serde::Serialize;

#[derive(Serialize)]
struct AddressResponse {
    address: String,
    index: u32,
}

struct AppError(anyhow::Error);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    println!("Hello, world!");

    let descriptor = env::var("WALLET_DESCRIPTOR")?;

    let wallet = Wallet::new(
        &descriptor, 
        None, 
        Network::Testnet, 
        SqliteDatabase::new("payrock.db")
    )?;
    // println!("Descriptor: {}", descriptor);
    // dbg!(descriptor);
    // let blockchain = ElectrumBlockchain::from(Client::new("ssl://electrum.blockstream.info:60002")?);

    // dbg!(wallet)?;
    // wallet.sync(&blockchain, SyncOptions::default())?;
    
    // Axum starts

    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();


    let balance = wallet.get_balance()?;
    
    dbg!(balance);
    
    let address = wallet.get_address(AddressIndex::New)?;
    
    dbg!(address);
  
    let address = wallet.get_address(AddressIndex::New)?;
    
    dbg!(address);
    
    Ok(())
}

async fn handler() -> Result<impl IntoResponse, AppError> {
    let response = AddressResponse {
        address: "test".to_string(),
        index: 0,
    };

    Ok(Json(response))
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
        .into_response()
    }
}