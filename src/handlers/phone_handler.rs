use actix_web::{web, HttpResponse};

use crate::{
    models::phone::{PhoneForSearch, PhoneFullSpec},
    modules::phone_response::ChangePhoneResponse,
    state::AppState,
};

pub struct PhoneHandler;

impl PhoneHandler {
    pub async fn get_phones(
        app_state: web::Data<AppState>,
        payload: web::Path<usize>,
    ) -> HttpResponse {
        let pid: usize = payload.into_inner();

        let result: sqlx::Result<PhoneFullSpec> =
            sqlx::query_file_as!(PhoneFullSpec, "src/queries/get_phone_by_id.sql", pid as i64,)
                .fetch_one(&app_state.pool)
                .await;

        match result {
            Ok(phone) => {
                let response = ChangePhoneResponse::change_phone_response(phone);
                HttpResponse::Ok().json(response)
            }
            Err(_) => HttpResponse::NotFound().into(),
        }
    }

    pub async fn get_all_phones(app_state: web::Data<AppState>) -> HttpResponse {
        let result = sqlx::query_file_as!(PhoneForSearch, "src/queries/get_phones.sql")
            .fetch_all(&app_state.pool)
            .await;

        match result {
            Ok(phones) => HttpResponse::Ok().json(phones),
            Err(_) => HttpResponse::BadRequest().into(),
        }
    }
}
