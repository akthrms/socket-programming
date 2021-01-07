use std::str::FromStr;
use std::{env, process};

mod tcp_client;
mod tcp_server;
mod udp_client;
mod udp_server;

enum Protocol {
    Tcp,
    Udp,
}

impl FromStr for Protocol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tcp" => Ok(Self::Tcp),
            "udp" => Ok(Self::Udp),
            _ => Err("Please specify tcp or udp on the 1st argument.".to_string()),
        }
    }
}

enum Role {
    Server,
    Client,
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "server" => Ok(Self::Server),
            "client" => Ok(Self::Client),
            _ => Err("Please specify server or client on the 2nd argument.".to_string()),
        }
    }
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args = env::args().collect::<Vec<String>>();
    if args.len() != 4 {
        log::error!("Please specify [tcp|udp] [server|client] [addr:port].");
        process::exit(1);
    }

    let protocol = args[1].parse::<Protocol>().unwrap_or_else(|e| {
        log::error!("{}", &e);
        process::exit(1);
    });
    let role = args[2].parse::<Role>().unwrap_or_else(|e| {
        log::error!("{}", &e);
        process::exit(1);
    });
    let address: &str = &args[3];

    match (protocol, role) {
        (Protocol::Tcp, Role::Server) => {
            tcp_server::serve(address).unwrap_or_else(|e| log::error!("{}", e));
        }
        (Protocol::Tcp, Role::Client) => {
            tcp_client::connect(address).unwrap_or_else(|e| log::error!("{}", e));
        }
        (Protocol::Udp, Role::Server) => {
            udp_server::serve(address).unwrap_or_else(|e| log::error!("{}", e));
        }
        (Protocol::Udp, Role::Client) => {
            udp_client::communicate(address).unwrap_or_else(|e| log::error!("{}", e));
        }
    }
}
