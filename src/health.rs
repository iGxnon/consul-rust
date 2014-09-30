#![allow(non_snake_case)]
use std::str::from_utf8;
use serialize::json;

use curl::http;
use structs::{Node, Service};


#[deriving(Decodable, Show)]
pub struct HealthService{
    Node: Node,
    Service: Service,
}


pub struct Health{
    address: &'static str,
}

impl Health {

    pub fn new(address: &'static str) -> Health {
        Health{address: address}
    }

    pub fn service(&self, name: &'static str) -> Vec<HealthService> {
        let url = self.address.to_string() + "/health/service/" + name;
        let resp = http::handle().get(url).exec().unwrap();
        let result = from_utf8(resp.get_body()).unwrap();
        json::decode(result).unwrap()
    }
}