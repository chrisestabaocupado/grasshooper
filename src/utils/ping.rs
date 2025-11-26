use std::net::ToSocketAddrs;
use ping;


pub fn get_discordapi_ping() -> String {
    let address = "discord.com:0"
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap()
        .ip(); // convert to IP;

    match ping::new(address).send() {
        Ok(result) => return format!("{}ms", result.payload[0].to_string()),
        Err(e) => return format!("Failed: {}", e),
    }
}