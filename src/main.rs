use inquire::Select;

mod guess_the_number_game;
mod placeholder_http_requester;
mod decimal_to_binary_and_hexadecimal_sample;
mod write_data_to_local_db_sample;
mod id_service;
mod log_example;
mod todo_api;

#[tokio::main]
async fn main() {
    let options = [
        "Play Guess the Number Game",
        "Fetch Placeholder JSON",
        "Convert Decimal to Binary and Hexadecimal",
        "Write a Sample Data to Local DB as Binary",
        "ID Service Example",
        "Simple Log Example"
    ];

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
            "Write a Sample Data to Local DB as Binary" => {
                write_data_to_local_db_sample::execute().expect("Unfortunately, we were unable to complete the writing task for db");
            },
            "ID Service Example" => {
                // Call the start_id_service function and await its completion
                if let Err(e) = id_service::start_id_service().await {
                    eprintln!("Error starting ID Service: {}", e);
                }
            },
            "Simple Log Example" => {
                log_example::execute()
            },
            _ => unreachable!("Unexpected choice"),
        },
        Err(_) => println!("There was an error, please try again."),
    }
}
