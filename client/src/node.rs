use mainline::Dht;

pub fn dht_client() -> Dht {
    return Dht::server().unwrap()
}
