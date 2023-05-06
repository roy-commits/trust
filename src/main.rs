use etherparse::{Ipv4HeaderSlice, ReadError};
use std::io;

fn main() -> io::Result<()> {
    let face = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun).expect("failed to initialize tun0");
    let mut buf = [0u8; 1504];
    loop {
        let bytes = face.recv(&mut buf[..])?;
        // let flags = u16::from_ne_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_ne_bytes([buf[2], buf[3]]);
        if eth_proto != 0x0800 {
            // not ipv4
            break;
        }
        match etherparse::TcpHeaderSlice::from_slice(&buf[4..bytes]) {
            Ok(packet) => {
                let src = packet.source_addr();
                let dst = packet.destination_addr();
                let proto = packet.protocol();
                eprintln!(
                    "{} -> {} {}b tcp to port {}",
                    src,
                    dst,
                    packet.slice().len(),
                    packet.destination_port()
                );
            }
            Err(e) => {
                eprintln!("ignoring weird tcp packet: {:?}", e);
            }
        }
        eprintln!("recv: {} bytes: {:?}", recv - 4, &buf[4..recv]);
    }
    Ok(())
}
