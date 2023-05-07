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
        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..bytes]) {
            Ok(p) => {
                let src = p.source_addr();
                let dst = p.destination_addr();
                let proto = p.protocol();
                if proto != 0x06 {
                    // not tcp protocol
                    break;
                }
                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + p.slice().len()..]) {
                    Ok(p) => {
                        eprintln!(
                            "{} -> {} {}b of tcp to port: {}",
                            src,
                            dst,
                            p.slice().len(),
                            p.destination_port()
                        );
                    }
                    Err(e) => {
                        eprintln!("ignoring weird tcp packet: {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("ignoring weird packet: {:?}", e);
            }
        }
    }
    Ok(())
}
