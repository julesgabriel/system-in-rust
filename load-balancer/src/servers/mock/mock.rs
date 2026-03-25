use crate::contracts::server_provider::ServerProvider;
use crate::models::Server;
use rand::Rng;

pub struct MockServerProvider;
impl ServerProvider for MockServerProvider {
    fn get_servers(&self) -> Vec<Server> {
        vec![
            Server {
                id: 0,
                name: String::from("Serveur 0"),
                is_healthy: true
            },
            Server {
                id: 1,
                name: String::from("Serveur 1"),
                is_healthy: true
            },
            Server {
                id: 2,
                name: String::from("Serveur 2"),
                is_healthy: true
            },
        ]
    }


}
