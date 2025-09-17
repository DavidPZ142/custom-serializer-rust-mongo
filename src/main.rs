use mongodb::bson;
use serde::{ Serialize};
use bson::serde_helpers::bson_datetime_as_rfc3339_string;

fn main() {
    println!("Hello, world!");
    let user = User {
        id : "1".to_owned(),
        name : "David".to_owned(),
        age : 24,
        birthday : bson::DateTime::from_millis(977875200000)
    };

    let string_user = serde_json::to_string(&user).unwrap();
    let document_user = bson::to_document(&user).unwrap();
    println!("{:?}",document_user);
    println!("{}", string_user);

}

#[derive(Serialize)]
struct User{
    #[serde(rename="_id")]
    id : String,
    name:String,
    age : u16,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    birthday: bson::DateTime
}