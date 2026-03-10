use crate::contracts::server_provider::ServerProvider;
use crate::models::Server;

pub struct MockServerProvider;
impl ServerProvider for MockServerProvider {
    fn get_servers(&self) -> Vec<Server> {
        vec![
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
        ]
    }
}