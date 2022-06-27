use warp::Filter;
use rand::Rng;
use chrono::prelude::*;
use nanoid::nanoid;

const HOST: [u16; 4] = [0, 0, 0, 0];
const PORT: u16 = 8000;

fn random(min: u16, max: u16) -> String {
    format!("{}", rand::thread_rng().gen_range(min..max))
}

fn right_now(mode: String) -> String {
    if mode == "utc" {
        format!("{}", Utc::now())
    } else if mode == "local" {
        format!("{}", Local::now())
    } else {
        "Unknown mode. Mode available is utc or local".to_string()
    }
}

fn unique_id(length: usize) -> String {
    nanoid!(length)
}

fn index() -> String {
    format!("
    Randy - Open-source, self-hosted Rust web app with a bunch of random tools
    ====================
    Tools:
        RNG: /rand/<min>/<max>
        Now: /now/<utc/local>
        Unique ID: /id/<length>
    ")
}

#[tokio::main]
async fn main() {
    // GET /rand/<min>/<max>
    let rand = warp::path!("rand" / u16 / u16)
        .map(|min, max| random(min, max));
    // GET /now/<utc/local>
    let now = warp::path!("now" / String)
        .map(|mode| right_now(mode));
    // GET /id/<length>
    let id = warp::path!("id" / usize)
        .map(|length| unique_id(length));
    // GET /
    let index = warp::path::end()
        .map(index);

    // Routing everything together
    let routes = warp::get().and(
        index
            .or(rand)
            .or(now)
            .or(id)
    );
    println!("Choo Choo! Listening at 0.0.0.0:{}!", PORT);

    warp::serve(routes)
        .run(([0, 0, 0, 0], PORT))
        .await;
}
