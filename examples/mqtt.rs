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
	let mut c = {
        	let (c, id) = c.recv();
        	if !approved(&id) {
            		c.sel2().close();
            		return;
        	}
        	c.sel1().enter()
    	};

	let mut balance = 0;
	loop {
        	c = offer! {
            		c,
           		 Results => {
				let (c, connected) = c.recv();
				println!("Connected: {}", connected);
				let (c, results) = c.send("test".to_string()).recv();
				println!("Results: {}", results);
				c.send("Complete".to_string()).zero();
//                		let (c, server) = c.recv();
//                		balance += server;
//                		c.send(balance).zero()
            		},
            		Blank1 => {
                		let (c, amt) = c.recv();
                		if amt > balance {
                    			c.sel2().zero()
                		} else {
                    			balance -= amt;
                    			c.sel1().zero()
                		}
            		},
            		Blank2 => {
                		c.send(balance).zero()
            		},
            		Quit => {
                		c.close();
                		break
            		}
        	}
	}
}

fn client(c: Chan<(), Client>) {
	let c = match c.send("Client".to_string()).offer() {
        	Left(c) => c.enter(),
        	Right(_) => panic!("Client: expected to be approved"),
    	};

	let (c, request) = c.sel1().send("Connected".to_string()).recv();

	println!("Request: {}", request);

	c.send("GTX 1060".to_string()).zero().skip3().close();

}

//fn withdraw_client(c: Chan<(), Client>) {
//	let c = match c.send("Withdraw Client".to_string()).offer() {
//        	Left(c) => c.enter(),
//        	Right(_) => panic!("withdraw_client: expected to be approved"),
//    	};
//
//	let (c, new_balance) = c.sel1().send(200).recv();
//    	println!("deposit_client: new balance: {}", new_balance);
//
//    	match c.zero().sel2().sel1().send(100).offer() {
//        	Left(c) => {
//            		println!("withdraw_client: Successfully withdrew 100");
//            		c.zero().skip3().close();
//        	}
//        	Right(c) => {
//            		println!("withdraw_client: Could not withdraw. Depositing instead.");
//            		c.zero().sel1().send(50).recv().0.zero().skip3().close();
//        	}
//    	}
//}


fn main() {
    let (server_chan, client_chan) = session_channel();
    spawn(|| server(server_chan));
    client(client_chan);
}
