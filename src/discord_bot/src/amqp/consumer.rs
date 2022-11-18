use std::str;

use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties, Channel, Consumer};
use serenity::futures::StreamExt;
use tokio_amqp::LapinTokioExt;
use tracing::info;

#[derive(Clone)]
pub struct AmpqConsumer {
    channel: Option<Channel>,
    consumer: Option<Consumer>,
}

impl AmpqConsumer {
    pub async fn new() -> Self {
        tracing_subscriber::fmt::init();

        let addr = std::env::var("CLOUDAMQP_URL").unwrap_or("amqp://127.0.0.1:5672/%2f".into());
        let exchange = std::env::var("CLOUDAMQP_EXCHANGE").unwrap_or("DiscordBot".into());
        println!("{}", addr);

        let conn = Connection::connect(&addr, ConnectionProperties::default())
            .await
            .expect("connection error");

        info!("CONNECTED");

        //receive channel
        let channel = conn.create_channel().await.expect("create_channel");
        info!(state=?conn.status().state());

        let queue = channel
            .queue_declare(
                "liquidation_state",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("queue_declare");
        info!(state=?conn.status().state());
        info!(?queue, "Declared queue");

        channel.queue_bind(
            "liquidation_state",
            exchange.as_str(),
            "",
            QueueBindOptions::default(),
            FieldTable::default(),
        ).await.expect("queue_bind");

        info!("will consume");
        let mut consumer = channel
            .basic_consume(
                "liquidation_state",
                "discord_bot",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("basic_consume");
        info!(state=?conn.status().state());

        AmpqConsumer {
            channel: Some(channel),
            consumer: Some(consumer),
        }
    }

    pub async fn start_consuming (&mut self) -> String {
        match &mut self.consumer {
            Some(consumer) => {
                match consumer.next().await {
                    Some(delivery) => {
                        info!(message=?delivery, "received message");
                        match delivery {
                            Ok(delivery) => {
                                let s = match str::from_utf8(&delivery.1.data) {
                                    Ok(v) => v,
                                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                                };
                                s.to_string()
                            },
                            Err(e) => panic!("Error: {}", e),
                        }
                    },
                    None => panic!("No message")
                }
            },
            None => panic!("No consumer")
        }
    }
}

