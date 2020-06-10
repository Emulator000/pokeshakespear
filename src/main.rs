mod config;
mod logger;
mod server;
#[cfg(test)]
mod test;

use crate::config::Config;
use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new("config/config.toml");
    let address = format!("{}:{}", config.server.bind_address, config.server.bind_port);

    logger::log(format!("Server up and running on {}", address));

    HttpServer::new(move || App::new().data(config.clone()).service(server::get_pokemon))
        .bind(address)?
        .run()
        .await
}
