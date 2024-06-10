use kafka::producer::{Producer, Record};

pub struct KafkaProducer {
  producer: Producer
}

impl KafkaProducer {

  pub fn new(hosts: Vec<String>) -> Self {
    let producer =
      Producer::from_hosts(hosts)
        .create()
        .expect("REASON");
    
    Self {
      producer: producer
    }
  }

  pub fn send_data_to_topic(&mut self, topic: &str, data: String ) {
    let record = Record::from_value( topic, data.as_bytes() );
    self.producer.send(&record).unwrap();
  }

}