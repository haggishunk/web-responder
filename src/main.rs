use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio;
use warp::Filter;

#[tokio::main]
async fn main() {
    let route = warp::any().map(|| "Static response");
    let server = warp::serve(route);
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000);
    server.run(socket_addr).await;
}
