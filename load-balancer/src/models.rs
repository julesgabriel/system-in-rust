#[derive(Debug)]
pub struct Server {
    id: usize,
    pub(crate) name: String,
}

#[derive(Debug)]
pub struct LoadBalancer {
    servers: Vec<Server>,
    current_cursor: usize,
}

impl LoadBalancer {
    pub(crate) fn new() -> Self {
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

    pub(crate) fn route_request(&mut self) -> Option<&Server> {
        if(self.servers.is_empty()) {
            return None;
        }
        let index_to_use = self.current_cursor;
        let next_index = (self.current_cursor + 1) % self.servers.len();
        self.current_cursor = next_index;
        Some(&self.servers[index_to_use])
    }
}