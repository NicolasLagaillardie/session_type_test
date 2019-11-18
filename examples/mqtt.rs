#![cfg_attr(feature = "cargo-clippy", allow(ptr_arg))]
extern crate session_types;
use session_types::*;
use std::thread::spawn;

type Id = String;
type Server = Recv<Id, Choose<Rec<ServerInner>, Eps>>;

type ServerInner = Offer<ServerDeposit, Offer<ServerWithdraw, Offer<ServerBalance, Eps>>>;

type ServerDeposit = Recv<u64, Send<u64, Var<Z>>>;
type ServerWithdraw = Recv<u64, Choose<Var<Z>, Var<Z>>>;
type ServerBalance = Send<u64, Var<Z>>;

type Client = <Server as HasDual>::Dual;

fn approved(id: &Id) -> bool {
    !id.is_empty()
}

fn server(c: Chan<(), Server>) {
    loop {
	let mut c = {
        	let (c, id) = c.recv();
        	if !approved(&id) {
            		c.sel2().close();
            		return;
        	}
        	c.sel1().enter()
    	};
	
	let (server_chan, client_chan) = session_channel();
	
	spawn(|| server(server_chan));

	c.send(client_chan);

    	server_request_test(server_chan);

	c.close();
    }
}

fn server_request_test(c: Chan<(), Server>){
   	 let mut c = {
        	let (c, id) = c.recv();
        	if !approved(&id) {
            		c.sel2().close();
            		return;
        	}
        	c.sel1().enter()
    	};
	
	let (c, server) = c.send("Test?".to_string());
	let (c, results) = c.recv();

	println!("Results: {}", results);

	c.zero().close();
}

fn client(c: Chan<(), Client>) {
	let c = match c.send("New client".to_string()).offer() {
		Left(c) => c.enter(),
		Right(_) => panic!("New client: expected to be approved"),
	};

	let (c, new_chan) = c.recv();

	client_send_test(new_chan);

	c.close();
}

fn client_send_test(c: Chan<(), Client>){
	let c = match c.send("Recognized client".to_string()).offer() {
        	Left(c) => c.enter(),
        	Right(_) => panic!("Recognized client: expected to be approved"),
    	};

	let (c, payload) = c.recv();

	if payload == "Test?".to_string() {
		c.send("coco".to_string()).zero().close();
	}
	
}

fn main() {
    let (server_chan, client_chan) = session_channel();
    spawn(|| server(server_chan));
    client(client_chan);
}
