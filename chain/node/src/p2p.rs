use std::net::{UdpSocket, ToSocketAddrs};
use std::thread;
use std::time::Duration;
use std::path::PathBuf;
use std::fs;

pub fn spawn(node_id: char, state_dir: PathBuf) {
    thread::spawn(move || {
        // each node listens on a unique local UDP port
        let port = match node_id {
            'A' => 9001,
            'B' => 9002,
            'C' => 9003,
            'D' => 9004,
            'E' => 9005,
            _ => 9010,
        };

        let bind_addr = format!("127.0.0.1:{}", port);
        let socket = UdpSocket::bind(&bind_addr).expect("bind failed");

        socket
            .set_nonblocking(true)
            .expect("failed to set nonblocking");

        println!("Node {} P2P listener on {}", node_id, bind_addr);

        let peers = vec![
            "127.0.0.1:9001",
            "127.0.0.1:9002",
            "127.0.0.1:9003",
            "127.0.0.1:9004",
            "127.0.0.1:9005",
        ];

        let mut buf = [0u8; 256];

        loop {
            // Broadcast a small heartbeat to peers
            for peer in &peers {
                if *peer == bind_addr {
                    continue;
                }

                if let Ok(mut iter) = peer.to_socket_addrs() {
                    if let Some(addr) = iter.next() {
                        let msg = format!("{{\"node\":\"{}\",\"ping\":true}}", node_id);
                        let _ = socket.send_to(msg.as_bytes(), addr);
                    }
                }
            }

            // Listen for incoming gossip messages
            if let Ok((n, src)) = socket.recv_from(&mut buf) {
                let msg = String::from_utf8_lossy(&buf[..n]);
                let path = state_dir.join("p2p_inbox.log");
                let _ = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&path)
                    .and_then(|mut f| {
                        use std::io::Write;
                        writeln!(f, "[{}] {}", src, msg)
                    });
            }

            thread::sleep(Duration::from_millis(1000));
        }
    });
}
