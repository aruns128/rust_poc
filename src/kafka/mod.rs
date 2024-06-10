mod kafka_consumer;
mod kafka_producer;
mod texts;

pub use kafka_consumer::KafkaConsumer;
pub use kafka_producer::KafkaProducer;
pub use texts::Texts;



// mod kafka_consumer;
// mod kafka_producer;
// mod texts;


// use kafka_consumer::KafkaConsumer;
// use kafka_producer::KafkaProducer;
// use texts::Texts;


pub async fn handle_kafka(kafka_url: &str) -> std::io::Result<()> {
    let hosts = vec![kafka_url.to_string()];

    let mut texts = Texts::new();
    let mut consumer = KafkaConsumer::new(hosts.clone(), "actions".to_string());
    let mut producer = KafkaProducer::new(hosts);

    log::info!("Kafka Consumer and Producer started...");

    loop {
        for ms in consumer.consume_events().iter() {
            for m in ms.messages() {
                let event_data = KafkaConsumer::get_event_data(m);
                let action = event_data["action"].to_string();

                if action == "\"add\"" {
                    log::info!("added");
                    texts.add_text(event_data["value"].to_string());
                } else if action == "\"remove\"" {
                    log::info!("removed");
                    let index = event_data["value"].to_string().parse::<usize>().unwrap();
                    texts.remove_text(index);
                } else {
                    log::error!("Invalid action");
                }

                producer.send_data_to_topic("texts", texts.to_json());
            }
            consumer.consume_messageset(ms);
        }
        consumer.commit_consumed();
    }
}




// fn get_event_data(m: &Message) -> Value {
//     let event = std::str::from_utf8(m.value).unwrap().to_string();
//     serde_json::from_str(&event).unwrap()
// }

