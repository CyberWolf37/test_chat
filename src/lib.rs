pub mod utils;
mod error;

use crate::utils::*;
use uuid::Uuid;
use crate::error::Error;
use mongodb::Client;

#[derive(Debug)]
pub struct ChatManager {
    connection: Client,
    list_salon: Vec<Salon>,
}

impl ChatManager {
    pub fn new(connection: Client) -> Self {
        let db = connection.database("chat");
        let collection = db.collection::<Salon>("Salon");
        
        ChatManager {
            list_salon: Vec::new(),
            connection: connection,
        }
    }

    pub fn add_salon(&mut self) -> Uuid {
        let salon = Salon::new();
        let salon_id = Uuid::from(salon.id);
        self.list_salon.push(salon);

        salon_id
    }

    pub fn add_user(&mut self, salon: &Uuid, user: &Uuid) -> Result<(), Error> {
        let salon = self.list_salon.iter_mut().find(|s| s.id == *salon);

        match salon {
            Some(s) => {
                s.add_user(*user);
                Ok(())
            }
            None => Err(Error::MissedSalon)
        }
    }

    pub fn delete_user(&mut self, salon: &Uuid, user: &Uuid) -> Result<(), Error> {
        let salon = self.list_salon.iter_mut().enumerate().find(|s| s.1.id == *salon);

        match salon {
            Some(s) => {
                s.1.remove_user(user)
            }
            None => {
                Err(Error::MissedSalon)
            }
        }
    }

    pub fn delete_salon(&mut self, salon: &Uuid) -> Result<(), Error> {
        let salon = self.list_salon.iter().enumerate().find(|s| s.1.id == *salon);

        match salon {
            Some(s) => {
                self.list_salon.remove(s.0);
                Ok(())
            }
            None => {
                Err(Error::MissedSalon)
            }
        }
    }

    pub fn send_message(&mut self, salon: &Uuid, message: Message) -> Result<(), Error> {
        let salon = self.list_salon.iter_mut().find(|s| s.id == *salon);

        match salon {
            Some(s) => {
                s.add_message(message)
            }
            None => {
                Err(Error::UnAuthorizedClient)
            }
        }
    }


}