use uuid::Uuid;
use chrono::{
    DateTime,
    prelude::Utc,
};
use crate::error::Error;

#[derive(Debug)]
pub struct Message {
    pub id: Uuid,
    timestamp: DateTime<Utc>,
    core: String,
    pub user_id: Uuid,
}

pub trait ChatClient {
    fn getUuid(&self) -> &uuid::Uuid;
}

impl Message {
    pub fn new(user_id: Uuid, core: String) -> Self {
        Message {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            core: core,
            user_id: user_id
        }
    }
}

#[derive(Debug)]
pub struct Salon {
    pub id: Uuid,
    created_at: DateTime<Utc>,
    users: Vec<Uuid>,
    messages: Vec<Message>,
}

impl Salon {
    pub fn new() -> Self {
        Salon { 
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            users: Vec::new(),
            messages: Vec::new(),
        }
    }
    
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn add_user(&mut self, user: Uuid) {
        self.users.push(user);
    }

    pub fn remove_user(&mut self, user: &Uuid) -> Result<(), Error>  {
        match self.users.iter().enumerate().find(|&u| u.1 == user) {
            Some(u) => {
                self.users.remove(u.0);
                Ok(())
            }
            None => Err(Error::MissedUser)
        }
    }

    pub fn add_message(&mut self, message: Message) -> Result<(), Error> {
        let result = self.users.iter().find(|&&user| user == message.user_id);

        match result {
            Some(_) => {self.messages.push(message); return Ok(());}
            None => Err(Error::UnAuthorized)
        }       
    }

    pub fn get_messages(self) -> Vec<Message> {
        self.messages
    }

    pub fn has_user(&self, user: &Uuid) -> bool {
        match self.users.iter().find(|&&u| u == *user) {
            Some(_) => true,
            None => false,
        }
    }

    fn notification_users(self) {
        unimplemented!()
    }

}