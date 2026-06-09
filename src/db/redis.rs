pub fn connect_redis(redis_url: &str) -> redis::Connection {
    let client_result = redis::Client::open(redis_url);

    let connection_result = match client_result {
        Ok(client) => client.get_connection(),
        Err(error) => panic!("Problem connecting to redis {error:?}"),
    };

    match connection_result {
        Ok(client) => client,
        Err(error) => panic!("Problem connecting to redis {error:?}"),
    }
}
