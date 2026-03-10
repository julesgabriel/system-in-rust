use crate::contracts::server_provider::ServerProvider;

#[derive(Debug)]
pub struct Server {
    pub(crate) id: usize,
    pub(crate) name: String,
}

#[derive(Debug)]
pub struct LoadBalancer {
    servers: Vec<Server>,
    current_cursor: usize,
}

impl LoadBalancer {
    pub(crate) fn new<T: ServerProvider>(provider: &T) -> Self {
        let servers = provider.get_servers();
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