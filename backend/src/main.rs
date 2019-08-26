#[macro_use]
extern crate serde;
#[macro_use(bson, doc)]
extern crate mongodb;


use std::sync::{Arc};
use std::env;

use std::process::Command;

use actix_web::{middleware, web, App, HttpServer};
use mongodb::{Client, ThreadedClient, ClientInner,db::{ThreadedDatabase}};
use actix_files as fs;
use mongodb::coll::Collection;
use mongodb::db::DatabaseInner;

mod routes;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        if &args[1] == "start" {
            let child = Command::new(&args[0])
                .spawn().expect("Child process failed to start.");
            println!("child pid: {}", child.id());
        }
    } else {
        let cnt=Client::connect("localhost", 27017);
        let cnt:Arc<ClientInner> =match cnt {
            Ok(y)=>y,
            Err(_)=>panic!("Unable connect to serve")
        };
        let database=cnt.db("manthan");
        database.create_collection("users",None);
        HttpServer::new(move || {
            let col=database.collection("users");
            let mut opt=mongodb::coll::options::IndexOptions::new();
            opt.unique=Some(true);
            col.create_index(doc!{"email":1},Some(opt));
            let db = routes::Db{col};
            App::new()
                .register_data(web::Data::new(db))
                .wrap(middleware::Logger::default())
                .route("/",web::post().to(routes::save_data))
                .default_service(fs::Files::new("/", "./../frontend/")
                    .index_file("home.html")
                    .default_handler(web::route().to(routes::not_found)))
        })
            .bind("0.0.0.0:8080").unwrap()
            .run().unwrap();
    }
}