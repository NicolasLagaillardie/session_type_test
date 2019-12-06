////#![cfg_attr(feature = "cargo-clippy", allow(ptr_arg))]
////extern crate session_types;
////use session_types::*;
////use std::thread::spawn;
////
////type Id = String;
//////type Server = Recv<Id, Choose<Recv<Chan<(), Testing>, Eps>, Eps>>;
//////type ServerInner = Recv<String, Send<Chan<(), Testing>, Var<Z>>>;
////type Server = Recv< Id, Choose< Recv< Chan< (), Testing>, Send< Chan< (), Testing>, Eps>>, Eps>>;
////
//////type Server = Send<Chan<(), Testing>, Var<Z>>;
////type Testing = Recv<String, Eps>;
////
////type Client = Send< Id, Offer< Recv< Chan< (), ClientResults>, Eps>, Eps>>;
////type ClientResults = <Testing as HasDual>::Dual;
////
////fn approved(id: &Id) -> bool {
////    !id.is_empty()
////}
////
////fn create_new_channel(tx, rx) {
////	let mut ts = vec![];
////
////	let (c1, c2) = session_channel();
////
////	ts.push(spawn(move || {
////		client_results(c2);
////	}));
////
////	tx = tx.send(c1).close();
////
////	server_handling(rx);
////
////	ts[0].join();
////}
////
////fn server(c: Chan<(), Server>) {
////	let mut c = {
////        	let (c, id) = c.recv();
////        	if !approved(&id) {
////            		c.sel2().close();
////            		return;
////        	}
////        	c.sel1()
////
////    	};
////
////	println!("First step");
////	let (c, new_client) = session_channel();
////	c.send(new_client);
////	spawn(move || server_handling(c));
////	c.close();
////}
////
////fn server_handling(c: Chan<(), Testing>) {
////
////	let (c, result) = c.recv();
////	
////	println!("Success: {}", result);
////
////	c.close();
////} 
////
////fn client_basic(c: Chan<(), Client>) {
////	let c = match c.send("Client".to_string()).offer() {
////        	Left(c) => c,
////        	Right(_) => panic!("Client: expected to be approved"),
////    	};
////
////	println!("Accepted");
////
////	let (c, new_chan) = c.recv();
////
////	client_results(new_chan);
////}
////
////fn client_results(c: Chan<(), ClientResults>) {
////	c.send("Test".to_string()).close();
////}
////
////fn main() {
////    	let (server_chan, client_chan) = session_channel();
////    	spawn(|| client_basic(client_chan));
////	server(server_chan);
////}
//
//
//#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
//extern crate rand;
//extern crate session_types;
//
//use session_types::*;
//use std::thread::spawn;
//use rand::random;
//
//type Id = String;
//type Server = Recv<u8, Choose<Send<u8, Eps>, Eps>>;
//type Client = <Server as HasDual>::Dual;
//
//fn server_handler(c: Chan<(), Server>) {
//    let (c, n) = c.recv();
//    match n.checked_add(42) {
//        Some(n) => c.sel1().send(n).close(),
//        None => c.sel2().close(),
//    }
//}
//
///// A channel on which we will receive channels
/////
//type ChanChan = Offer<
//			Eps,
//			Recv<
//				Id,
//				Choose<
//					Recv<
//						Chan<
//							(),
//							Server
//						>,
//						Var<Z>
//					>,
//					Send<
//						String,
//						Var<Z>
//					>
//				>
//			>
//		>;
//
//fn approved(id: &Id) -> bool {
//    !id.is_empty()
//}
//
///// server sits in a loop accepting session-typed channels. For each received channel, a new thread
///// is spawned to handle it.
/////
///// When the server is asked to quit, it returns how many connections were handled
//fn server(rx: Chan<(), Rec<ChanChan>>) -> usize {
//    
//	let mut c = rx.enter();
//	let mut count = 0;
//	loop {
//		c = offer! { c,
//			Quit => {
//				c.close();
//				break
//			},
//			NewChan => {
//				let (c, id) = c.recv();
//				if !approved(&id) {
//					c.sel2().send("Refused".to_string()).zero()
//				} else {
//					let (c, new_chan)= c.sel1().recv();
//					spawn(move || server_handler(new_chan));
//					count += 1;
//					c.zero()
//				}
//			}
//		}
//    	}
//	count
//}
//
//fn client_handler(c: Chan<(), Client>) {
//    let n = random();
//    match c.send(n).offer() {
//        Left(c) => {
//            let (c, n2) = c.recv();
//            c.close();
//            println!("{} + 42 = {}", n, n2);
//        }
//        Right(c) => {
//            c.close();
//            println!("{} + 42 is an overflow :(", n);
//        }
//    }
//}
//
//fn main() {
//    let (tx, rx) = session_channel();
//
//    let n: u8 = random();
//    let mut tx = tx.enter();
//
//    println!("Spawning {} clients", n);
//    let mut ts = vec![];
//    for _ in 0..n {
//        let (c1, c2) = session_channel();
//        ts.push(spawn(move || {
//            client_handler(c2);
//        }));
//
//	tx = match tx.sel2().send("Test Client".to_string()).offer() {
//        	Left(tx) => tx.send(c1).zero(),
//        	Right(_) => panic!("test_client: expected to be approved"),
//	};
//    }
//    tx.sel1().close();
//    let count = server(rx);
//    for t in ts {
//        let _ = t.join();
//    }
//    println!("Handled {} connections", count);
//}

fn main(){
}
