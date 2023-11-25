#[derive(Clone, Debug)]
pub struct MatrixClientSettings {
    pub addrs: Vec<String>,
}

pub struct MatrixClient {
    pub opts: MatrixClientSettings,
    pub sockets: Vec<zmq::Socket>,
}

impl MatrixClient {
    pub fn new(opts: MatrixClientSettings) -> MatrixClient {
        let context = zmq::Context::new();
        let mut sockets: Vec<zmq::Socket> = Vec::new();
        for addr in &opts.addrs {
            let new_socket = context.socket(zmq::REQ).unwrap();
            new_socket
                .connect(addr)
                .expect("Failed to connect to server!");
            sockets.push(new_socket)
        }

        MatrixClient { opts, sockets }
    }

    pub fn send_frame(&self, frame: &[u8]) {
        let mut v: Vec<u8> = frame.to_vec();
        v.extend(vec![0]); // ID 0
        for socket in &self.sockets {
            socket.send(&v, 0).expect("Failed to send frame!");
            socket
                .recv_bytes(0)
                .expect("Couldn't acknowledge sending frame!");
        }
    }

    pub fn send_brightness(&self, brightness: u8) {
        let frame_encap: &[u8] = &[brightness, 1]; // ID 1
        for socket in &self.sockets {
            socket.send(frame_encap, 0).expect("Failed to send frame!");
            socket.recv_bytes(0).expect("Couldn't acknowledge sending frame!");
        }
    }
}
