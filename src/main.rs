use actix_web::{web, App, HttpResponse, HttpServer, get};
use serde::Deserialize;
use serde_json;
use std::{fs, collections::HashMap};

#[derive(Deserialize, Clone, Debug)]
struct ConfigItem {
    name: String,
    status_code: u16,
    url: String,
}

#[get("/{path}")]
async fn check(path: web::Path<String>, data: web::Data<HashMap<String, ConfigItem>>) -> HttpResponse {
    let site = path.into_inner();

    let client_result = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build();

    match client_result {
        Ok(client) => {
            let site_config = data.get(&site as &str);
            match site_config {
                Some(c) => check_status(client, c.status_code, &c.url).await,
                None => HttpResponse::InternalServerError().body("Not configured!"),
            }
        }
        Err(e) => {
            println!("Error creating client: {:#?}", e);
            HttpResponse::InternalServerError().body("Error creating client!")
        }
    }

}

async fn check_status(client: reqwest::Client, expected_status: u16, url: &str) -> HttpResponse {
    let resp = client.get(url).send().await;
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
    let json_config = fs::read_to_string("config/config.json")?;
    let config: Vec<ConfigItem> = serde_json::from_str(&json_config)?;

    let mut hashed_items: HashMap<String, ConfigItem> = HashMap::new();

    for item in config.iter() {
        hashed_items.insert(item.name.clone(), item.clone());
    }

    let host = "0.0.0.0";
    let port = 8080;


    println!("Starting server on {}:{}", host, port);
    println!("{} sites configured", config.len());
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(hashed_items.clone()))
            .service(check)
    })
    .bind((host, port))?
    .run()
    .await
}
