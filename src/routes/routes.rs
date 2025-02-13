use actix_web::web;

use crate::handlers::{
    gpt_handler::GptApi, image_handler::ImageHandler, phone_handler::PhoneHandler,
    token_handler::TokenHandler, user_handler::UserHandler,
};

pub fn healthcheck(cfg: &mut web::ServiceConfig) {
    cfg.route("/healthcheck", web::get().to(PhoneHandler::healthcheck));
}

pub fn phone(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/v1/get-phone/{phone_id}",
        web::get().to(PhoneHandler::get_phone),
    );
    cfg.route(
        "/v1/get-all-phones",
        web::get().to(PhoneHandler::get_all_phones),
    );
}

pub fn image(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/v1/insert-image",
        web::post().to(ImageHandler::insert_image),
    );
}

pub fn user(cfg: &mut web::ServiceConfig) {
    cfg.route("/v1/create-user", web::post().to(UserHandler::create_user));
    cfg.route("/v1/signin-user", web::post().to(UserHandler::signin_user));
}

pub fn gpt(cfg: &mut web::ServiceConfig) {
    cfg.route("/v1/send-message", web::post().to(GptApi::send_message));
}

pub fn token(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/v1/verify-token",
        web::post().to(TokenHandler::verify_token),
    );
    cfg.route(
        "/v1/protect-route",
        web::post().to(TokenHandler::protect_route),
    );
}
