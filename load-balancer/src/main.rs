mod models;

use std::ops::Deref;
use models::{LoadBalancer};


fn main() {
    let mut load_balancer = LoadBalancer::new();
    println!("Here is my loadBalancer: {:?}", load_balancer);
    for i in 0..20 {
        let route_requested = load_balancer.route_request();
        println!(
            "Here is my {} route_request: {:?}",
            i,
            route_requested
        );
        let server_requested = &load_balancer.servers[route_requested];
        println!("It thus targets {:?}", server_requested.name);
        println!("------------------------")
    }
}
