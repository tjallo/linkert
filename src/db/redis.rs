pub fn connect_redis() -> redis::Connection {
    let client_result = redis::Client::open("redis://127.0.0.1/");

    let connection_result = match client_result {
        Ok(client) => client.get_connection(),
        Err(error) => panic!("Problem connecting to redis {error:?}"),
    };

    match connection_result {
        Ok(client) => client,
        Err(error) => panic!("Problem connecting to redis {error:?}"),
    }
}
