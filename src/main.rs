use warp::Filter;
use rand::Rng;

fn random(min: u16, max: u16) -> String {
    format!("{}", rand::thread_rng().gen_range(min..max))
}

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let rand = warp::path!("rand" / u16 / u16)
        .map(|min, max| random(min, max));

    warp::serve(rand)
        .run(([0, 0, 0, 0], 8000))
        .await;
}
