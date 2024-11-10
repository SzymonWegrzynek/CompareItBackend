use actix_web::web;

use crate::handlers::auth_handler::JwtToken;
use crate::handlers::image_handler::ImageHandler;
use crate::handlers::phone_handler::PhoneHandler;

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

pub fn jwt_token(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/v1/encode-token/{id}",
        web::get().to(JwtToken::encode_token),
    );
    cfg.route("/v1/decode-token", web::post().to(JwtToken::decode_token));
    cfg.route("/v1/protected", web::get().to(JwtToken::protected));
}
