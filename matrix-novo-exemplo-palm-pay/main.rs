use std::env;

mod config;
mod server;

#[tokio::main]
async fn main() {
    config::init();
    
    let mut app = server::App::new();
    app.start().await;
    
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    
    if let Err(err) = app.run(&port).await {
        panic!("{}", err);
    }
    
    app.stop().await;
}