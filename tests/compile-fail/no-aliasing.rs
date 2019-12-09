extern crate session_types_extension;

use std::thread::spawn;

use session_types_extension::*;

fn srv(c: Chan<(), Recv<u8, Eps>>) {
    let (c, _) = c.recv();
    c.close();
}

fn main() {
    let (c1, c2) = session_channel();
    let t1 = spawn(|| { srv(c2) });

    let c1_ = c1;
    c1_.send(42).close();
    c1.send(42).close();        //~ ERROR

    t1.join().unwrap();
}
