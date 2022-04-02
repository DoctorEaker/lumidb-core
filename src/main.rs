mod modules;

use jsonrpc_core::*;
use jsonrpc_http_server::*;
use dotenv::dotenv;
use std::env;
//use crate::modules::*;
use crate::modules::db::*;

fn main() {
    dotenv().ok();

    let ip = env::var("IP").unwrap().to_owned();
    let port = env::var("PORT").unwrap().to_owned();
    let name = env::var("INSTANCE_NAME");
    let url = format!("{}:{}",ip,port);

    let mut io = IoHandler::new();
    io.add_sync_method("say_hello", move |_: Params| {
        Ok(Value::String(name.clone().unwrap()))
    });

    let mut db = DB::new();

    db.set(&"magic".to_string(), "magic_value".to_string());
    println!("mv");
    db.set(&"magic".to_string(), "magic2".to_string());
    println!("m2");

    db.delete_at_index(&"magic".to_string(),0);

    let _response = db.serialize(&"magic".to_string());
    //println!("{}",response);

    let _server = ServerBuilder::new(io)
        .start_http(&url.parse().unwrap())
        .expect("Unable to start RPC server");

    _server.wait();
}
