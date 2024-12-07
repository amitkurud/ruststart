use log::{debug, info};
use redis::Commands;
use std::env;

fn main() {
    env_logger::init();
    let names = "Hello world";
    let names2 = "Hello world once more timee";
    info!("Hello, world! {} {}", names, names2);
    if names2.len()>10 {
        info!("Hello, world!2 {} {}", names, names2);
    }
    let chars = names2.chars();
    chars.for_each(move |x| {
        debug!("Hello, world!2 {} {}", names, x);
    });
    for i in 0..4550 {
        let mut conn = connect();
        println!("******* Running SET commands {} *******", i);

        let set_name = "users";

        let _: () = conn
            .sadd(set_name, "user1".to_owned() + i.to_string().as_str())
            .expect("failed to execute SADD for 'users'");
    }
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
