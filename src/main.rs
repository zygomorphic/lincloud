mod config_lin;
mod errors;

use crate::errors::ContextualError;
use actix_web::{guard, web, App, HttpResponse, HttpServer};
use config_lin::{CliMod, LinConfig};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use yansi::Color;

fn execute<F: FnOnce() -> LinConfig>(op: F) -> config_lin::LinConfig {
    op()
}

fn main() {
    let mode = config_lin::get_starting_mode();
    let config = match mode {
        CliMod::Client {
            interfaces,
            port,
            path,
        } => execute(|| {
            config_lin::get_cli_mode(CliMod::Client {
                interfaces,
                port,
                path,
            })
        }),
        CliMod::YmlConf { file } => execute(|| config_lin::get_yml_mode(CliMod::YmlConf { file })),
        CliMod::Default => execute(|| config_lin::get_def_mode()),
    };

    match web(config) {
        Ok(()) => (),
        Err(e) => {
            println!("{:?}", e)
        }
    }
}

#[actix_web::main(lincloud)]
async fn web(config_params: LinConfig) -> Result<(), ContextualError> {
    let interfaces = config_params
        .interfaces
        .iter()
        .map(|&interface| {
            if interface == IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)) {
                // If the interface is 0.0.0.0, we'll change it to 127.0.0.1 so that clicking the link will
                // also work on Windows. Why can't Windows interpret 0.0.0.0?
                "127.0.0.1".to_string()
            } else if interface.is_ipv6() {
                // If the interface is IPv6 then we'll print it with brackets so that it is clickable.
                format!("[{}]", interface)
            } else {
                format!("{}", interface)
            }
        })
        .collect::<Vec<String>>();

    let mut address_str = String::new();
    for interface in &interfaces {
        if !address_str.is_empty() {
            address_str.push_str(", ");
        }
        address_str.push_str(&format!(
            "{}",
            Color::Green
                .paint(format!(
                    "http://{interface}:{port}",
                    interface = &interface,
                    port = config_params.port
                ))
                .bold()
        ));
    }

    let socket_addrs = interfaces
        .iter()
        .map(|interface| {
            format!(
                "{interface}:{port}",
                interface = &interface,
                port = config_params.port,
            )
            .parse::<SocketAddr>()
        })
        .collect::<Result<Vec<SocketAddr>, _>>();

    let socket_addrs = match socket_addrs {
        Ok(address) => address,
        Err(e) => {
            // Note that this should never fail, since CLI parsing succeeded
            // This means the format of each IP address is valid, and so is the port
            // Valid IpAddr + valid port == valid SocketAddr
            return Err(ContextualError::ParseError(
                "string as socket address".to_string(),
                e.to_string(),
            ));
        }
    };

    let serv_path = config_params.path.canonicalize().map_err(|e| {
        ContextualError::IOError("Failed to resolve path to be served".to_string(), e)
    })?;

    let serv_path_string = serv_path.to_string_lossy();

    let app = HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "www.rust-lang.org"))
                    .route("", web::to(|| HttpResponse::Ok().body("www"))),
            )
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "users.rust-lang.org"))
                    .route("", web::to(|| HttpResponse::Ok().body("user"))),
            )
            .route("/", web::to(|| HttpResponse::Ok().body("/")))
    })
    .bind(socket_addrs.as_slice())
    .map_err(|e| ContextualError::IOError("Failed to bind server".to_string(), e))?
    .shutdown_timeout(3)
    .run();

    println!(
        "Serving file path {serv_path} at {address_str}",
        serv_path = Color::Yellow.paint(serv_path_string).bold(),
        address_str = address_str,
    );
    println!(
        "Server is running...\nQuit by pressing {quit}",
        quit = Color::Red.paint("CTRL-C").bold()
    );

    app.await
        .map_err(|e| ContextualError::IOError("".to_owned(), e))
}
