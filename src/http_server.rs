#![allow(unused_variables)]

use actix_web::{fs, middleware, server, App};
use types::ServerConfig;
use actix;
use env_logger;

pub fn create_server (config: ServerConfig) {
  ::std::env::set_var("RUST_LOG", "actix_web=info");
  ::std::env::set_var("RUST_BACKTRACE", "1");
  env_logger::init();

  let sys = actix::System::new("fanserve");
  let protocol = config.protocol.clone();
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
          fs::StaticFiles::new(format!("{}", &config.path_static))
            .unwrap()
            .show_files_listing()
        )
  })
  .bind(&server_address).unwrap()
  .start();

  println!("Server start for {}://{}", protocol, server_address);
  let _ = sys.run();
}