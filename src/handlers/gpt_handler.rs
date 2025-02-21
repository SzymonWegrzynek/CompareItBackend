use actix_web::{web, HttpResponse};
use std::env;

use crate::{
    models::gpt::{AskGpt, GptAnswer},
    modules::gpt::Gpt,
};

pub struct GptApi;

impl GptApi {
    pub async fn send_message(payload: web::Json<AskGpt>) -> HttpResponse {
        let api_key = match env::var("OPENAI_API_KEY") {
            Ok(key) => key,
            Err(_) => return HttpResponse::InternalServerError().body("Error with api key"),
        };

        let gpt = match Gpt::new(api_key) {
            Ok(client) => client,
            Err(_) => {
                return HttpResponse::InternalServerError()
                    .body("Error during initialization client")
            }
        };

        let ask_gpt = gpt.ask(&payload.question).await;

        match ask_gpt {
            Ok(response) => {
                let gpt_response = GptAnswer { answer: response };
                HttpResponse::Ok().json(gpt_response)
            }
            Err(_) => HttpResponse::BadRequest().body("Error during communication with AI model"),
        }
    }
}
