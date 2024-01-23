use rcon::Connection;
use std::{env, process::exit};
use tokio::net::TcpStream;
use toml::Value as TomlValue;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: palworld_rcon");
        exit(1);
    }

    let config: TomlValue = toml::from_str(include_str!("../config.toml")).unwrap();
    let server_config = config.get("server").unwrap().clone();
    let ip = server_config.get("ip").unwrap().as_str().unwrap();
    let port = server_config
        .get("port")
        .and_then(TomlValue::as_str)
        .unwrap_or("25575");
    let password = server_config.get("password").unwrap().as_str().unwrap();

    let mut connection = Connection::builder()
        .connect(&format!("{}:{}", ip, port), password)
        .await?;

    let command = &args[1];
    let params = &args[2..];

    match command.as_str() {
        "bcast" => {
            let message = params.join(" ");
            broadcast(&mut connection, &message).await;
        }
        "q" => {
            let seconds = params
                .get(0)
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(60);
            let message = params
                .get(1)
                .map(String::as_str)
                .unwrap_or("Server will be shutdown soon");
            shutdown(&mut connection, seconds, message).await;
        }
        "q!" => {
            do_exit(&mut connection).await;
        }
        "x" => {
            save(&mut connection).await;
        }
        _ => {
            eprintln!("Unknown command");
            exit(1);
        }
    }

    exit(0);
}

async fn send_command(connection: &mut Connection<TcpStream>, command: &str) {
    let _ = connection.cmd(command).await;
}

async fn shutdown(connection: &mut Connection<TcpStream>, seconds: u32, message: &str) {
    send_command(connection, &format!("Shutdown {} {}", seconds, message)).await;
}

async fn do_exit(connection: &mut Connection<TcpStream>) {
    send_command(connection, "DoExit").await;
}

async fn broadcast(connection: &mut Connection<TcpStream>, message: &str) {
    send_command(connection, &format!("Broadcast {}", message)).await;
}

async fn save(connection: &mut Connection<TcpStream>) {
    send_command(connection, "Save").await;
}
