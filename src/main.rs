use crate::redis::db::{call_redis_sadd, connect};
use log::{debug, info};
use std::collections::{linked_list, HashSet};
mod redis;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut mrarr = vec!["Hello", "world", "from", "Rust", "and", "Tokio"];
    mrarr.append(&mut vec!["Hello", "world", "from", "Rust", "and", "Tokio"]);
    mrarr.iter().for_each(|x| {
        info!("Hello, world! {}", x);
    });
    mrarr.push("Hello");
    for n in mrarr {
        info!("Hello, world! {}", n);
    }
    let hello_world = "Hello world";
    let hello_world_once_more = "Hello world once more timee";
    info!("{}", hello_world_once_more);
    info!("Hello, world! {} {}", hello_world, hello_world_once_more);
    if hello_world_once_more.len() > 10 {
        info!("Hello, world!2 {} {}", hello_world, hello_world_once_more);
    }
    let chars = hello_world_once_more.chars();
    chars.for_each(move |x| {
        debug!("Hello, world! {} {}", hello_world, x.to_uppercase());
    });
    let usu = hello_world.parse::<String>().unwrap();
    info!("{}", usu);
    let kaiku = "Hello_word wehe weh22www 2sdhshdww"
        .replace("w", "2")
        .to_string();

    //createa linked list and push elements
    let mut list = linked_list::LinkedList::new();
    list.push_back("Hello");
    list.push_back("world");

    let mut string_set = HashSet::new();

    string_set.insert("Hello".to_string());
    string_set.iter().for_each(|x| {
        x.chars().for_each(|y| {
            debug!("set of y {}", y);
        });
    });

    list.iter().for_each(|x| {
        x.chars().for_each(|y| {
            debug!("list of y {}", y);
        });
        info!("list of {}", x);
    });

    info!("{}", kaiku);
    let mut conn = connect();
    for i in 0..4 {
        call_redis_sadd(i, &mut conn).await;
    }
}
