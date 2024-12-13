use log::{debug, info};
use redis::Commands;
use std::{env, ops::Add};

#[tokio::main]
async fn main() {
    env_logger::init();
    let hello_world = "Hello world";
    let hello_world_once_more = "Hello world once more timee";
    info!("Hello, world! {} {}", hello_world, hello_world_once_more);
    if hello_world_once_more.len() > 10 {
        info!("Hello, world!2 {} {}", hello_world, hello_world_once_more);
    }
    let chars = hello_world_once_more.chars();
    chars.for_each(move |x| {
        debug!("Hello, world! {} {}", hello_world, x);
    });

    let mut conn = connect();
    for i in 0..4550 {
        call_redis_sadd(i, &mut conn).await;
    }
}

async fn call_redis_sadd(i: i32, conn: &mut redis::Connection) {
    println!("******* Running SET commands  ******* {} ", i);

    let _: () = conn.set(i.to_string() + "_key223213", i.to_string() + "").expect("Do not make dumb mistake again");
    let set_name = "users";

    let _: () = conn
        .sadd(set_name, "user1".to_owned() + i.to_string().as_str())
        .expect("failed to execute SADD for 'users'");
}

fn connect() -> redis::Connection {
    //format - host:port
    let redis_host_name =
        env::var("REDIS_HOSTNAME").expect("missing environment variable REDIS_HOSTNAME");
    let redis_password = env::var("REDIS_PASSWORD").unwrap_or_default();

    //if Redis server needs secure connection
    let uri_scheme = match env::var("IS_TLS") {
        Ok(_) => "rediss",
        Err(_) => "redis",
    };

    let redis_conn_url = format!("{}://:{}@{}", uri_scheme, redis_password, redis_host_name);
    //println!("{}", redis_conn_url);
    redis::Client::open(redis_conn_url.as_str())
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}
