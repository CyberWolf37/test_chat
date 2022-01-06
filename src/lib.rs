pub mod utils;
mod error;

use crate::utils::*;
use uuid::Uuid;
use crate::error::Error;
use mongodb::{
    sync::Client,
    sync::Collection,
    bson::doc,
    bson::oid::ObjectId,
    options::FindOptions,
};

#[derive(Debug)]
pub struct ChatManager {
    connection: Collection<utils::Salon>,
    list_salon: Vec<Salon>,
}

impl ChatManager {
    pub fn new(connection: Client) -> Self {
        let db = connection.database("chat");
        let collection_salon = db.collection::<Salon>("Salon");

        ChatManager {
            list_salon: Vec::new(),
            connection: collection_salon,
        }
    }

    pub fn add_salon(&mut self) -> ObjectId {
        let salon = Salon::new();
        let salon_id = salon.id;
        self.connection.insert_one(salon,None);

        salon_id
    }

    pub fn add_user(&mut self, salon: &ObjectId, user: &ObjectId) -> Result<(), Error> {
        let filter = doc! { "$and" : [ {"_id": salon}, { "users": { "$elemMatch": { "$not" : {"$eq" : user} } } } ] };
        let update = doc! { "$push" : {"users" : user }};
        let salon_db = self.connection.update_one(filter, update, None);

        match salon_db {
            Ok(_) => Ok(()),
            Err(er) => {
                eprintln!("Error during inserting user :{}",er);
                Err(Error::MissedSalon)
            }
        }
    }

    pub fn delete_user(&mut self, salon: &ObjectId, user: &ObjectId) -> Result<(), Error> {
        let filter = doc! {"_id": salon};
        let update = doc! { "$pull" : {"users" : { "$eq" : user }}};
        let salon_db = self.connection.update_one(filter, update, None);

        match salon_db {
            Ok(_) => Ok(()),
            Err(er) => {
                eprintln!("Error during deleting user :{}",er);
                Err(Error::MissedSalon)
            }
        }
    }

    pub fn delete_salon(&mut self, salon: &ObjectId) -> Result<(), Error> {
        let filter = doc! {"_id": salon};
        let salon_db = self.connection.delete_one(filter, None);

        match salon_db {
            Ok(_) => Ok(()),
            Err(er) => {
                eprintln!("Error during deleting Salon :{}",er);
                Err(Error::MissedSalon)
            }
        }
    }

    pub fn send_message(&mut self, salon: &ObjectId, message: Message) -> Result<(), Error> {
        let filter = doc! {"$and" : [ {"_id": salon} , {"users": { "$eq": message.id }}]};
        let doc = bson::to_document(&message).expect("Failed to parse Message to Document");
        let update = doc! { "$push" : {"messages" : doc }};
        let salon_db = self.connection.update_one(filter, update, None);

        match salon_db {
            Ok(_) => Ok(()),
            Err(er) => {
                eprintln!("Error during deleting user :{}",er);
                Err(Error::MissedSalon)
            }
        }
    }


}