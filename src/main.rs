use warp::Filter;
use rand::Rng;
use chrono::prelude::*;
use nanoid::nanoid;
use ureq::Error;
use log::{info, trace, warn};

// Constants
const PORT: u16 = 8000;
const GET_IP_URL: &'static str = "https://httpbin.org/ip";

// Generate random numbers from <min> to <max> using `rand` crate
fn random(min: u16, max: u16) -> String {
    trace!("Generating random number from {} to {}", min, max);
    format!("{}", rand::thread_rng().gen_range(min..max))
}

fn right_now(mode: String) -> String {
    trace!("Displaying current time. Mode: {}", mode);
    if mode == "utc" {
        format!("{}", Utc::now())
    } else if mode == "local" {
        format!("{}", Local::now())
    } else {
        info!("Unknown mode requested: {}", mode);
        "Unknown mode. Mode available is utc or local".to_string()
    }
}

// Return unique ID, by default length is 21
fn unique_id(length: usize) -> String {
    trace!("Generating unique ID with length {}", length);
    nanoid!(length)
}

fn get_ip() -> String {
    // Handling different cases where fetching the URL can failed
    trace!("Fetching IP address.");
    match ureq::get(GET_IP_URL).call() {
        Ok(response) => {
            info!("Fetching successful!");
            let json: serde_json::Value = response.into_json().unwrap();
            format!("{}", json["origin"])
        }
        Err(Error::Status(code, _response)) => {
            warn!("Error when scraping data. Response code: {}", code)
            format!("Error when scraping data from httpbin.org: Response code {}", code)
        }
        Err(_) => {
            warn!("Unexpected error. May related to IO/transport.");
            format!("Unexpected error.")
        }
    };
    format!("Coming soon!")
}

fn index() -> String {
    trace!("The main page");
    format!("
Randy - Open-source, self-hosted Rust web app with a bunch of random tools
====================
Tools:
    RNG: /rand/<min>/<max>
    Now: /now/<utc/local>
    Unique ID: /id/<length> (By default length is 21)
    IP: /ip/
    ")
}

#[tokio::main]
async fn main() {
    // GET /rand/<min>/<max>
    let rand = warp::path!("rand" / u16 / u16)
        .map(|min, max| random(min, max));
    // GET /rand/
    let rand_noparam = warp::path("rand")
        .map(|| format!("Usage: /rand/<min>/<max>"));
    // GET /now/<utc/local>
    let now = warp::path!("now" / String)
        .map(|mode| right_now(mode));
    // GET /now/
    let now_noparam = warp::path!("now")
        .map(|| format!("Usage: /now/<utc/local>"));
    // GET /id/<length>
    let id = warp::path!("id" / usize)
        .map(|length| unique_id(length));
    // GET /id/
    let id_noparam = warp::path!("id")
        .map(|| unique_id(21));
    // GET /ip/
    let ip = warp::path!("ip")
        .map(get_ip);
    // GET /
    let index = warp::path::end()
        .map(index);

    // Routing everything together
    let routes = warp::get().and(
        index
            .or(rand)
            .or(now)
            .or(id)
            .or(ip)
            .or(rand_noparam)
            .or(now_noparam)
            .or(id_noparam)
    );
    println!("Choo Choo! Listening at 0.0.0.0:{}!", PORT);

    warp::serve(routes)
        .run(([0, 0, 0, 0], PORT))
        .await;
}
