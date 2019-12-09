extern crate session_types_extension;
 
use session_types_extension::mpst::{global, local, Message, PayloadType, Role}; 

use std::rc::Rc;

#[test]
fn new_role() {
	let alice = Role::new("Alice");
	assert!(String::from("Alice").eq(alice.name()));
	// Test alice.name() can be reused.
	assert!(String::from("Alice").eq(alice.name()));
}

#[test]
fn new_message() {
	let msg = Message::new("l");
	assert!(String::from("l").eq(msg.label()));
	assert_eq!(msg.to_string(), String::from("l()"));

	let msg_with_payloads = Message::with_payload_type("l", "int");
	assert!(String::from("l").eq(msg_with_payloads.label()));
	assert_eq!(msg_with_payloads.to_string(), String::from("l(int)"));
}

#[test]
fn example_global_type() {
	let sndr = Role::new("alice");
	let rcvr = Role::new("bob");
	let msg1 = Message::new("label1");
	let msg2 = Message::new("label2");

	let p1 = global::Type::interaction(&sndr, &rcvr);
	let p1_1 = global::Type::add_message(p1, msg1, global::Type::typevar("T"));
	let p1_2 = global::Type::add_message(p1_1, msg2, global::Type::end());
	let p = global::Type::recur("T", p1_2);

	match *p {
	    global::Type::Recur { ref t, ref g } => {
		assert_eq!(t, "T");
		match **g {
		    global::Type::Interact { ref p, ref q, ref g } => {
			assert!(Rc::ptr_eq(p, &sndr));
			assert!(Rc::ptr_eq(q, &rcvr));
			for (m_i, g_i) in g {
			    match m_i.label().as_str() {
				"label1" => match **g_i {
				    global::Type::TypeVar { ref t } => assert_eq!(t, "T"),
				    _ => assert!(false),
				},
				"label2" => match **g_i {
				    global::Type::End => assert!(true),
				    _ => assert!(false),
				},
				_ => assert!(false),
			    }
			}
		    }
		    _ => assert!(false),
		}
	    }
	    _ => assert!(false),
	}
}

#[test]
fn example_local_type() {
	let sndr = Role::new("alice");
	let rcvr = Role::new("bob");
	let msg1 = Message::with_payload_type("label", "int");
	let msg2 = Message::with_payload_type("label2", "int");
	let msg3 = Message::with_payload_type("label3", "");

	let sel2 = local::Type::select(&sndr);
	let sel2_1 = local::Type::add_message(sel2, msg2, local::Type::typevar("T"));
	let sel2_2 = local::Type::add_message(sel2_1, msg3, local::Type::end());
	let br1 = local::Type::branch(&rcvr);
	let br1_1 = local::Type::add_message(br1, msg1, sel2_2); // Recv
	let p = local::Type::recur("T", br1_1);

	match *p {
	    local::Type::Recur { ref t, ref s } => {
		assert_eq!(t, "T");
		match **s {
		    local::Type::Branch { ref p, ref s } => {
			assert!(Rc::ptr_eq(p, &rcvr));
			for (m_i, s_i) in s {
			    match m_i.label().as_str() {
				"label" => {
				    match **s_i {
					local::Type::Select { ref p, ref s } => {
					    assert!(Rc::ptr_eq(p, &sndr));
					    for (m_i, s_i) in s {
						match m_i.label().as_str() {
						    "label2" => match **s_i {
							local::Type::TypeVar{ ref t } => assert_eq!(t.as_str(), "T"),
							_ => assert!(false),
						    }
						    "label3" => match **s_i {
							local::Type::End => assert!(true),
							_ => assert!(false)
						    }
						    _ => assert!(false),
						}
					    }
					}
					_ => assert!(false),
				    }
				}
				_ => assert!(false),
			    }
			}
		    }
		    _ => assert!(false),
		}
	    }
	    _ => assert!(false),
	}
}

#[test]
fn example_session_passing() {
	let sndr = Role::new("alice");
	let rcvr = Role::new("bob");

	let p = global::Type::interaction(&sndr, &rcvr);
	let l = Message::with_payload_session("label", local::Type::recur("T", local::Type::add_message(local::Type::branch(&Role::new("a")), Message::with_payload_type("L", "T"), local::Type::end())));
	let p2 = global::Type::add_message(p, l, global::Type::end());

	match *p2 {
	    global::Type::Interact { ref p, ref q, ref g } => {
		assert_eq!(p.name(), sndr.name());
		assert_eq!(q.name(), rcvr.name());
		for (m_i, g_i) in g {
		    assert!(*m_i.label() == String::from("label"));
		    match m_i.payload {
			PayloadType::Session(ref s) => {
			    match **s {
				local::Type::Recur { ref t, ref s } => {
				    assert!(*t == String::from("T"));
				    match **s {
					local::Type::Branch { ref p, ref s } => {
					    assert_eq!(*p.name(), String::from("a"));
					    for (m_i, s_i) in s {
						assert!(*m_i.label() == String::from("L"));
						match m_i.payload {
						    PayloadType::BaseType(ref t) => assert!(*t == String::from("T")),
						    _ => assert!(false),
						}
						match **s_i {
						    local::Type::End => (),
						    _ => assert!(false),
						}
					    }
					},
					_ => assert!(false),
				    }
				},
				_ => assert!(false)
			    }
			},
			_ => assert!(false),
		    }
		    match **g_i {
			global::Type::End => (),
			_ => assert!(false),
		    }
		}
	    },
	    _ => assert!(false),
	}

	assert_eq!(p2.to_string(), "alice → bob:label(μT.a?L(T).end).end");
}
