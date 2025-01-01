use std::io::{self, Write};

pub fn execute() {
    print!("Enter a decimal number: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    match input.parse::<u32>() {
        Ok(decimal_number) => {
            // Convert to hexadecimal and binary
            let hex_string = format!("{:X}", decimal_number);
            let binary_string = format!("{:b}", decimal_number);

            // Display the results
            println!("Decimal: {}", decimal_number);
            println!("Hexadecimal: 0x{}", hex_string);
            println!("Binary: 0b{}", binary_string);
        }
        Err(_) => {
            // Handle invalid input
            println!("'{}' is not a valid decimal number.", input);
        }
    }
}
