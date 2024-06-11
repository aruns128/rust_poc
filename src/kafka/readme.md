to run this in Macos

# steps:

1. Install kafka (depends on your machine)
2. write topics
3. read topics

# create texts topic

- `kafka-topics --create --topic texts --bootstrap-server localhost:9092`

# create actions topic

- `kafka-topics  --create --topic actions --bootstrap-server localhost:9092`

3. Run consumer in kafka
   `kafka-console-consumer --bootstrap-server localhost:9092 --topic texts`

4. Run producer in kafka

- `kafka-console-producer --bootstrap-server localhost:9092 --topic actions`

5. run `cargo run kafka`

6. Feed message to the producer.

   > { "action": "add", "value": "first text" }
   > { "action": "add", "value": "second text" }
   > { "action": "remove", "value": 1 }
   > { "action": "add", "value": "third text" }

7. output in consumer console.

["\"first text\""]
["\"first text\"","\"second text\""]
["\"first text\""]
["\"first text\"","\"third text\""]
