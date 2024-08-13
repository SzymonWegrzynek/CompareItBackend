use std::error::Error;
// use sqlx::Row;

struct Phones {
    pub id: i32,
    pub manufacturer: String,
    pub model: String,
    pub price: String,
    pub colors: Option<String>,
    pub internal_memory_variants: Option<String>,
    pub battery_capacity: Option<String>,
    pub screen_size: Option<String>,
    pub screen_technology: Option<String>,
    pub resolution: Option<String>,
    pub screen_refresh_rate: Option<String>,
    pub ram_memory: Option<String>,
    pub camera: Option<String>,
    pub processor: Option<String>,
    pub processor_speed: Option<String>,
    pub processor_cores: Option<i32>,
    pub operating_system: Option<String>,
    pub connectivity: Option<String>,
    pub dust_water_resistance_standard: Option<String>,
    pub sim_card_type: Option<String>,
}

async fn create(phones: &Phones, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO phones (id, manufacturer, model, price, colors, internal_memory_variants, 
    battery_capacity, screen_size, screen_technology, resolution,screen_refresh_rate, ram_memory, camera, 
    processor, processor_speed, processor_cores, operating_system, connectivity, dust_water_resistance_standard, 
    sim_card_type) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)";

    sqlx::query(query)
    .bind(&phones.id)
    .bind(&phones.manufacturer)
    .bind(&phones.model)
    .bind(&phones.price)
    .bind(&phones.colors)
    .bind(&phones.internal_memory_variants)
    .bind(&phones.battery_capacity)
    .bind(&phones.screen_size)
    .bind(&phones.screen_technology)
    .bind(&phones.resolution)
    .bind(&phones.screen_refresh_rate)
    .bind(&phones.ram_memory)
    .bind(&phones.camera)
    .bind(&phones.processor)
    .bind(&phones.processor_speed)
    .bind(&phones.processor_cores)
    .bind(&phones.operating_system)
    .bind(&phones.connectivity)
    .bind(&phones.dust_water_resistance_standard)
    .bind(&phones.sim_card_type)
    .execute(pool)
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://admin:admin@localhost:5432/ci_db";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let phone = Phones{
        id: 1,
        manufacturer: "Samsung".to_string(),
        model: "Galaxy S24".to_string(),
        price: "3499.00 PLN".to_string(),
        colors: Some("Marble Gray".to_string()),
        internal_memory_variants: Some("128GB, 256GB".to_string()),
        battery_capacity: Some("4000 mAh".to_string()),
        screen_size: Some("6.2\"".to_string()),
        screen_technology: Some("Dynamic AMOLED 2X".to_string()),
        resolution: Some("2340 x 1080".to_string()),
        screen_refresh_rate: Some("120 Hz".to_string()),
        ram_memory: Some("8 GB".to_string()),
        camera: Some("Rear 50MP + 12MP + 10MP, Front 12MP".to_string()),
        processor: Some("Samsung Exynos 2400".to_string()),
        processor_speed: Some("3.2 GHz".to_string()),
        processor_cores: Some(10),
        operating_system: Some("Android".to_string()),
        connectivity: Some("5G, NFC, Wi-Fi, Bluetooth 5.3, USB Type-C".to_string()),
        dust_water_resistance_standard: Some("IP68".to_string()),
        sim_card_type: Some("NanoSIM, eSIM".to_string()),
    };

    create(&phone, &pool).await?;

    // let res = sqlx::query("SELECT 1 + 1 AS sum")
    // .fetch_one(&pool)
    // .await?;

    // let sum: i32 = res.get("sum");
    // println!("1 + 1 = {}", sum);

    Ok(())
}
