use serveur_chat::ChatManager;
use serveur_chat::utils::Message;
use uuid::Uuid;
use mongodb::{Client, ClientOptions};

const MONGO_URL: &'static str = "127.0.0.1:27017";


#[test]
fn it_works() {
        let mut client_options = ClientOptions::parse(format!("mongodb://{}",MONGO_URL));
        client_options.app_name = Some("test-app".to_string());

        let mut chat = ChatManager::new(Client::with_options(client_options).expect("Sorry don't have this database"));

        let salon_id = chat.add_salon();
        println!("Salon id = {:?}", salon_id);
        let user = Uuid::new_v4();
        chat.add_user(&salon_id,&user);
        let sms = Message::new(user.clone(), String::from("hello tout le monde"));
        chat.send_message(&salon_id,sms);
        println!("{:?}",chat)
}