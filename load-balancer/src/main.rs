mod models;
use models::LoadBalancer;

fn main() {
    let mut load_balancer = LoadBalancer::new();
    println!("Here is my loadBalancer: {:?}", load_balancer);
    for i in 0..20 {
        let route_requested = load_balancer.route_request();
        println!(
            "The server targetted for {} the servers which name is {:?}",
            i, route_requested
        );
        println!("------------------------")
    }
}
