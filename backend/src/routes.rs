use actix_web::{HttpResponse, HttpRequest,web};
use mongodb::{ClientInner,ThreadedClient};
use actix_web::web::Form;
use std::sync::Arc;
use mongodb::coll::Collection;
use bson::Bson;

//use sendgrid::SGClient;
//use sendgrid::{Destination, Mail};

pub fn not_found(_req:HttpRequest) ->HttpResponse{
    HttpResponse::NotFound().content_type("text/html").body(include_str!("../../frontend/404.html"))
}

#[derive(Debug)]
pub struct Db{
    pub col:Collection
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MyParams {
    first_name:String,
    last_name:String,
    #[serde(default="default")]
    roll_number:String,
    branch:String,
    email:String,
    phone:String,
    #[serde(default="default")]
    os_link:String,
    #[serde(default="default")]
    message:String,
    #[serde(default="default")]
    other:String,
    #[serde(default="default")]
    designer:String,
    #[serde(default="default")]
    year:String,
    anime:String,
}

    fn default()->String{
        String::from("None")
    }

pub fn save_data(db:web::Data<Db>,eq:HttpRequest,data:Form<MyParams>)->HttpResponse{
    let d=data.into_inner();
    let MyParams {first_name, last_name, roll_number, branch, email, phone, os_link, message, other, designer, year,anime }=d;
//    let sg = SGClient::new("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");
//    let mut full_name=first_name.clone();
//    full_name.push_str(" ");
//    full_name.push_str(last_name.as_str());
//    let mail_info = Mail::new()
//        .add_to(Destination {
//            address: email.as_str(),
//            name: full_name.as_str(),
//        })
//        .add_from("manthan@manan.club")
//        .add_subject("Manthan Registration")
//        .add_html("<h1>Congration you have been registered successfully!</h1>")
//        .add_from_name("Manthan")
//        .add_header("x-cool".to_string(), "indeed")
//
//    match sg.send(mail_info) {
//        Err(err) => println!("Error: {}", err),
//        Ok(body) => println!("Response: {}", body),
//    };
    db.col.insert_one(doc!{
    "first_name"=>first_name,
    "last_name"=>last_name,
    "roll_number"=>roll_number,
    "branch"=>branch,
    "email"=>email,
    "phone"=>phone,
     "os_link"=>os_link,
     "message"=>message,
     "other"=>other,
     "designer"=>designer,
     "year"=>year,
     "anime"=>anime,
    },None);
    HttpResponse::NotFound().content_type("text/html").body(include_str!("../../frontend/successfully resgistered.html"))
}
