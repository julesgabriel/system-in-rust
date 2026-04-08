use std::cmp::max;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;
use std::{thread, time};
use time::Duration;

fn main() {
    // rate_limiter()
    complex_rate_limiter()
}

fn rate_limiter() {
    let max_token_bucket = 10;

    let token_bucket = Arc::new(Mutex::new(10));
    let bucket_clone = Arc::clone(&token_bucket);
    let refresh_rate_in_millis = Duration::from_millis(100);
    refresh_tokens(max_token_bucket, bucket_clone, refresh_rate_in_millis);

    let interval_between_request_in_millis = 10;

    for i in 0..101 {
        let interval = Duration::from_millis(interval_between_request_in_millis);
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

fn refresh_tokens(
    max_token_bucket: i32,
    bucket_clone: Arc<Mutex<i32>>,
    refresh_rate_in_millis: Duration,
) {
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
}

struct TokenBucket {
    bucket: Arc<Mutex<i32>>,
    epoch: Mutex<Instant>,
}

fn complex_rate_limiter() {
    let users: RwLock<HashMap<String, TokenBucket>> = RwLock::new(HashMap::from([
        (
            "Jules".to_string(),
            TokenBucket {
                epoch: Mutex::new(Instant::now()),
                bucket: Arc::new(Mutex::new(10)),
            },
        ),
        (
            "Solène".to_string(),
            TokenBucket {
                epoch: Mutex::new(Instant::now()),
                bucket: Arc::new(Mutex::new(10)),
            },
        ),
    ]));


    for i in 0..101 {
        let interval = Duration::from_millis(10);
        thread::sleep(interval);

        if i == 62 {
            println!("\n🚀 [EVENT] Bob arrive sur le réseau ! Acquisition du verrou d'écriture (WRITE)...");
            // On demande l'accès exclusif à la Map
            let mut write_guard = users.write().unwrap();
            write_guard.insert(
                "Bob".to_string(),
                TokenBucket {
                    epoch: Mutex::new(Instant::now()),
                    bucket: Arc::new(Mutex::new(10)),
                },
            );
            println!("✅ ------------------------- [EVENT] Bob est enregistré. Libération du verrou.\n");
        }


        let name = if i % 2 == 0 { "Solène" } else { "Jules" };


        if let Some(user) = users.read().unwrap().get(name) {
            // On récupère le temps écoulé
            let elapsed = user.epoch.lock().unwrap().elapsed();
            let mut current_bucket = user.bucket.lock().unwrap();

            let tokens_to_add = (elapsed.as_millis() / 100) as i32;

            // LOG DE RECHARGE
            if tokens_to_add > 0 {
                let old_val = *current_bucket;
                *current_bucket = (*current_bucket + tokens_to_add).min(10);
                *user.epoch.lock().unwrap() = Instant::now();

                println!(
                    "  ⏳ [REGEN] {} : +{} jetons (attente de {}ms). Bucket: {} -> {}",
                    name,
                    tokens_to_add,
                    elapsed.as_millis(),
                    old_val,
                    *current_bucket
                );
            }

            // LOG DE DÉCISION
            if *current_bucket > 0 {
                *current_bucket -= 1;
                println!(
                    "✅ [PASS] Req #{} pour {} | Jetons restants: {}",
                    i, name, *current_bucket
                );
            } else {
                println!(
                    "⛔ [FAIL] Req #{} pour {} | Bucket vide ({}ms depuis dernier refresh)",
                    i,
                    name,
                    elapsed.as_millis()
                );
            }
        }
    }
}
