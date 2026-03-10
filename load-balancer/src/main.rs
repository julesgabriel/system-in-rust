mod models;

use crate::models::Server;
use models::LoadBalancer;
use std::rc::Rc;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn main() {
    channel();
    mutex();
}

fn mutex() {
    let load_balancer = Arc::new(Mutex::new(LoadBalancer::new()));

    for i in 0..20 {
        let load_balancer = Arc::clone(&load_balancer);
        thread::spawn(move || {
            let mut locked_load_balancer = load_balancer.lock().unwrap();
            println!("MUTEX");
            match locked_load_balancer.route_request() {
                None => println!("Alerte : Aucun serveur disponible pour la requête n°{i}"),
                Some(server) =>  println!("Requête n°{i} traitée par : {}", server.name)
            }
        });
    }
}

/**
Using this method the requests are scoped in a channel, so it is cleaner but not very scalable, nor efficient
*/
fn channel() {
    let mut load_balancer = LoadBalancer::new();
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
