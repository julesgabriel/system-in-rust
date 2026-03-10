
use crate::models::Server;

pub trait ServerProvider {
    fn get_servers(&self) -> Vec<Server>;
}