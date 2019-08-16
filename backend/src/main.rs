use actix_web::{web, App, HttpServer, middleware, HttpResponse};
use actix_files;
use actix_web::http::Method;

fn index()->HttpResponse{
    HttpResponse::Ok().content_type("text/html").body("<H1>Hello<H1>")
}

fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
//            .service(actix_files::Files::new("/static", "./src/static"))
            .route("/static/hello",web::route().method(Method::GET).to(index))
            .wrap(middleware::Logger::default()))
        .bind("127.0.0.1:8080").unwrap().run()
}