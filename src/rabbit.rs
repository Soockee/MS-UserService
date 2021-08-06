use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};

fn test() -> Result<()> {

	const NAME:&str = "msCloudBois";
	const PASSWORD:&str = "_420this_is_madness_1337";
	const HOST:&str = "master.ms.depressive.life:5672"; 


	let url:String = format!("amqp://{}:{}@{}", NAME, PASSWORD, HOST);

	let mut connection = Connection::insecure_open(&url)?;

	let channel = connection.open_channel(None)?;

	let queue = channel.queue_declare("hello", QueueDeclareOptions::default())?;

	let consumer = queue.consume(ConsumerOptions::default())?;

	println!("Waiting for messages. Press Ctrl-C to exit.");

	for (i, message) in consumer.receiver().iter().enumerate() {

		match message {

			ConsumerMessage::Delivery(delivery) => {

				let body = String::from_utf8_lossy(&delivery.body);

				println!("({:>3}) Received [{}]", i, body);

				consumer.ack(delivery)?;

			}

			other => {

				println!("Consumer ended: {:?}", other);

				break;

			}

		}

	}

	connection.close()
}