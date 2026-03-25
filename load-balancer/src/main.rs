mod models;
mod contracts;
mod servers;

use models::LoadBalancer;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;
use crate::contracts::server_provider::ServerProvider;
use crate::servers::mock::mock::MockServerProvider;

fn main() {
    let provider = MockServerProvider;
    //channel(&provider);
    mutex(&provider);
}

fn mutex<S: ServerProvider>(server_provider: &S) {
    let load_balancer = Arc::new(Mutex::new(LoadBalancer::new(server_provider)));

    let mut handles = vec![];

    for i in 0..20 {
        let load_balancer = Arc::clone(&load_balancer);
        let lb_for_health_check = Arc::clone(&load_balancer);
        thread::spawn(move || {
            loop { // Une boucle infinie pour vérifier en continu
                thread::sleep(Duration::from_secs(1));

                // On prend le verrou, on check, et on le relâche immédiatement
                // à la fin de cette itération (quand locked_lb sort de portée)
                let mut locked_lb = lb_for_health_check.lock().unwrap();
                locked_lb.check_health();
                println!("--- [Health Check] Mise à jour des serveurs effectuée ---");
            }
        });


        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(200 * i as u64));
            let mut locked_load_balancer = load_balancer.lock().unwrap();
            println!("MUTEX");
            match locked_load_balancer.route_request() {
                None => println!("Alerte : Aucun serveur disponible pour la requête n°{i}"),
                Some(server) =>  println!("Requête n°{i} traitée par : {}", server.name)
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/**
Using this method the requests are scoped in a channel, so it is cleaner but not very scalable, nor efficient
*/
fn channel<S: ServerProvider>(server_provider: &S) {
    let mut load_balancer = LoadBalancer::new(server_provider);
    let (transmitter, receiver) = mpsc::channel();
    for i in 0..20 {
        let copy_transmitter = transmitter.clone();
        thread::spawn(move || {
            copy_transmitter.send(i).expect("Erreur d'envoi");
        });
    }

    drop(transmitter);

    for i_recovered in receiver {
        match load_balancer.route_request() {
            Some(server) => println!("Requête n°{} traitée par : {}", i_recovered, server.name),
            None => println!("Alerte : Aucun serveur disponible pour la requête n°{i_recovered}"),
        }
    }
}
