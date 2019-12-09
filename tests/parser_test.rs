extern crate session_types_extension;

use session_types_extension::mpst::global;

use session_types_extension::mpst::projection::project;

use session_types_extension::mpst::parser::{parse_global_type, parse_local_type};

#[test]
fn test_parse_global() {
	let g = parse_global_type(String::from("* T .A->B:{ l(int).T}"));
	assert_eq!(g.unwrap().0.to_string(), "μT.A → B:l(int).T");
}

#[test]
fn test_parse_local() {
	let l = parse_local_type(String::from("*T .A&{?l(int).T}"));
	assert_eq!(l.unwrap().0.to_string(), "μT.A?l(int).T");
}

#[test]
fn test_parse_project() {
	let g = parse_global_type(String::from("* T .A->B:{ l(int).B->A:{ l2().T } }"));
	let (global_type, registry) = g.unwrap();
	let l = project(&global_type, &registry.find_role_str("A").unwrap());
	assert_eq!(l.unwrap().to_string(), "μT.B!l(int).B?l2().T");
}

#[test]
fn test_parse_ambiguous() {
	let g = parse_global_type(String::from("*T.A"));
	match g {
	    Some(_) => assert!(false), // Not expecting this to parse
	    None => (),
	}
}

#[test]
fn test_parse_ambiguous2() {
	let g = parse_global_type(String::from("*T.end"));
	match g {
	    Some(parsed) => {
		let (t, _reg) = parsed;
		match *t {
		    global::Type::Recur { ref t, ref g } => {
			assert_eq!(*t, String::from("T"));
			match **g {
			    global::Type::End => (),
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
fn test_parse_ambiguous3() {
	let g = parse_global_type(String::from("*T.T"));
	match g {
	    Some(parsed) => {
		let (t, _reg) = parsed;
		match *t {
		    global::Type::Recur { ref t, ref g } => {
			assert_eq!(*t, String::from("T"));
			match **g {
			    global::Type::TypeVar { ref t } => assert_eq!(*t, String::from("T")),
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
fn test_parse_ambiguous4() {
	let g = parse_global_type(String::from("*T.A->B: { l().end }"));
	match g {
	    Some(parsed) => {
		let (t, reg) = parsed;
		match *t {
		    global::Type::Recur { ref t, ref g } => {
			assert_eq!(*t, String::from("T"));
			match **g {
			    global::Type::Interact {
				ref p,
				ref q,
				ref g,
			    } => {
				assert_eq!(reg.find_role_str("A").unwrap().name(), p.name());
				assert_eq!(reg.find_role_str("B").unwrap().name(), q.name());
				for (m, g) in g {
				    assert_eq!(*m.label(), String::from("l"));
				    match **g {
					global::Type::End => (),
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
fn test_equality() {
	//
	// Test that the two processes are equal despite the ordering.
	//
	let l1 = parse_local_type(String::from("A&{ ?l1(int).end, ?l2( *T.B!().*T.T ).end}"));
	let (l1_type, _l1_reg) = l1.unwrap();
	let l2 = parse_local_type(String::from("A&{ ?l2(*T.B!().*T.T).end, ?l1(int).end}"));
	let (l2_type, _l2_reg) = l2.unwrap();

	assert_eq!(l1_type, l2_type);
}

#[test]
fn test_deep_equality() {
	//
	// Test that the two processes are equal despite the ordering.
	//
	let l1 = parse_local_type(String::from("A&{ ?l1(int).end, ?l2( *T.B!().*T.T ).end}"));
	let (l1_type, _l1_reg) = l1.unwrap();
	let l2 = parse_local_type(String::from("A&{ ?l2(*T.B!().*T.T).end, ?l1(int).end}"));
	let (l2_type, _l2_reg) = l2.unwrap();

	assert!(!l1_type.deep_eq(&l2_type));
}

