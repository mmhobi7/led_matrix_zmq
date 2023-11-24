#[derive(Clone, Debug)]
pub struct MatrixClientSettings {
    pub addr: String,
}

pub struct MatrixClient {
    pub opts: MatrixClientSettings,

    socket: zmq::Socket,
}

impl MatrixClient {
    pub fn new(opts: MatrixClientSettings) -> MatrixClient {
        let context = zmq::Context::new();
        let socket = context.socket(zmq::REQ).unwrap();
        socket.connect(&opts.addr).expect("Failed to connect to server!");

        MatrixClient {
            opts,
            socket
        }
    }

    pub fn send_frame(&self, frame: &[u8]) {
        let mut v: Vec<u8> = vec![0];
        v.extend_from_slice(frame);
        self.socket.send(v, 0).expect("Failed to send frame!");
        self.socket.recv_bytes(0).expect("Couldn't acknowledge sending frame!");
    }

    pub fn send_brightness(&self, brightness: u8) {
        let frame_encap: &[u8] = &[1, brightness];
        self.socket.send(frame_encap, 0).expect("Failed to send frame!");
        self.socket.recv_bytes(0).expect("Couldn't acknowledge sending frame!");
    }
}
