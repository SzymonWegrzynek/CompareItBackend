use actix_web::web;

use crate::handlers::{
    gpt_handler::GptApi, healthcheck_handler::HealthCheck, image_handler::ImageHandler,
    login_handler::LoginHandler, phone_handler::PhoneHandler, token_handler::TokenHandler,
};

pub fn healthcheck(cfg: &mut web::ServiceConfig) {
    cfg.route("/healthcheck", web::get().to(HealthCheck::healthcheck));
}

pub fn phone(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/v1/phones/{phone_id}",
        web::get().to(PhoneHandler::get_phones),
    );

    cfg.route("/v1/phones", web::get().to(PhoneHandler::get_all_phones));
}

pub fn image(cfg: &mut web::ServiceConfig) {
    cfg.route("/v1/images", web::post().to(ImageHandler::insert_image));
}

pub fn user(cfg: &mut web::ServiceConfig) {
    cfg.route("/v1/users", web::post().to(LoginHandler::create_user));
}

pub fn chat(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/v1/chat/conversation",
        web::post().to(GptApi::send_message),
    );
}

pub fn auth(cfg: &mut web::ServiceConfig) {
    cfg.route("/v1/auth/signin", web::post().to(LoginHandler::signin_user));

    cfg.route(
        "/v1/auth/extract",
        web::get().to(TokenHandler::extract_token),
    );

    cfg.route("/v1/auth/verify", web::get().to(TokenHandler::verify_token));
}
