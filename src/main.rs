use inquire::Select;

mod guess_the_number_game;
mod placeholder_http_requester;
mod decimal_to_binary_and_hexadecimal_sample;

#[tokio::main]
async fn main() {
    // Define the options
    let options = [
        "Play Guess the Number Game",
        "Fetch Placeholder JSON",
        "Convert Decimal to Binary and Hexadecimal",
    ];

    // Prompt the user to select an option
    match Select::new("Select an option:", Vec::from(&options)).prompt() {
        Ok(choice) => match choice {
            "Play Guess the Number Game" => guess_the_number_game::play(),
            "Fetch Placeholder JSON" => {
                if let Err(e) = placeholder_http_requester::fetch_placeholder_json().await {
                    eprintln!("Error fetching placeholder JSON: {}", e);
                }
            }
            "Convert Decimal to Binary and Hexadecimal" => {
                decimal_to_binary_and_hexadecimal_sample::execute()
            }
            _ => unreachable!("Unexpected choice"),
        },
        Err(_) => println!("There was an error, please try again."),
    }
}
