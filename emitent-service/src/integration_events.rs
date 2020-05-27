use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Connection,
    ConnectionProperties, Result,
};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct EventService {
    conn: Connection,
}

impl EventService {
    pub async fn new(uri: &str) -> Result<Self> {
        let conn = Connection::connect(uri, ConnectionProperties::default())
            .await?
            .into_inner();
        Ok(EventService { conn })
    }

    pub async fn publish<T: Serialize>(
        &self,
        data: &T,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let channel = self.conn.create_channel().await?;
        let _queue = channel
            .queue_declare(
                "emitentChan",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;
        let payload = bincode::serialize(data)?;
        let confirm = channel
            .basic_publish(
                "",
                "emitentChan",
                BasicPublishOptions::default(),
                payload.clone(),
                BasicProperties::default(),
            )
            .await?
            .await?;
        info!("produced");
        assert_eq!(confirm, Confirmation::NotRequested);
        Ok(())
    }
}
