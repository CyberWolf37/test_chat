use std::fmt::{Display, Formatter, Result};

pub enum Error {
    UnAuthorized,
    MissedSalon,
    MissedUser,
    UnAuthorizedClient,
    QueryError,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        
        let speak = match self {
            UnAuthorized => "You are not authorized to this scope",
            MissedSalon => "The Salon was missed",
            MissedUser => "The User was missed",
            UnAuthorizedClient => "The user was not authorized",
            QueryError => "The query has failed",
        };

        write!(f, "{}", speak)
    }
}