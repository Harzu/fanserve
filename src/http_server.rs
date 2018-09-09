#![allow(unused_variables)]
use std::fs::File;
use actix_web::{fs, middleware, server, App,};
use types::ServerConfig;
use actix;

fn get_index(path: &str) -> bool {
  match File::open(format!("{}/index.html", path)) {
    Ok(file) => true,
    Err(err) => false
  }
}

pub fn create_server (config: ServerConfig) {
  let sys = actix::System::new("fanserve");
  let protocol = config.protocol.clone();
  let static_path = config.path_static.clone();
  let server_address = format!(
    "{host}:{port}",
    port = &config.port,
    host = &config.host
  );

  server::new(move || {
      App::new()
        .middleware(middleware::Logger::default())
        .handler(
          r"/",
          if get_index(&static_path) {
            fs::StaticFiles::new(&static_path).unwrap()
              .index_file("index.html")
          } else {
            fs::StaticFiles::new(&static_path).unwrap()
              .show_files_listing()
          }
        )
  })
  .bind(&server_address).unwrap()
  .start();

  println!("Server start for {}://{}", protocol, server_address);
  let _ = sys.run();
}