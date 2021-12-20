use crate::utils::*;
use uuid::Uuid;
use crate::error::Error;

pub struct chatManager {
    list_salon: Vec<Salon>,
}

impl chatManager {
    pub fn new() -> Self {
        chatManager {
            list_salon: Vec::new(),
        }
    }

    pub fn add_salon(&mut self) {
        self.list_salon.push(Salon::new());
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

    pub fn send_message(&self, salon: &Uuid, message: Message) -> Result<(), Error> {
        let salon = self.list_salon.iter().find(|&s| s.id == *salon);

        match salon {
            Some(s) => {
                s.has_user(&message.user_id);
                Ok(())
            }
            None => {
                Err(Error::UnAuthorizedClient)
            }
        }
    }
}