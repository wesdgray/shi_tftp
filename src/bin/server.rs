use log::{debug, error, info, warn};
use shi_tftp::{protocol::Mode, server::Server};

fn main() {
    let _ = env_logger::builder()
        .format_file(true)
        .format_line_number(true)
        .try_init();

    info!("Starting Server");
    info!("Current directory: {:?}", std::env::current_dir());

    let mut server = Server::new(
        "0.0.0.0:0".parse().unwrap(),
        ".".parse().unwrap(),
        Mode::Octet
    ).unwrap(); 
    server.start();
}

