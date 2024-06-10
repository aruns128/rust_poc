use kafka::consumer::{Consumer, FetchOffset, MessageSets, MessageSet, Message};
use std::str;
use serde_json::Value;

#[derive(Debug)]
pub struct KafkaConsumer {
 consumer: Consumer
}

impl KafkaConsumer {

 pub fn new (hosts: Vec<String>, topic: String) -> Self {
  println!("topic {}",topic);
   Self {
     consumer: Consumer::from_hosts(hosts)
                 .with_topic(topic)
                 .with_fallback_offset(FetchOffset::Latest)
                 .create()
                 .expect("REASON")
   }
 }

 pub fn get_event_data (m: &Message) -> Value {
   let event = str::from_utf8(m.value).unwrap().to_string();
   serde_json::from_str(&event).unwrap()
 }

 pub fn consume_events(&mut self) -> MessageSets {
   self.consumer.poll().unwrap()
 }

 pub fn consume_messageset(&mut self, ms: MessageSet) {
   self.consumer.consume_messageset(ms).unwrap();
 }

 pub fn commit_consumed(&mut self) {
  let _ = self.consumer.commit_consumed();
 }

}