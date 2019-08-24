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

//fn main(){
//    use std::process::Command;
//    use std::env;
//
//    fn main() {
//        let args: Vec<String> = env::args().collect();
//        println!("{:?}",args);
//        if args.len() == 2 {
//            if &args[1] == "start" {
//                let child = Command::new(&args[0])
//                    .spawn().expect("Child process failed to start.");
//                println!("child pid: {}", child.id());
//                // child.forget() No Child Left Behind
//            }
//        } else {
//            std::env::set_var("RUST_LOG", "actix_web=info");

//        }
//    }
//}

use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        if &args[1] == "start" {
            let child = Command::new(&args[0])
                .spawn().expect("Child process failed to start.");
            println!("child pid: {}", child.id());
            // child.forget() No Child Left Behind
        }
    } else {
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
                .default_service(fs::Files::new("/", "./../frontend/")
                    .index_file("home.html")
                    .default_handler(web::route().to(routes::notFound)))
        })
            .bind("127.0.0.1:8080").unwrap()
            .run();
    }
}