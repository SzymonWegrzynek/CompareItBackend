use actix_web::{web, HttpResponse};

use crate::{
    models::phone::{PhoneForSearch, PhoneFullSpec},
    modules::phone_response::ChangePhoneResponse,
    state::AppState,
};

pub struct PhoneHandler;

impl PhoneHandler {
    pub async fn get_phone(
        app_state: web::Data<AppState>,
        payload: web::Path<usize>,
    ) -> HttpResponse {
        let phone_id: usize = payload.into_inner();

        let phone: sqlx::Result<PhoneFullSpec> = sqlx::query_file_as!(
            PhoneFullSpec,
            "src/queries/get_phone_by_id.sql",
            phone_id as i64,
        )
        .fetch_one(&app_state.pool)
        .await;

        match phone {
            Ok(phone) => {
                let response = ChangePhoneResponse::change_phone_response(phone);
                HttpResponse::Ok().json(response)
            }
            Err(_) => HttpResponse::NotFound().into(),
        }
    }

    pub async fn get_all_phones(app_state: web::Data<AppState>) -> HttpResponse {
        match sqlx::query_file_as!(PhoneForSearch, "src/queries/get_phones.sql")
            .fetch_all(&app_state.pool)
            .await
        {
            Ok(phones) => HttpResponse::Ok().json(phones),
            Err(_) => HttpResponse::BadRequest().into(),
        }
    }
}
