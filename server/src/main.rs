use actix_web::{web, App, HttpServer};

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Port 8080 if in development, otherwise port 80
    #[cfg(debug_assertions)]
    let port = 8080;
    #[cfg(not(debug_assertions))]
    let port = 80;
    
    println!("Starting Actix-web server on http://0.0.0.0:{}", port);
    HttpServer::new(|| {
        println!("Building app!");
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(
                web::scope("/api")
                    .service(api::world::get_world)
            )
    })
    // Docker requires 0.0.0.0 and i wasted over an hour of my life
    // figuring this out.
    .bind(("0.0.0.0", port))?
    .run()
    .await
}