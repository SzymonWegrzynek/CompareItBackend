SELECT 
    Phone.phone_id,
    Producer.producer, 
    Model.model, 
    Model.price, 
    Battery.battery_capacity, 
    Screen.screen_size, 
    Screen.screen_technology, 
    Screen.resolution, 
    Screen.screen_refresh_rate, 
    Ram.ram_memory, 
    Camera.front_camera, 
    Camera.rear_camera, 
    Processor.processor_name, 
    Processor.processor_cores, 
    OperatingSystem.operating_system, 
    WaterDustResistance.water_dust_resistance,
    ARRAY_AGG(DISTINCT Color.color) AS colors,       
    ARRAY_AGG(DISTINCT SimCard.sim_card_type) AS sim_cards, 
    ARRAY_AGG(DISTINCT InternalMemory.internal_memory_variant) AS memory_variants,
    ARRAY_AGG(DISTINCT Connectivity.connectivity) AS connectivities,
    (
        SELECT ARRAY_AGG(image_url ORDER BY image_id ASC)
        FROM Image
        WHERE Image.model_id = Phone.model_id
    ) AS images   
FROM Phone
JOIN Producer ON Phone.producer_id = Producer.producer_id
JOIN Model ON Phone.model_id = Model.model_id
JOIN Battery ON Phone.battery_id = Battery.battery_id
JOIN Screen ON Phone.screen_id = Screen.screen_id
JOIN Ram ON Phone.ram_id = Ram.ram_id
JOIN Camera ON Phone.camera_id = Camera.camera_id
JOIN Processor ON Phone.processor_id = Processor.processor_id
JOIN OperatingSystem ON Phone.operating_system_id = OperatingSystem.operating_system_id
JOIN WaterDustResistance ON Phone.water_dust_resistance_id = WaterDustResistance.water_dust_resistance_id
LEFT JOIN Color ON Color.model_id = Phone.model_id 
LEFT JOIN SimCard ON SimCard.model_id = Phone.model_id 
LEFT JOIN InternalMemory ON InternalMemory.model_id = Phone.model_id
LEFT JOIN Connectivity ON Connectivity.model_id = Phone.model_id 
LEFT JOIN Image ON Image.model_id = Phone.model_id 
WHERE phone_id = $1
GROUP BY 
    Phone.phone_id,
    Producer.producer, 
    Model.model, 
    Model.price, 
    Battery.battery_capacity, 
    Screen.screen_size, 
    Screen.screen_technology, 
    Screen.resolution, 
    Screen.screen_refresh_rate, 
    Ram.ram_memory, 
    Camera.front_camera, 
    Camera.rear_camera, 
    Processor.processor_name, 
    Processor.processor_cores, 
    OperatingSystem.operating_system, 
    WaterDustResistance.water_dust_resistance;
