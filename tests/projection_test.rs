extern crate session_types_extension;

use session_types_extension::mpst::*;

use session_types_extension::mpst::projection::project;

use std::rc::Rc;

#[test]
fn test_projection() {
	let sndr = Role::new("alice");
	let rcvr = Role::new("bob");
	let msg1 = Message::new("label1");
	let msg2 = Message::new("label2");

	let p1 = global::Type::interaction(&sndr, &rcvr);
	let p1_1 = global::Type::add_message(p1, msg1, global::Type::typevar("T"));
	let p1_2 = global::Type::add_message(p1_1, msg2, global::Type::end());
	let p0 = global::Type::recur("T", p1_2);

	let local_alice = project(&p0, &sndr);
	let local_bob = project(&p0, &rcvr);

	match local_alice {
	    Some(ref t) => {
		match **t {
		    local::Type::Recur { ref t, ref s } => {
			assert_eq!(t, "T");
			match **s {
			    local::Type::Select { ref p, ref s } => {
				assert!(Rc::ptr_eq(p, &rcvr));
				for (m_i, s_i) in s {
				    match m_i.label().as_str() {
					"label1" => {
					    match **s_i {
						local::Type::TypeVar { ref t } => {
						    assert_eq!(t, "T")
						}
						_ => assert!(false),
					    }
					}
					"label2" => {
					    match **s_i {
						local::Type::End => assert!(true),
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
	    None => assert!(false),
	};

	match local_bob {
	    Some(ref t) => {
		match **t {
		    local::Type::Recur { ref t, ref s } => {
			assert_eq!(t, "T");
			match **s {
			    local::Type::Branch { ref p, ref s } => {
				assert!(Rc::ptr_eq(p, &sndr));
				for (m_i, s_i) in s {
				    match m_i.label().as_str() {
					"label1" => {
					    match **s_i {
						local::Type::TypeVar { ref t } => {
						    assert_eq!(t, "T")
						}
						_ => assert!(false),
					    }
					}
					"label2" => {
					    match **s_i {
						local::Type::End => assert!(true),
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
	    None => assert!(false),
	}
}

#[test]
fn test_project_mergefail() {
	let p = Role::new("P");
	let q = Role::new("Q");
	let r = Role::new("R");
	let a = Message::new("a");
	let b = Message::new("b");
	let a2_1 = Message::new("a2");
	let a2_2 = Message::new("a2");
	let a3 = Message::new("a3");

	let p3_1 =
	    global::Type::add_message(global::Type::interaction(&r, &p), a3, global::Type::end());
	let p2_1 = global::Type::add_message(global::Type::interaction(&q, &r), a2_1, p3_1);
	let p2_2 =
	    global::Type::add_message(global::Type::interaction(&q, &r), a2_2, global::Type::end());
	let p1 = global::Type::interaction(&p, &q);
	let p1_1 = global::Type::add_message(p1, a, p2_1);
	let p1_2 = global::Type::add_message(p1_1, b, p2_2);

	let local_r = projection::project(&p1_2, &r);
	// This should return None because merge is not possible.
	match local_r {
	    Some(_) => assert!(false),
	    None    => (),
	}
}
