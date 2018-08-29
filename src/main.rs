extern crate clap;
extern crate actix;
extern crate actix_web;
extern crate env_logger;

mod types;
mod http_server;

use clap::{ App, Arg, SubCommand };
use types::ServerConfig;
use http_server::create_server;

fn main() {    
    let fanserve = App::new("fanserve")
        .version("0.0.1")
        .about("Close it")
        .author("Ilya Shu <dev@ilyashu.com>")
        .subcommand(SubCommand::with_name("http")
            .arg(Arg::with_name("directory")
                .required(false))
            .arg(Arg::from_usage("-p, --port=[PORTNAME]")
                .required(false))
            .arg(Arg::with_name("host")
                .required(false)))
        .subcommand(SubCommand::with_name("https")
            .arg(Arg::with_name("directory")
                .required(false)))
        .subcommand(SubCommand::with_name("ws")
            .arg(Arg::with_name("<directory>")
                .required(false)))
        .get_matches();

    match fanserve.subcommand() {
        ("http", Some(_http)) => {
            let port = _http.value_of("port").unwrap_or("8888");
            let host = _http.value_of("host").unwrap_or("127.0.0.1");
            let directory = _http.value_of("directory").unwrap_or("./");

            let server_config = ServerConfig::new(host, port, "http", directory);
            
            create_server(server_config);
        },
        ("https", Some(_https)) => {
            println!("https")
        },
        ("ws", Some(_ws)) => {
            println!("WS")
        }
        ("", None) => println!("No subcommand"),
        _ => unreachable!(),
    };
}
