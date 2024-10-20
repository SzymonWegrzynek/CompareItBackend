SELECT 
    Phone.phone_id,
    Producer.producer, 
    Model.model
FROM Phone
JOIN Producer ON Phone.producer_id = Producer.producer_id
JOIN Model ON Phone.model_id = Model.model_id
GROUP BY 
    Phone.phone_id,
    Producer.producer, 
    Model.model;
    