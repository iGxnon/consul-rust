## consul-rust (Incomplete implement)

Rust client libray for Consul HTTP API

### Usage

    extern crate consul;

    use std::collections::HashMap;

    fn main(){
        let catalog1 = consul::catalog::Catalog::new("http://localhost:8500/v1");
        let services: HashMap<String, Vec<String>> = catalog1.services();
        println!("{}", services);
    }


For more example, see the **[tests](https://github.com/youngking/consul-rust/blob/master/src/test/basic.rs)** .

### Installation

Simply include the consul-rust in your Cargo dependencies.

    [dependencies.consul]
    git = "https://github.com/youngking/consul-rust"