use warp::Filter;
use rand::Rng;

fn random(min: u16, max: u16) -> String {
    format!("{}", rand::thread_rng().gen_range(min..max))
}

fn index() -> String {
    format!("Services:\nRNG: /rand/<min>/<max>")
}

#[tokio::main]
async fn main() {
    // GET /rand/<min>/<max>
    let rand = warp::path!("rand" / u16 / u16)
        .map(|min, max| random(min, max));
    let index = warp::path!("/")
        .map(index);

    warp::serve(index)
        .run(([0, 0, 0, 0], 8000))
        .await;
}
