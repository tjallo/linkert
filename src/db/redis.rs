use std::sync::{Arc, Mutex};

pub fn connect_redis(redis_url: &str) -> Arc<Mutex<redis::Connection>> {
    let client_result = redis::Client::open(redis_url);

    let connection_result = match client_result {
        Ok(client) => client.get_connection(),
        Err(error) => panic!("Problem connecting to redis {error:?}"),
    };

    let client = match connection_result {
        Ok(client) => client,
        Err(error) => panic!("Problem connecting to redis {error:?}"),
    };

    Arc::new(Mutex::new(client))
}
