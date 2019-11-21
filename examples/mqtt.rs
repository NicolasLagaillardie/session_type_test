#![cfg_attr(feature = "cargo-clippy", allow(ptr_arg))]
extern crate session_types;
use session_types::*;
use std::thread::spawn;

type Id = String;
//type Server = Recv<Id, Choose<Rec<ServerInner>, Eps>>;
//type ServerInner = Recv<String, Send<Chan<(), Testing>, Var<Z>>>;

type Server = Send<Chan<(), U>, Var<Z>>;
type Testing = Recv<String, Eps>;

//type Testing = Recv<Id, Choose<Rec<TestingInner>, Eps>>;
//
//type TestingInner = Offer<Phone, Offer<Computer, Eps>>;
//
//type Phone = Recv<String, Send<String, Var<Z>>>;
//type Computer = Recv<String, Send<String, Var<Z>>>;

type Client = <Server as HasDual>::Dual;
type ClientApproved = <Testing as HasDual>::Dual;

fn approved(id: &Id) -> bool {
    !id.is_empty()
}

fn server(c: Chan<(), Server>) {
//	let mut c = {
//        	let (c, id) = c.recv();
//        	if !approved(&id) {
//            		c.sel2().close();
//            		return;
//        	}
//        	c.sel1().enter()
//    	};

	let (new_server, new_client) = session_channel();

	spawn(|| server_testing(new_server));

	c.send(new_client);
}

fn server_testing(c: Chan<(), Testing>) {

	let (c, result) = c.recv();
	
	println!("Success: {}", result);

	c.close();

//	let mut c = {
//        	let (c, id) = c.recv();
//        	if !approved(&id) {
//            		c.sel2().close();
//            		return;
//        	}
//        	c.sel1().enter()
//    	};
//
//       	c = offer! {
//		c,
//          	Phone => {
//			let (c, device) = c.recv();
//			println!("Name: {}", device); 
//            		c.send("Complete".to_string()).zero()
//		},
//            	Computer => {
//			let (c, device) = c.recv();
//			println!("Name: {}", device); 
//            		c.send("Complete".to_string()).zero()			
//           	},
//            	Quit => {
//                	c.close();
//            	}
//	}      
} 

fn client_basic(c: Chan<(), Client>) {
//	let c = match c.send("Client".to_string()).offer() {
//        	Left(c) => c.enter(),
//        	Right(_) => panic!("Client: expected to be approved"),
//    	};

	let (c, new_channel) = c.recv();

	client_phone(new_channel);
}

fn client_phone(c: Chan<(), ClientApproved>) {
	c.send("True".to_string()).close();

//	let c = match c.send("Client Phone".to_string()).offer() {
//        	Left(c) => c.enter(),
//        	Right(_) => panic!("Client Phone: expected to be approved"),
//    	};
//
//	let (c, complete) = c.sel1().send("Phone".to_string()).recv();
//	
//	if complete == "Complete".to_string() {
//		c.zero().skip2().close();
//	}
}

fn main() {
    let (server_chan, client_chan) = session_channel();
    spawn(|| server(server_chan));
    client_basic(client_chan);
}
