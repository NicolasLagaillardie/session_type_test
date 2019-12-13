extern crate session_types_extension;

use session_types_extension::mpst::{global, local, Message, PayloadType, Role};

use std::rc::Rc;

fn main(){
	let client = Role::new("Client");
	let server = Role::new("Server");

	let p = global::Type::interaction(&client, &server);

	let l = Message::with_payload_session("label", local::Type::recur("T", local::Type::add_message(local::Type::branch(&Role::new("a")), Message::with_payload_type("L", "T"), local::Type::end())));

	let p2 = global::Type::add_message(p, l, global::Type::end());

        match *p2 {
            global::Type::Interact { ref p, ref q, ref g } => {
                assert_eq!(p.name(), client.name());
                assert_eq!(q.name(), server.name());
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


	println!("Protocol: {}", p2.to_string());
}
