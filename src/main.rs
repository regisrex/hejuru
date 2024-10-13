mod github;
use actix_web::{web, App, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse<T> {
    success: bool,
    message: String,
    data: T,
}


#[actix_web::post("/webhook")]
async fn webhook(form: actix_web::web::Json<github::PullRequestEvent>) -> Result<web::Json<ApiResponse<String>>, actix_web::Error>  {
println!("{:?}",form);

   Ok(web::Json(
       ApiResponse {
            success: true,
            message: String::from("Domain checked"),
            data: "pong".to_string()
        }
    ))
}

#[actix_web::get("/webhook")]
async fn ping() -> Result<web::Json<ApiResponse<String>>, actix_web::Error>  {
   Ok(web::Json(
       ApiResponse {
            success: true,
            message: String::from("Pong"),
            data: "pong".to_string()
        }
    ))
}


/**
 *   Server entry
 *   @author : NDIZIHIWE Regis
 */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/hejuru")
                .service(webhook)
                .service(ping)
        )
    })
    .bind(("0.0.0.0", 8765))?
    .run()
    .await
}
