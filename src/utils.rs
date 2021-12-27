use uuid::Uuid;
use chrono::{
    DateTime,
    prelude::Utc,
};
use crate::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    #[serde(with = "my_date_format")]
    timestamp: DateTime<Utc>,
    core: String,
    pub user_id: Uuid,
}

mod my_date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Salon {
    pub id: Uuid,
    #[serde(with = "my_date_format")]
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