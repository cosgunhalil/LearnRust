mod guess_the_number_game;
mod placeholder_http_requester;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    guess_the_number_game::play();
    placeholder_http_requester::fetch_placeholder_json().await.expect("Fetching operation is failed!");
}
