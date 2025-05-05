use std::sync::Arc;
use std::sync::Mutex;

use rust::server::control_server;
use rust::server::control_server::TCPServerTrait;
use rust::server::server::MSDPServer;
use rust::structs::entry::Entry;

fn main() {
    let multicast_group = [226, 0, 10, 70];
    let port = 10000;

    let entries: Arc<Mutex<Vec<Entry>>> = Arc::new(Mutex::new(Vec::new()));

    let mut server = MSDPServer::new(multicast_group, port, 5, Arc::clone(&entries)).unwrap();

    println!(
        "MSDP server started on {}:{}",
        multicast_group
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("."),
        port
    );

    let control_server =
        control_server::ControlServer::new("127.0.0.1:10001".to_string(), Arc::clone(&entries))
            .unwrap();

    std::thread::spawn(move || {
        control_server
            .handle_connections()
            .expect("Failed to handle connections");
    });

    let raw_proxy_server = 
        control_server::ControlServer::new("0.0.0.0:10002".to_string(), Arc::clone(&entries))
            .unwrap();


    std::thread::spawn(move || {
        raw_proxy_server
            .handle_connections()
            .expect("Failed to handle connections");
    });

    server.run().expect("Failed to run server");
}
