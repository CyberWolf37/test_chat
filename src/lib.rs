pub mod utils;
mod error;

use crate::utils::*;
use uuid::Uuid;
use crate::error::Error;
use log::{info, trace, warn, debug};
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
}

impl ChatManager {
    pub fn new(connection: Client) -> Self {
        let db = connection.database("chat");
        let collection_salon = db.collection::<Salon>("Salon");

        ChatManager {
            connection: collection_salon,
        }
    }

    pub fn add_salon(&mut self, name: &str) -> ObjectId {
        let salon = Salon::new(name);
        let salon_id = salon.id;
        self.connection.insert_one(salon,None);

        info!("Salon id:{} was added",&salon_id);
        salon_id
    }

    pub fn add_user(&mut self, salon: &ObjectId, user: &ObjectId) -> Result<(), Error> {
        let filter = doc! { "$and" : [ {"_id": salon}, { "$or": [ { "users": { "$elemMatch": { "$ne" : user} } }, { "users": { "$size": 0 } } ] } ] };
        let update = doc! { "$push" : {"users" : user }};
        let salon_db = self.connection.update_one(filter, update, None);

        match salon_db {
            Ok(_) => {
                info!("User id:{} was added in Salon id:{}",&user,&salon);
                Ok(())
            },
            Err(er) => {
                eprintln!("Error during inserting user :{}",er);
                Err(Error::MissedSalon)
            }
        }
    }

    pub fn add_user_client(&mut self, salon: &ObjectId, user: &impl ChatClient) -> Result<(), Error> {
        self.add_user(salon, user.getId())
    }

    pub fn delete_user(&mut self, salon: &ObjectId, user: &ObjectId) -> Result<(), Error> {
        let filter = doc! {"_id": salon};
        let update = doc! { "$pull" : {"users" : { "$eq" : user }}};
        let salon_db = self.connection.update_one(filter, update, None);

        match salon_db {
            Ok(_) => {
                info!("User id:{} was deleted to Salon id:{}",&user,&salon);
                Ok(())
            },
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
            Ok(_) => {
                info!("Salon id:{} was deleted",&salon);
                Ok(())
            },
            Err(er) => {
                eprintln!("Error during deleting Salon :{}",er);
                Err(Error::MissedSalon)
            }
        }
    }

    pub fn send_message(&mut self, salon: &ObjectId, message: Message) -> Result<(), Error> {
        let filter = doc! {"$and" : [ {"_id": salon} , {"users": { "$elemMatch": { "$eq" : message.user_id}}}]};
        let doc = bson::to_document(&message).expect("Failed to parse Message to Document");
        let update = doc! { "$push" : {"messages" : doc }};
        let salon_db = self.connection.update_one(filter, update, None);

        match salon_db {
            Ok(_) => {
                info!("Message send by id:{} in Salon id:{}",&message.user_id,&salon);
                Ok(())
            },
            Err(er) => {
                eprintln!("Error during deleting user :{}",er);
                Err(Error::MissedSalon)
            }
        }
    }

    pub fn get_messages_salon(&self, salon: &ObjectId, user: &ObjectId) -> Result<Vec<Message>, Error> {
        let filter = doc! {"$and" : [ {"_id": salon} , {"users": { "$elemMatch": { "$eq" : user}}}]};
        let salon_db = self.connection.find_one(filter,None);
        let _messages: Vec<Message> = Vec::new();

        match salon_db {
            Ok(res) => {
                if let Some(mes) = res {
                    info!("User id:{} request messages in Salon id:{}",&user,&salon);
                    return Ok(mes.get_messages());
                }
                else {
                    Err(Error::MissedSalon)
                }
            },
            Err(er) => {
                eprintln!("Error during get messages :{}",er);
                Err(Error::MissedSalon)
            }
        }
    }

    fn notification_users(&self, salon: &ObjectId) {
        unimplemented!()
    }


}