use borsh::{BorshDeserialize, BorshSerialize};
use lapin::{Connection, ConnectionProperties, options::BasicConsumeOptions};
use futures_lite::stream::StreamExt;

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::connect(
        "amqp://guest:guest@localhost:5672",
        ConnectionProperties::default(),
    )
    .await?;

    let channel = conn.create_channel().await?;
    
    // Declare queue
    let _queue = channel
        .queue_declare(
            "user_created_queue",
            lapin::options::QueueDeclareOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await?;

    // Bind queue to exchange
    channel
        .queue_bind(
            "user_created_queue",
            "user_created",
            "user.created",
            lapin::options::QueueBindOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await?;

    // Consume messages
    let mut consumer = channel
        .basic_consume(
            "user_created_queue",
            "subscriber",
            BasicConsumeOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await?;

    println!("Listening for messages...");
    
    while let Some(delivery) = consumer.next().await {
        match delivery {
            Ok(delivery) => {
                match UserCreatedEventMessage::try_from_slice(&delivery.data) {
                    Ok(msg) => {
                        println!("In Raymundo's Computer [2406404642]. Message received: {:?}", msg);
                    }
                    Err(_) => {
                        eprintln!("Failed to deserialize message");
                    }
                }
                delivery.ack(lapin::options::BasicAckOptions::default()).await?;
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}