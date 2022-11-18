use std::sync::mpsc::channel;

use lapin::options::{BasicPublishOptions, ExchangeDeclareOptions};
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind};
use lapin::types::FieldTable;
use tokio_amqp::*;

use crate::config::Config;

#[derive(Clone)]
pub struct AmpqService {
    channel: Option<Channel>,
    network_name: String,
    router_key: String
}

impl AmpqService {
    pub async fn new(config: &Config) -> Self {
        let channel = if config.ampq_addr == String::from("") {
            None
        } else {
            let conn = Connection::connect(
                &config.ampq_addr,
                ConnectionProperties::default().with_tokio(),
            )
            .await
            .unwrap(); // Note the `with_tokio()` here
            let channel =
                conn.create_channel()
                    .await
                    .expect("Cant connect to channel");
            channel.exchange_declare(
                "DiscordBot",
                ExchangeKind::Fanout,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            ).await.unwrap();
            Some(channel)
        };


        let router_key = config.ampq_router_key.clone();
        dbg!(&config.ampq_addr);

        AmpqService {
            channel,
            network_name: config.chain_id_name.clone(),
            router_key
        }
    }

    pub async fn send(&self, msg: String) {
        let message = format!("[{}]\n {}", &self.network_name, msg);
        dbg!(&message);

        match &self.channel {
            Some(ch) => {
                match ch.basic_publish(
                    "DiscordBot",
                    &self.router_key,
                    BasicPublishOptions::default(),
                    message.into(),
                    BasicProperties::default(),
                )
                .await {
                    Ok(_) => {
                        dbg!("Message sent");
                    }
                    Err(e) => {
                        dbg!("Error sending message: {}", e);
                    }
                }
            }
            _ => {
                println!("{}", message);
            }
        }
    }
}
