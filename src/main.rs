use env_logger::Env;
use log::{debug, error, info, trace, warn};
use std::env;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

fn resolve_first_address(a: String) -> SocketAddr {
    match a
        .to_socket_addrs()
        .expect(format!("{:} could not be resolved", a).as_str())
        .as_slice()
    {
        [first, ..] => *first,
        _ => panic!("no address resolved for {:}", a),
    }
}

fn main() -> std::io::Result<()> {
    {
        const BUFFER_SIZE: usize = 65507;

        let env = Env::default().filter_or("LOG_LEVEL", "trace");
        env_logger::init_from_env(env);

        let args: Vec<String> = env::args().collect();
        info!("args: {:?}", args);

        match &args[..] {
            [_p, port, target] => {
                let receiver_address: SocketAddr =
                    resolve_first_address(format!("{:}:{:}", target, port));
                let socket: UdpSocket = UdpSocket::bind(format!("0.0.0.0:{:}", port))
                    .expect("could not bind to address");

                let mut buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
                let mut max: usize = 0;

                info!("Bound to socket: {:?}", socket);
                info!("Sending to {:}", receiver_address);

                loop {
                    let (len, _addr): (usize, SocketAddr) =
                        socket.recv_from(&mut buf).expect("could not receive");
                    if len > max {
                        max = len;
                        info!("max message size: {:?} bytes", max)
                    }

                    let out: &mut [u8] = &mut buf[..len];
                    match socket.send_to(&out, receiver_address) {
                        Ok(_) => debug!("{:}", String::from_utf8(out.to_vec()).unwrap()),
                        Err(e) => error!("{}", e)
                    };
                }
            }
            _ => panic!("invalid arguments -- port target"),
        }
    }
}
