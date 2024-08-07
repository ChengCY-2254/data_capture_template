mod traits;
mod parse;
mod build_info;
mod data_model;
mod handler;
mod handlers;


#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Hello, world!");
}
