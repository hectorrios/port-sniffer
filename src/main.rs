use bpaf::Bpaf;
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc::{channel, Sender};
use tokio::net::TcpStream;
use tokio::task;

const MAX: u16 = 65535;
const IPFALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0,0,1));

#[derive(Debug, Clone, Bpaf)]
pub struct Arguments {
    #[bpaf(long, short, argument("Address"), fallback(IPFALLBACK))]
    pub address: IpAddr,
    #[bpaf(long("start"), short('s'), guard(start_port_guard, "Must be greater than 0"), fallback(1u16))]
    pub start_port: u16,
    #[bpaf(long("end"), short('e'), guard(end_port_guard, "Must be less than or equal to 65535"), fallback(MAX))]
    pub end_port: u16,
}



fn main() {
    println!("Hello, world!");
}
