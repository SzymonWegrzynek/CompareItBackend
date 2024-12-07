use actix_web::{web, HttpResponse};
use std::env;

use crate::models::gpt_api::AskGpt;
use crate::modules::gpt_in_use::GptInUse;

pub struct GptApi;

impl GptApi {
    pub async fn send_message(form: web::Json<AskGpt>) -> HttpResponse {
        let api_key = match env::var("OPENAI_API_KEY") {
            Ok(key) => key,
            Err(_) => {
                return HttpResponse::InternalServerError().body("Error related to the api key")
            }
        };

        let gpt = match GptInUse::new(api_key) {
            Ok(client) => client,
            Err(_) => {
                return HttpResponse::InternalServerError()
                    .body("Error during initialization client")
            }
        };

        match gpt.ask(&form.question).await {
            Ok(response) => HttpResponse::Ok().body(response),
            Err(_) => HttpResponse::BadRequest().body("Error during communication with AI model"),
        }
    }
}
