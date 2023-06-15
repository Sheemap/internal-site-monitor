use actix_web::{web::{self, Data}, App, HttpResponse, HttpServer, get, HttpRequest};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone, Debug)]
struct Config {
    jackett_url: String,
    jackett_code: u16,
    kopia_url: String,
    kopia_code: u16,
}

#[get("/jackett")]
async fn jackett(req: HttpRequest) -> HttpResponse {
    let config = req.app_data::<Data<Config>>();

    match config {
        Some(config) => {
            check_status(config.jackett_code, &config.jackett_url).await
        }
        None => HttpResponse::InternalServerError().body("No config!"),
    }

}

#[get("/kopia")]
async fn kopia(req: HttpRequest) -> HttpResponse {
    let config = req.app_data::<Data<Config>>();

    match config {
        Some(config) => {
            check_status(config.kopia_code, &config.kopia_url).await
        }
        None => HttpResponse::InternalServerError().body("No config!"),
    }

}


async fn check_status(expected_status: u16, url: &String) -> HttpResponse {
    let resp = reqwest::get(url).await;
    match resp {
        Ok(resp) => {
            if resp.status().as_u16() == expected_status {
                HttpResponse::Ok().body("Up!")
            } else {
                HttpResponse::InternalServerError().body("Down!")
            }
        }
        Err(e) => match e.status() {
            Some(status) => {
                if status.as_u16() == expected_status {
                    HttpResponse::Ok().body("Up!")
                } else {
                    HttpResponse::InternalServerError().body("Down!")
                }
            }
            None => {
                println!("No status code!");
                println!("{:#?}", e);
                HttpResponse::InternalServerError().body("Down!")
            }
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let toml_config = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&toml_config).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .service(jackett)
            .service(kopia)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
