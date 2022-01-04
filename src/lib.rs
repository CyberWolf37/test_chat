pub mod utils;
mod error;

use crate::utils::*;
use uuid::Uuid;
use crate::error::Error;
use mongodb::{
    sync::Client,
    sync::Collection,
    bson::doc,
    bson,
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

    pub fn add_salon(&mut self) -> Uuid {
        let salon = Salon::new();
        let salon_id = salon.id;
        self.connection.insert_one(salon,None);
        //self.list_salon.push(salon);

        salon_id
    }

    pub fn add_user(&mut self, salon: &Uuid, user: &Uuid) -> Result<(), Error> {
        let filter = doc! { "&and" : [ {"id": salon.to_string()}, { "users": { "$elemMatch": { "$not" : {"&eq" : user.to_string()} } } } ] };
        let update = doc! { "&push" : {"users" : user.to_string()}};
        let salon_db = self.connection.update_one(filter, update, None);

        match salon_db {
            Ok(_) => Ok(()),
            Err(er) => {
                eprintln!("Error during inserting user :{}",er);
                Err(Error::MissedSalon)
            }
        }
    }

    pub fn delete_user(&mut self, salon: &Uuid, user: &Uuid) -> Result<(), Error> {
        let filter = doc! {"id": salon.to_string()};
        let update = doc! { "&pull" : {"users" : { "&eq" : user.to_string()}}};
        let salon_db = self.connection.update_one(filter, update, None);

        match salon_db {
            Ok(_) => Ok(()),
            Err(er) => {
                eprintln!("Error during deleting user :{}",er);
                Err(Error::MissedSalon)
            }
        }
    }

    pub fn delete_salon(&mut self, salon: &Uuid) -> Result<(), Error> {
        let filter = doc! {"id": salon.to_string()};
        let salon_db = self.connection.delete_one(filter, None);

        match salon_db {
            Ok(_) => Ok(()),
            Err(er) => {
                eprintln!("Error during deleting Salon :{}",er);
                Err(Error::MissedSalon)
            }
        }
    }

    pub fn send_message(&mut self, salon: &Uuid, message: Message) -> Result<(), Error> {
        let filter = doc! {"&and" : [ {"id": salon.to_string()} , {"users": { "&eq": message.id.to_string() }}]};
        let doc = utils::MessageTest {
            core: "hello world".to_string(),
            id: 32,
        };
        let update = doc! { "&push" : {"messages" : doc }};
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