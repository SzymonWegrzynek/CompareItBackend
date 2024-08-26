// use std::fs;


// pub async fn insert_img<E>(phone_id: i32, image_path: &str, pool: &PgPool) -> Result<((), sqlx::Error), E> {
//     let image_data = fs::read(image_path);

//     let result = sqlx::query!("UPDATE phones SET images = $1 WHERE id = $2", image_data, phone_id).execute(pool).await;

//     match result {
//         Ok(_) => {println!("image successfully insert"); Ok(())}
//         Err(e) => {println!("failed to insert image: {}", e); Err(e)}
//     }
// }



// let phone_id = 1;
// let image_path = "./assets/image.png";

// database::insert_img::insert_img(phone_id, image_path, &pool).await?;
