use crate::emitent::{Emitent, EmitentRepository};
use async_trait::async_trait;
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

#[async_trait]
impl EmitentRepository for Repository {
    fn store(&self, e: &Emitent) -> Result<(), Box<dyn std::error::Error>> {
        let collection = self.db.collection("emitents");
        let serialized_emitent = bson::to_bson(e)?; // Serialize

        // TODO: Remove error double checking
        //collection.insert_one(document, None)?; // Insert into a MongoDB collection

        if let bson::Bson::Document(document) = serialized_emitent {
            collection.insert_one(document, None)?; // Insert into a MongoDB collection
        } else {
            println!("Error converting the BSON object into a MongoDB document");
        }

        Ok(())
    }
    async fn get_all(&self) -> Result<Vec<Emitent>, Box<dyn std::error::Error>> {
        let collection = self.db.collection("emitents");
        let mut cursor = collection.find(None, None)?;

        let mut output = vec![];
        // Iterate over the results of the cursor.

        while let Some(result) = cursor.next() {
            match result {
                Ok(document) => {
                    let e = bson::from_bson(bson::Bson::Document(document))?;
                    output.push(e);
                    // if let Some(title) = document.get("title").and_then(Bson::as_str) {
                    //     println!("title: {}", title);
                    // } else {
                    //     println!("no title found");
                    // }
                }
                Err(e) => return Err(e.into()),
            }
        }

        Ok(output)
    }
}
