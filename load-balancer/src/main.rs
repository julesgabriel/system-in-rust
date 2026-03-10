mod models;

use models::LoadBalancer;
use std::sync::mpsc;
use std::thread;

fn main() {
    channel()
}

/**
Using this method the requests are scoped in a channel, so it is cleaner but not very scalable, nor efficient
*/
fn channel(){
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
        let server = load_balancer.route_request();
        println!("Requête n°{} traitée par : {}", i_recovered, server.name);
    }
}