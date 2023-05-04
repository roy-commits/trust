use std::io;

fn main() -> io::Result<()> {
    let face = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun).expect("failed to initialize tun0");
    let mut buf = [0u8; 1504];
    let recv = face.recv(&mut buf[..])?;
    eprintln!("recv: {} bytes: {:?}", recv, &buf[..recv]);
    Ok(())
}
