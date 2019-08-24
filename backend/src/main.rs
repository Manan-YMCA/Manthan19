use std::io;
use std::sync::{Arc};
use std::env;

use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use mongodb::{Client, ThreadedClient, ClientInner,db::{ThreadedDatabase,options}};
use actix_web::http::Method;
use actix_files as fs;

mod routes;

fn index(client: web::Data<Arc<ClientInner>>, req: HttpRequest) -> HttpResponse {
    let db = client.db("movies");
    let col=db.create_collection("comedies",None);
    println!("{:?}",col);
    HttpResponse::Ok().body(format!("Num of requests:"))
}

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    let cnt=Client::connect("localhost", 27017);
    let cnt:Arc<ClientInner> =match cnt {
        Ok(y)=>y,
        Err(_)=>panic!("Unable connect to serve")
    };
    let db = web::Data::new(cnt.clone());
    HttpServer::new(move || {
        App::new()
            .register_data(db.clone())
            .wrap(middleware::Logger::default())
            .route("*",web::route().to(routes::notFound))
    })
        .bind("127.0.0.1:8080")?
        .run()

}
