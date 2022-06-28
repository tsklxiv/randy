use warp::Filter;
use rand::Rng;
use chrono::prelude::*;
use nanoid::nanoid;
use owoify::OwOifiable;

// Constants
const PORT: u16 = 8000;
const GET_IP_URL: &'static str = "https://httpbin.org/ip";

// Generate random numbers from <min> to <max> using `rand` crate
fn random(min: u16, max: u16) -> String {
    println!("Generating random number from {} to {}", min, max);
    format!("{}", rand::thread_rng().gen_range(min..max))
}

// Return current time, either in UTC or local time
fn right_now(mode: String) -> String {
    println!("Displaying current time. Mode: {}", mode);
    if mode == "utc" {
        format!("{}", Utc::now())
    } else if mode == "local" {
        format!("{}", Local::now())
    } else {
        println!("Unknown mode requested: {}", mode);
        "Unknown mode. Mode available is utc or local".to_string()
    }
}

// Return unique ID, by default length is 21
fn unique_id(length: usize) -> String {
    println!("Generating unique ID with length {}", length);
    nanoid!(length)
}

// Get IP address.
// This part took me almost an hour to get it right.
fn get_ip() -> Result<String, ureq::Error> {
    println!("Fetching IP address.");
    let json: serde_json::Value = ureq::get(GET_IP_URL)
        .call()?
        .into_json()?;
    println!("Fetching successful!");
    Ok(format!("{}", json["origin"]))
}

// Owoify some text
fn owoify_text(text: String) -> String {
    text.replace("%20", " ").replace("+", " ").owoify()
}

fn index() -> String {
    println!("The main page");
    format!("
Randy - Open-source Rust web app with a bunch of random (but probably useful) tools
====================
Tools:
    RNG: /rand/<min>/<max>
    Now: /now/<utc/local>
    Unique ID: /id/<length> (By default length is 21)
    Owoify: /owo/<text>
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
        .map(|| match get_ip() {
            Ok(returned) => returned,
            Err(_) => "Unexpected error when fetching IP.".to_string()
        });
    // GET /owo/
    let owo = warp::path!("owo" / String)
        .map(|text| owoify_text(text));
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
            .or(owo)
            .or(rand_noparam)
            .or(now_noparam)
            .or(id_noparam)
    );
    println!("Choo Choo! Listening at 0.0.0.0:{}!", PORT);

    warp::serve(routes)
        .run(([0, 0, 0, 0], PORT))
        .await;
}
