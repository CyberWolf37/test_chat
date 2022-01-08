use serveur_chat::ChatManager;
use serveur_chat::utils::Message;
use uuid::Uuid;
use mongodb::{
        sync::Client,
        options::ClientOptions,
        bson::oid::ObjectId,
    };

const MONGO_URL: &'static str = "127.0.0.1:27017";


#[test]
fn it_works() {
        
        env_logger::init();
        let mut client_options = ClientOptions::parse(format!("mongodb://{}",MONGO_URL)).expect("Failed to link into DB");
        client_options.app_name = Some("test-app".to_string());

        let mut chat = ChatManager::new(Client::with_options(client_options).expect("Sorry don't have this database"));

        let salon_id = chat.add_salon();
        let user = ObjectId::new();
        chat.add_user(&salon_id,&user);
        let sms = Message::new(user.clone(), String::from("hello tout le monde"));
        chat.send_message(&salon_id,sms);
        let get = chat.get_messages_salon(&salon_id,&user);
        match get {
                Ok(v) => {
                        for i in v {
                                println!("{:?}",i) 
                        }
                },
                Err(_) => {println!("error")}
        }
}