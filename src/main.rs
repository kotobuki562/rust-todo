use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: Option<u32>,
    content: String,
    done: bool,
}

#[get("/todos/{id}")]
async fn get_todo(web::Path(id): web::Path<u32>) -> impl Responder {
    println!("get_todo");
    let id_option: Option<u32> = Some(id);
    HttpResponse::Ok().json(Todo {
        id: id_option,
        content: String::from("やること"),
        done: false,
    })
}

#[get("/todos")]
async fn get_posts() -> impl Responder {
    HttpResponse::Ok().body("sss")
}

#[post("/todos")]
async fn post_todo(todo: web::Json<Todo>) -> impl Responder {
    println!("post_todo");
    println!("{:?}", todo);
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| App::new().service(get_todo).service(post_todo).service(get_posts)).bind("127.0.0.1:8080")?.run().await
}