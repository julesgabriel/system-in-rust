#[derive(Debug)]
struct Server {
    id: usize,
    name: String,
}

#[derive(Debug)]
struct LoadBalancer {
    servers: Vec<Server>,
    current_cursor: usize,
}

impl LoadBalancer {
    fn new() -> Self {
        let servers: Vec<Server> = vec![
            Server {
                id: 0,
                name: String::from("Serveur 0"),
            },
            Server {
                id: 1,
                name: String::from("Serveur 1"),
            },
            Server {
                id: 2,
                name: String::from("Serveur 2"),
            },
        ];
        LoadBalancer {
            servers,
            current_cursor: 0,
        }
    }

    fn route_request(&mut self) -> usize {
        let index_to_use = self.current_cursor;
        let next_index = (self.current_cursor + 1) % self.servers.len();
        self.current_cursor = next_index;
        index_to_use
    }
}

fn main() {
    let mut load_balancer = LoadBalancer::new();
    println!("Here is my loadBalancer: {:?}", load_balancer);
    for i in 0..20 {
        println!(
            "Here is my {} route_request: {:?}",
            i,
            load_balancer.route_request()
        );
    }
}
