use crate::emitent::{Emitent, EmitentRepository};
use mongodb::{options::ClientOptions, Client, Database};

pub struct Repository {
    db: Database,
}

impl Repository {
    pub fn new(con_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // let mut client_options = ClientOptions::parse("mongodb://localhost:27017")?;
        let mut client_options = ClientOptions::parse(con_str)?;

        // Manually set an option.
        client_options.app_name = Some("My App".to_string());

        // Get a handle to the deployment.
        let client = Client::with_options(client_options)?;

        Ok(Repository {
            db: client.database("mydb"),
        })
    }
}

impl EmitentRepository for Repository {
    fn store(&self, e: &Emitent) -> Result<(), Box<dyn std::error::Error>> {
        let collection = self.db.collection("emitents");
        let serialized_person = bson::to_bson(e)?; // Serialize

        if let bson::Bson::Document(document) = serialized_person {
            collection.insert_one(document, None)?; // Insert into a MongoDB collection
        } else {
            println!("Error converting the BSON object into a MongoDB document");
        }

        Ok(())
    }
}
