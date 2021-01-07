use std::{env, process};

mod tcp_client;
mod tcp_server;
mod udp_client;
mod udp_server;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args = env::args().collect::<Vec<String>>();
    if args.len() != 4 {
        log::error!("Please specify [tcp|udp] [server|client] [addr:port].");
        process::exit(1);
    }
    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address: &str = &args[3];

    match protocol {
        "tcp" => match role {
            "server" => {
                tcp_server::serve(address).unwrap_or_else(|e| log::error!("{}", e));
            }
            "client" => {
                tcp_client::connect(address).unwrap_or_else(|e| log::error!("{}", e));
            }
            _ => {
                missing_role();
            }
        },
        "udp" => match role {
            "server" => {}
            "client" => {}
            _ => {
                missing_role();
            }
        },
        _ => {
            log::error!("Please specify tcp or udp on the 1st argument.");
            process::exit(1);
        }
    }
}

fn missing_role() {
    log::error!("Please specify server or client on the 2nd argument.");
    process::exit(1);
}
