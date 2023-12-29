use std::net::{Ipv6Addr, SocketAddrV6};

use cargo_http_registry::serve;

fn main() -> anyhow::Result<()> {
    serve("/home/bzm3r/rse/playground/test/", "127.0.0.1")
}
