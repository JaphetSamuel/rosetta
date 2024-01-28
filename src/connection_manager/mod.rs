use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, RwLock};
use tokio::task;


pub struct ConnectionInfo {
    pub address: SocketAddr,
}


pub struct ConnectionManager {
    listener: TcpListener,
    connections: RwLock<Vec<ConnectionInfo>>,

}

impl ConnectionManager {
    pub async fn new(bind_address: &str) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(bind_address).await?;
        let connections = RwLock::new(Vec::new());

        Ok(ConnectionManager { listener, connections })
    }


    pub async fn start(&self) {
        loop {
            if let Ok((socket, addr)) = self.listener.accept().await {
                print!("stack")
                // task::spawn(Self::handle_connection(socket, addr, self.connections.clone()));
            }
        }
    }

    async fn handle_connection(socket: TcpStream, address: SocketAddr, connections: RwLock<Vec<ConnectionInfo>>) {
        let mut connections = connections.write().await;
        connections.push(ConnectionInfo { address });
    }


    pub async fn close_connection(&self, address: SocketAddr) {
        let mut connections = self.connections.write().await;
        connections.retain(|info| info.address != address);
    }
}