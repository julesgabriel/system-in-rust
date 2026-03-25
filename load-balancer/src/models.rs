use crate::contracts::server_provider::ServerProvider;
use rand::Rng;

#[derive(Debug)]
pub struct Server {
    pub(crate) id: usize,
    pub(crate) name: String,
    pub(crate) is_healthy: bool,
}

pub trait Pingable {
    fn ping(&mut self) -> bool;
}
impl Pingable for Server {
    fn ping(&mut self) -> bool {
        let mut rng = rand::thread_rng();
        let is_healthy = rng.gen_bool(0.98);
        if !is_healthy {
            println!("❌ Le {} est down !", self.name);
        }
        is_healthy
    }
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

    pub(crate) fn check_health(&mut self) {
        self.servers
            .iter_mut()
            .for_each(|server| server.is_healthy = server.ping())
    }

    pub(crate) fn route_request(&mut self) -> Option<&Server> {
        if (self.servers.is_empty()) {
            return None;
        }

        let nb_servers = self.servers.len();

        for _ in 0..nb_servers {
            let index_to_check = self.current_cursor;
            self.current_cursor = (self.current_cursor + 1) % nb_servers;
            if self.servers[index_to_check].is_healthy {
                return Some(&self.servers[index_to_check]);
            }
        }

        None
    }
}
