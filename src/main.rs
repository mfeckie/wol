use clap::{App, Arg};
use hex;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let matches = App::new("Wake On Lan helper")
        .version("0.0.1")
        .author("Martin Feckie <martin@mfeckie.dev>")
        .arg(
            Arg::new("mac_address")
                .about("Mac Address of device you wish to wake. e.g. `00:01:FF:CC:09:22")
                .index(1)
                .required(true),
        )
        .get_matches();

    if let Some(ref string_address) = matches.value_of("mac_address") {
        let mac_bytes = string_mac_to_bytes(string_address);
        let mut magic_packet: Vec<u8> = vec![0xFF; 6];

        for _ in 0..16 {
            magic_packet.extend_from_slice(&mac_bytes[..]);
        }

        let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind to local address");
        socket.set_broadcast(true)?;

        match socket.send_to(&magic_packet, "255.255.255.255:9") {
            Ok(_) => {
                println!("Magic packet sent to MAC: {}", string_address);
            }
            Err(msg) => {
                println!("Failed to send {:?}", msg);
            }
        }
    }

    Ok(())
}

fn string_mac_to_bytes(incoming: &str) -> Vec<u8> {
        incoming
        .split(":")
        .flat_map(|pair| hex::decode(pair).expect("MAC address contains unexpected characters"))
        .collect()
}
