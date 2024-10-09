use actix_web::{web, HttpResponse};
use sqlx::postgres::PgQueryResult;

use crate::models::phone::Phone;
use crate::state::AppState;

pub struct PhoneController;

impl PhoneController {
    pub async fn health_check() -> HttpResponse {
        HttpResponse::Ok().into()
    }

    pub async fn get_phone(app_state: web::Data<AppState>, path: web::Path<usize>) -> HttpResponse {
        let phone_id: usize = path.into_inner();

        let phone: sqlx::Result<Phone> = sqlx::query_as!(
            Phone,
            "SELECT * FROM Phone WHERE phone_id = $1",
            phone_id as i64,
        )
        .fetch_one(&app_state.pool)
        .await;

        match phone {
            Ok(phone) => HttpResponse::Ok().json(phone),
            Err(_) => HttpResponse::NotFound().into(),
        }
    }

    pub async fn get_all_phones(app_state: web::Data<AppState>) -> HttpResponse {
        match sqlx::query_as!(Phone, "SELECT * FROM Phone")
            .fetch_all(&app_state.pool)
            .await
        {
            Ok(phones) => HttpResponse::Ok().json(phones),
            Err(_) => HttpResponse::BadRequest().into(),
        }
    }

    pub async fn delete_phone(
        app_state: web::Data<AppState>,
        path: web::Path<usize>,
    ) -> HttpResponse {
        let phone_id: usize = path.into_inner();

        let deleted: sqlx::Result<PgQueryResult> =
            sqlx::query!("DELETE FROM Phone WHERE phone_id = $1", phone_id as i64,)
                .execute(&app_state.pool)
                .await;

        match deleted {
            Ok(_) => HttpResponse::Ok().into(),
            _ => HttpResponse::BadRequest().into(),
        }
    }
}
