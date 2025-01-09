use log::{info, warn, error, debug, trace};

pub fn execute() {
    env_logger::init();

    trace!("This is a TRACE level message.");
    debug!("This is a DEBUG level message.");
    info!("This is an INFO level message.");
    warn!("This is a WARN level message.");
    error!("This is an ERROR level message.");

    let result = divide(10, 2);
    match result {
        Ok(value) => info!("Division result: {}", value),
        Err(e) => error!("Error occurred: {}", e),
    }
}

// Example function with logging
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        warn!("Attempt to divide by zero.");
        Err("Division by zero is not allowed.".to_string())
    } else {
        debug!("Performing division: {}/{}", a, b);
        Ok(a / b)
    }
}
