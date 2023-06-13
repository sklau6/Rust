use crate::api::{core_handler, order_handler, product_handler, stock_handler};
use actix_web::{web, App, HttpServer, HttpRequest, Responder, dev::Server};
use futures::executor;
use log::warn;
use serde_derive::Deserialize;
use std::{fs::File, io::{Read, ErrorKind}, net::{IpAddr, Ipv4Addr, SocketAddr}, sync::mpsc, thread};

// Define structure for server configuration
#[derive(Deserialize)]
pub struct ServerConfig {
    pub address: SocketAddr,
}

impl ServerConfig {
    // Define a getter method for retrieving the server's address
    fn address(&self) -> &SocketAddr {
        &self.address
    }
}

// Define structure for web service
pub struct WebService {
    config: ServerConfig,
}

impl WebService {
    // Constructor for initializing a new instance of WebService
    pub fn new() -> WebService {
        let config = WebService::get_config();
        WebService { config }
    }

    // Define a getter method for retrieving the server's config
    fn config(&self) -> &ServerConfig {
        &self.config
    }

    // Define an asynchronous method for checking the server's health
    async fn healthcheck(req: HttpRequest) -> impl Responder {
        let name = req.match_info().get("name").unwrap_or("Monolith");
        format!("Hello {}!", &name)
    }

    // Define method for fetching the server's config
    pub fn get_config() -> ServerConfig {
        let config = File::open("settings.toml")
            .and_then(|mut file| {
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)?;
                Ok(buffer)
            })
            .and_then(|buffer| {
                toml::from_str::<ServerConfig>(&buffer)
                    .map_err(|err| std::io::Error::new(ErrorKind::Other, err))
            })
            .unwrap_or_else(|err| {
                warn!("Can't read config file: {}", err);
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
            });
        config
    }

    // Define method for starting the web server
    pub async fn start_webserver(&mut self) -> std::io::Result<()> {
        let (sender, receiver) = mpsc::channel::<()>();
        let server = HttpServer::new(move || {
            App::new()
                .data(sender.clone())
                .route("/", web::get().to(WebService::healthcheck))
                .route("/stop", web::get().to(core_handler::stop))
                .configure(WebService::config_routes)
        })
        .bind(self.config().address())?
        .run();
        
        let srv = server.clone();
        WebService::setup_gracefulstop(srv, receiver);
        server.await
    }

    // Configure API endpoints and their corresponding handlers
    fn config_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(order_handler::order_list)
            .service(
                web::resource("/order/create")
                    .route(web::post().to(order_handler::order_create)),
            )
            .service(
                web::resource("/order/cancel")
                    .route(web::post().to(order_handler::order_cancel)),
            )
            .service(
                web::resource("/order/update")
                    .route(web::post().to(order_handler::order_update)),
            )
            .service(
                web::resource("/order/fulfill")
                    .route(web::post().to(order_handler::fulfill_order)),
            )
            .route(
                "/product/list",
                web::get().to(product_handler::product_list),
            )
            .service(
                web::resource("/product/create")
                    .route(web::post().to(product_handler::product_create)),
            )
            .service(
                web::resource("/product/delete")
                    .route(web::post().to(product_handler::product_delete)),
            )
            .service(
                web::resource("/product/update")
                    .route(web::post().to(product_handler::product_update)),
            )
            .route("/stock/list", web::get().to(stock_handler::stock_list))
            .service(
                web::resource("/stock/create")
                    .route(web::post().to(stock_handler::stock_create)),
            )
            .service(
                web::resource("/stock/delete")
                    .route(web::post().to(stock_handler::stock_delete)),
            )
            .service(
                web::resource("/stock/update")
                    .route(web::post().to(stock_handler::stock_update)),
            )
            .service(
                web::resource("/stock/increment")
                    .route(web::post().to(stock_handler::stock_increment)),
            );
    }

    // This method sets up a graceful shutdown for the server
    // It starts a new thread that waits for a signal to stop the server
    // When the signal is received, the server is stopped
    pub fn setup_gracefulstop(srv: Server, receiver: mpsc::Receiver<()>) {
        thread::spawn(move || {
            receiver.recv().unwrap();
            executor::block_on(srv.stop(true))
        });
    }
}
