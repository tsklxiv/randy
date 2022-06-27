use warp::Filter;
use rand::Rng;
use chrono::prelude::*;

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

fn index() -> String {
    format!("
    Randy - Open-source, self-hosted Rust web app with a bunch of random tools
    ====================
    Tools:
        RNG: /rand/<min>/<max>
        Now: /now/<utc/local>
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
    // GET /
    let index = warp::path::end()
        .map(index);

    // Routing everything together
    let routes = warp::get().and(
        index
            .or(rand)
            .or(now)
    );

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8000))
        .await;
}
