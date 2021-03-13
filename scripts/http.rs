#!/usr/bin/env rust-script
//! HTTP-POST server decoding BinaryEye scans
//! When HTTP request moe set to "POST application/json"
//! Deserializes into a Scan object (with hexdump raw) and prints
//!
//! "Tide" HTTP server derived from examples like
//! https://docs.rs/tide/0.16.0/tide/struct.Request.html#method.query
//! and adapted with Json decoding of Body examples
//! ```cargo
//! [dependencies]
//! tide = "0.14.0"
//! async-std = { version = "1.6.0", features = ["attributes"] }
//! serde = { version = "1.0", features = ["derive"] }
//! # Force log version due to: https://github.com/http-rs/tide/issues/787#issue-795373843
//! # And https://github.com/http-rs/tide/issues/800#issue-808698558
//! log = { version = "0.4.14", features = ["kv_unstable_std"]}
//! ```

//
use tide::prelude::*;
use tide::Request;

#[derive(Deserialize, Debug)]
#[serde(default)]
struct Scan {
    // content: Vec<u8>, // String
    raw: String,
    format: String,
    errorCorrectionLevel: String,
    timestamp: String,
}

impl Default for Scan {
    fn default() -> Self {
        Self {
            // content: [1, 2, 3].to_vec(), // String::from("NosuchPayload")
            raw: String::from("NORAWHERE"),
            format: String::from("None"),
            errorCorrectionLevel: String::from("N/A"),
            timestamp: String::from("now"),
        }
    }
}

async fn scan(mut req: Request<()>) -> tide::Result {
    let s: Scan = req.body_json().await?;
    println!("Scan {:?}", s);
    Ok(format!("Scanned: {}", s.raw).into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("Hello world");
    let mut app = tide::new();
    app.at("/scan").post(scan);
    app.listen("0.0.0.0:8081").await?;
    Ok(())
}
