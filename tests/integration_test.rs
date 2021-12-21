use serveur_chat::ChatManager;
use serveur_chat::utils::Message;
use uuid::Uuid;


#[test]
fn it_works() {
        let mut chat = ChatManager::new();
        let salon_id = chat.add_salon();
        println!("Salon id = {:?}", salon_id);
        let user = Uuid::new_v4();
        chat.add_user(&salon_id,&user);
        let sms = Message::new(user.clone(), String::from("hello tout le monde"));
        chat.send_message(&salon_id,sms);
        println!("{:?}",chat)
}