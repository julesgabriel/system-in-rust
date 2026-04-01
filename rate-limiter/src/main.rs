use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    rate_limiter()
}

/**
 * I need to:
- Have a certain amount of request by ms (a number of calls, it can be a loop or whatever)
- Each time a request pass in the system we must decrement one to the token amount
- Every 1s a token is incremented in the token bucket (if the maximum quantity of token is not reached)
- We must log everywhere the things
 **/
fn rate_limiter() {
    let max_token_bucket = 10;
    let token_bucket = Arc::new(Mutex::new(10));
    let bucket_clone = Arc::clone(&token_bucket);
    let refresh_rate_in_millis = time::Duration::from_millis(100);
    thread::spawn(move || {
        loop {
            thread::sleep(refresh_rate_in_millis);
            let mut bucket = bucket_clone.lock().unwrap();
            if *bucket < max_token_bucket {
                *bucket += 1;
                println!("Jeton ajouté Total: {}", *bucket)
            }
        }
    });

    let interval_between_request_in_millis = 10;

    for i in 0..101 {
        let interval = time::Duration::from_millis(interval_between_request_in_millis);
        thread::sleep(interval);
        let mut original_bucket_locked_reference = token_bucket.lock().unwrap();
        if *original_bucket_locked_reference > 0 {
            println!("✅ The request {} has passed", i);
            *original_bucket_locked_reference -= 1
        } else {
            println!("⛔️ The request {} did not passed", i)
        }
    }
}
