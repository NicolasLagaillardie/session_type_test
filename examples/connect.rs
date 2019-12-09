extern crate session_types_extension;
use session_types_extension::*;

fn server(c: Chan<(), Eps>) {
    c.close()
}

fn client(c: Chan<(), Eps>) {
    c.close()
}

fn main() {
    connect(server, client);
}
