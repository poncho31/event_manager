use std::io::Result;
use std::io::Write;
use std::{process::Command, process::exit, time::Duration, process};
use std::sync::mpsc;
use std::thread;
use std::path::Path;
use std::str;

use actix_web::{web, App, HttpServer, middleware};

use log::{info, warn};
use tera::Tera;
use crate::{database, utils};
use crate::utils::command::execute;


pub async fn run(
        routes: fn(&mut web::ServiceConfig),
        resources: fn(&mut web::ServiceConfig),
    ) -> Result<()> {
    info!("Lancement en mode Serveur Web");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())

            // CONFIGURATION
            .configure(|cfg| configure_app(cfg, template_engine("tera")))

            // RESOURCES
            .configure(resources)

            // ROUTES
            .configure(routes)
    })
        .workers(1)
        .bind(utils::env::get("APP_WEB_SERVER_URL"))?
        .run();

    let address = "localhost:81";
    info!("Serveur Proxy Nginx en cours de démarrage à l'adresse : {}", address);
    start_proxy_server();

    if let Err(e) = server.await {
        eprintln!("Erreur lors de l'exécution du serveur Actix-Web: {}", e);
    }

    Ok(())
}



fn start_proxy_server() {
    let stop_service = utils::env::get("PROXY_WEB_SERVER_STOP");
    let start_service = utils::env::get("PROXY_WEB_SERVER_START");

    execute::command("Stopping Nginx service", &stop_service);
    execute::command("Starting Nginx service", &start_service);
}


fn configure_app(cfg: &mut web::ServiceConfig, tera: Tera) {
    cfg
        // Pool de connexions
        .app_data(web::Data::new(database::establish_connection_pool()))

        // Moteur de templates
        .app_data(web::Data::new(tera));
}

fn template_engine(name: &str) -> Tera {
    if name == "tera" {
        match Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/views/**/*")) {
            Ok(t) => {
                info!("Moteur template Tera initialisé avec succès.");
                t.clone()  // Retourner l'objet `Tera` en cas de succès
            }
            Err(e) => {
                warn!("Erreur lors de l'initialisation du Moteur template Tera : {:?}", e);
                exit(1);
            }
        }
    } else {
        warn!("Aucun moteur de template trouvé pour: {}", name);
        exit(1);
    }
}
