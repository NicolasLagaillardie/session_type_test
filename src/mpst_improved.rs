use std::{marker, mem, ptr};
use std::thread::spawn;
use std::marker::PhantomData;

use std::cmp::Eq;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::string::ToString;

#[derive(Debug)]
pub enum PayloadType<A> {
	Data(PhantomData<A>),
	Empty,
}

impl<A> PartialEq for PayloadType<A> {
	fn eq(&self, other: &PayloadType<A>) -> bool {
		match (self, other) {
			(&PayloadType::Empty,	      &PayloadType::Empty)         => true,
			(&PayloadType::Data(ref s1),  &PayloadType::Data(ref s2))  => s1 == s2,
			_ => false,
		}
	}
}

#[derive(Debug)]
pub struct Message<A> {
	pub label: String, // label of the message, can be empty
	pub payload: PayloadType<A>, // (optional) payload types string
}

impl<A> Message<A> {
	pub fn new(label: &str) -> Message<A> {
		Message {
			label: String::from(label),
			payload: PayloadType::Empty,
		}
	}

	pub fn label(&self) -> &String {
		&self.label
	}

	pub fn with_payload_data(label: &str, payload: PhantomData<A>) -> Message<A> {
		Message {
			label: String::from(label),
			payload: PayloadType::Data(payload),
		}
	}
}

impl<A> Clone for Message<A> {
	/// Clone a Message
	fn clone(&self) -> Message<A> {
		match self.payload {
			PayloadType::Data(ref t) => Message {
				label: self.label.clone(),
				payload: PayloadType::Data(t.clone()),
			},
			PayloadType::Empty => Message {
				label: self.label.clone(),
				payload: PayloadType::Empty,
			},
		}
	}
}

impl<A> ToString for Message<A> {
	/// Converts `Message` into a human friendly `String` representation.
	///
	/// The format of the `String` representation is `Label(payload)`
	///
	fn to_string(&self) -> String {
		let payload = match self.payload {
			PayloadType::Data(ref t) => String::from("PhantomData"),
			PayloadType::Empty => String::from("Empty data"),
		};

		format!(
			"{label}({payload})",
			label = self.label.clone(),
			payload = payload
		)
	}
}

impl<A> PartialEq for Message<A> {
	fn eq(&self, other: &Message<A>) -> bool {
		self.label == other.label &&
		self.payload == other.payload
	}
}

impl<A> Eq for Message<A> {}

impl<A> Hash for Message<A> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.label.hash(state);
	}
}

/// A participant of a multiparty session.
///
/// The `Role` type represents an endpoint participant of a multiparty
/// session, and the `name` is used for uniquely identifying the
/// participant within the session.
///
/// Typical usage of a `Role` is to create once in a session, and reuse
/// the same `Role` variable in the session.
///
#[derive(Debug)]
pub struct Role {
	name: String,
}

impl Role {
	pub fn new(name: &str) -> Rc<Role> {
		Rc::new(Role { name: name.to_string() })
	}

	pub fn name(&self) -> &String {
		&self.name
	}
}

impl ToString for Role {
	fn to_string(&self) -> String {
		self.name.clone()
	}
}

///// The module provides types and utilities for working with global session types.
/////
//pub mod global {
//    use std::collections::HashMap;
//    use std::rc::Rc;
//
//    /// Global session types.
//    ///
//    /// > Definition 2.17
//    /// >
//    /// > G ::= p → q : { lᵢ(Uᵢ).Gᵢ } where i ∈ I
//    /// >     | μt.G
//    /// >     | t
//    /// >     | end
//    ///
//    #[derive(Debug)]
//    pub enum Type {
//        /// Interaction Type between two `Role`s.
//        ///
//        /// > G ::= p → q : { lᵢ(Uᵢ).Gᵢ } where i ∈ I
//        ///
//        /// `p` and `q` are sender and receiver `Role`s respectively, and
//        /// `g` is a mapping from `Message` to continuation `Type`.
//        ///
//        /// Each map entry represents a `Message` sent by `p` to `q`,
//        /// then the interaction proceeds as the continuation `Type`.
//        ///
//        /// The `Message` should be distinct across in this Interaction Type.
//        ///
//        Interact {
//            p: Rc<super::Role>,
//            q: Rc<super::Role>,
//            g: HashMap<super::Message, Box<Type>>,
//        },
//
//        /// Recursive Type for representing infinite behaviour.
//        ///
//        /// > G ::= μt.G
//        ///
//        /// `t` is the type variable for identifying the recursion, and
//        /// `g` is the `Type` for the continuation.
//        ///
//        Recur { t: String, g: Box<Type> },
//
//        /// Type Variable for use with `Recur` to represent infinite behaviour.
//        ///
//        /// > G ::= t
//        ///
//        /// `t` is the previously defined type variable in `Recur`.
//        ///
//        TypeVar { t: String },
//
//        /// Terminated Type with no continuation.
//        ///
//        /// > G ::= end
//        ///
//        End,
//    }
//
//    impl Type {
//        /// Returns a heap-allocated `Type::Interact` with no interactions.
//        ///
//        /// `p` and `q` are the sender and receiver `Role`s respectively.
//        ///
//        pub fn interaction(p: &Rc<super::Role>, q: &Rc<super::Role>) -> Box<Type> {
//            Box::new(Type::Interact {
//                p: p.clone(),
//                q: q.clone(),
//                g: HashMap::new(),
//            })
//        }
//
//        /// Adds a message `m_i` (and its continuation `g_i`) to a
//        /// `Type::Interact` and returns the updated Interact Type.
//        ///
//        /// `interact_type` is the input `Type`, if it is not a
//        /// `Type::Interact` it is returned unmodified.
//        ///
//        pub fn add_message(
//            interact_type: Box<Type>,
//            m_i: super::Message,
//            g_i: Box<Type>,
//        ) -> Box<Type> {
//            let mut m_interact_type = interact_type;
//            match *m_interact_type {
//                Type::Interact { ref mut g, .. } => {
//                    g.insert(m_i, g_i);
//                    ()
//                }
//                _ => (),
//            }
//
//            m_interact_type
//        }
//
//        /// Returns a heap-allocated `Type::Recur`.
//        ///
//        /// `name` is the name given to the recursion scope.
//        /// `body` is the recursion body.
//        ///
//        pub fn recur(name: &str, body: Box<Type>) -> Box<Type> {
//            Box::new(Type::Recur {
//                t: String::from(name),
//                g: body,
//            })
//        }
//
//        /// Returns a heap-allocated `Type::TypeVar`.
//        ///
//        /// `name` is the (previously defined) name of the recursion scope.
//        ///
//        pub fn typevar(name: &str) -> Box<Type> {
//            Box::new(Type::TypeVar { t: String::from(name) })
//        }
//
//        /// Returns a heap-allocated `Type::End`.
//        ///
//        /// All types should end in an instance of this.
//        pub fn end() -> Box<Type> {
//            Box::new(Type::End)
//        }
//    }
//
//    impl ToString for Type {
//        fn to_string(&self) -> String {
//            match *self {
//                Type::Interact {
//                    ref p,
//                    ref q,
//                    ref g,
//                } => {
//                    let mut interact_str = format!("{} → {}:", p.to_string(), q.to_string());
//                    match g.len() {
//                        1 => {
//                            for (m_i, g_i) in g {
//                                interact_str.push_str(&format!(
//                                    "{}.{}",
//                                    m_i.to_string(),
//                                    g_i.to_string()
//                                ))
//                            }
//                        }
//                        _ => {
//                            interact_str.push_str("{ ");
//                            let mut first = true;
//                            for (m_i, g_i) in g {
//                                if !first {
//                                    interact_str.push_str(", ")
//                                }
//                                interact_str.push_str(&format!(
//                                    "{}.{}",
//                                    m_i.to_string(),
//                                    g_i.to_string()
//                                ));
//                                first = false
//                            }
//                            interact_str.push_str(" }");
//                        }
//                    }
//
//                    interact_str
//                }
//                Type::Recur { ref t, ref g } => format!("μ{}.{}", t, g.to_string()),
//                Type::TypeVar { ref t } => format!("{}", t),
//                Type::End => String::from("end"),
//            }
//        }
//    }
//}
//
///// The module provides types and utilities for working with local session types.
/////
//pub mod local {
//    use std::collections::HashMap;
//    use std::rc::Rc;
//
//    /// Local session types.
//    ///
//    /// > Definition 2.5
//    /// >
//    /// > S ::= p &ᵢ ? lᵢ(Uᵢ).Sᵢ where i ∈ I
//    /// >     | p ⊕ᵢ ! lᵢ(Uᵢ).Sᵢ where i ∈ I
//    /// >     | μt.S
//    /// >     | t
//    /// >     | end
//    ///
//    #[derive(Debug)]
//    pub enum Type {
//        /// Branching Type receives a `Message` chosen by a `Role`.
//        ///
//        /// > S ::= p &ᵢ ? lᵢ(Uᵢ).Sᵢ where i ∈ I
//        ///
//        /// `p` is the receiver `Role`, and `s` is the mapping of
//        /// `Message` to continuation `Type` choices.
//        ///
//        /// Each map entry represents a choice received by `q`, where the
//        /// key of `s` (a `Message`) is the message expected, and the value
//        /// is the continuation `Type` of the Branching Type.
//        ///
//        Branch {
//            p: Rc<super::Role>,
//            s: HashMap<super::Message, Box<Type>>,
//        },
//
//        /// Selection Type sends a `Message` chosen by a `Role`.
//        ///
//        /// > S ::= p ⊕ᵢ ! lᵢ(Uᵢ).Sᵢ where i ∈ I
//        ///
//        /// `p` is the sender `Role`, and `s` is the mapping of
//        /// `Message` to continuation `Type` choices.
//        ///
//        /// Each map entry represents a choice sent by `p`, where the key of
//        /// `s` (a `Message`) is the message to send, and the value is the
//        /// continuation `Type` of the Selection Type.
//        ///
//        Select {
//            p: Rc<super::Role>,
//            s: HashMap<super::Message, Box<Type>>,
//        },
//
//        /// Recursive Type for representing infinite behaviour.
//        ///
//        /// > S ::= μt.S
//        ///
//        /// `t` is the type variable for identifying the recursion, and
//        /// `s` is the `Type` for the continuation.
//        ///
//        Recur { t: String, s: Box<Type> },
//
//        /// Type Variable for use with `Recur` to represent infinite behaviour.
//        ///
//        /// > S ::= t
//        ///
//        /// `t` is the previously defined type variable in `Recur`.
//        ///
//        TypeVar { t: String },
//
//        /// Terminated Type with no continuation.
//        ///
//        /// > G ::= end
//        ///
//        End,
//    }
//
//    impl Type {
//        /// Returns a heap-allocated `Type::Branch` with no interactions.
//        ///
//        /// `p` is the receiver `Role`.
//        ///
//        pub fn branch(p: &Rc<super::Role>) -> Box<Type> {
//            Box::new(Type::Branch {
//                p: p.clone(),
//                s: HashMap::new(),
//            })
//        }
//
//        /// Returns a heap-allocated `Type::Select` with no interactions.
//        ///
//        /// `p` is the sender `Role`.
//        ///
//        pub fn select(p: &Rc<super::Role>) -> Box<Type> {
//            Box::new(Type::Select {
//                p: p.clone(),
//                s: HashMap::new(),
//            })
//        }
//
//        /// Adds a message `m_i` (and its continuation `s_i`) to a
//        /// `Type::Branch` or `Type::Select` and returns the updated
//        /// Branch Type or Select Type respectively.
//        ///
//        /// `selbr_type` is the input `Type`, if it is not a
//        /// `Type::Branch` or `Type::Select` it is returned unmodified.
//        ///
//        pub fn add_message(
//            selbr_type: Box<Type>,
//            m_i: super::Message,
//            s_i: Box<Type>,
//        ) -> Box<Type> {
//            let mut m_selbr_type = selbr_type;
//            match *m_selbr_type {
//                Type::Branch { ref mut s, .. } => {
//                    s.insert(m_i, s_i);
//                    ()
//                }
//                Type::Select { ref mut s, .. } => {
//                    s.insert(m_i, s_i);
//                    ()
//                }
//                _ => (),
//            }
//
//            m_selbr_type
//        }
//
//        /// Returns the number of branches/selects
//        /// for the Type::Branch/Type::Select.
//        ///
//        /// Returns 1 for other Type variants (1 continuation).
//        ///
//        pub fn len(&self) -> usize {
//            match *self {
//                Type::Branch { ref s, .. } => s.len(),
//                Type::Select { ref s, .. } => s.len(),
//                _ => 1,
//            }
//
//        }
//
//        /// Returns a heap-allocated `Type::Recur`.
//        ///
//        /// `name` is the name given to the recursion scope.
//        /// `body` is the recursion body.
//        ///
//        pub fn recur(name: &str, body: Box<Type>) -> Box<Type> {
//            Box::new(Type::Recur {
//                t: String::from(name),
//                s: body,
//            })
//        }
//
//        /// Returns a heap-allocated `Type::TypeVar`.
//        ///
//        /// `name` is the (previously defined) name of the recursion scope.
//        ///
//        pub fn typevar(name: &str) -> Box<Type> {
//            Box::new(Type::TypeVar { t: String::from(name) })
//        }
//
//        /// Returns a heap-allocated `Type::End`.
//        ///
//        /// All types should end in an instance of this (or TypeVar).
//        ///
//        pub fn end() -> Box<Type> {
//            Box::new(Type::End)
//        }
//
//        /// Deep-compares two local types.
//        ///
//        /// This compare considers two types the same if the role names
//        /// (references) are identical, and Type is structurally equivalent.
//        ///
//        pub fn deep_eq(&self, other: &Type) -> bool {
//            match (self, other) {
//                (&Type::Branch { p: ref p1, s: ref s1 }, &Type::Branch { p: ref p2, s: ref s2 })
//                    => { let mut equal = Rc::ptr_eq(p1, p2);
//                         if s1.len() == s2.len() {
//                            for (m_i, s_i) in s1 {
//                                let mut found = false;
//                                for (m2_i, s2_i) in s2 {
//                                    if m_i.label() == m2_i.label() && m_i.payload == m2_i.payload {
//                                        found = true;
//                                        equal = equal && s_i == s2_i;
//                                    }
//                                }
//                                equal = equal && found;
//                            }
//                         } else {
//                             equal = false
//                         }
//                         equal
//                       },
//                (&Type::Select { p: ref p1, s: ref s1 }, &Type::Select { p: ref p2, s: ref s2 })
//                    => { let mut equal = Rc::ptr_eq(p1, p2);
//                         if s1.len() == s2.len() {
//                            for (m_i, s_i) in s1 {
//                                let mut found = false;
//                                for (m2_i, s2_i) in s2 {
//                                    if m_i.label() == m2_i.label() && m_i.payload == m2_i.payload {
//                                        found = true;
//                                        equal = equal && s_i == s2_i;
//                                    }
//                                }
//                                equal = equal && found;
//                            }
//                         } else {
//                             equal = false
//                         }
//                         equal
//                       },
//                (&Type::Recur { t: ref t1, s: ref s1 }, &Type::Recur { t: ref t2, s: ref s2 })
//                    => *t1 == *t2 && &*s1 == &*s2,
//                (&Type::TypeVar { t: ref t1 }, &Type::TypeVar { t: ref t2 })
//                    => *t1 == *t2,
//                (&Type::End, &Type::End) => true,
//                _ => false
//            }
//        }
//    }
//
//    impl Clone for Box<Type> {
//        fn clone(&self) -> Box<Type> {
//            match **self {
//                Type::Branch { ref p, ref s } => {
//                    let mut br = Type::branch(p);
//                    for (m_i, s_i) in s {
//                        br = Type::add_message(br, m_i.clone(), s_i.clone())
//                    }
//                    br
//                }
//                Type::Select { ref p, ref s } => {
//                    let mut sel = Type::select(p);
//                    for (m_i, s_i) in s {
//                        sel = Type::add_message(sel, m_i.clone(), s_i.clone())
//                    }
//                    sel
//                }
//                Type::Recur { ref t, ref s } => Type::recur(t, s.clone()),
//                Type::TypeVar { ref t } => Type::typevar(&t),
//                Type::End => Type::end(),
//            }
//        }
//    }
//
//    impl ToString for Type {
//        fn to_string(&self) -> String {
//            match *self {
//                Type::Branch { ref p, ref s } => {
//                    let mut branch_str = format!("{}", p.to_string());
//                    match s.len() {
//                        1 => {
//                            for (m_i, s_i) in s {
//                                branch_str.push_str(&format!(
//                                    "?{}.{}",
//                                    m_i.to_string(),
//                                    s_i.to_string()
//                                ))
//                            }
//                        }
//                        _ => {
//                            branch_str.push_str("&{ ");
//                            let mut first = true;
//                            for (m_i, s_i) in s {
//                                if !first {
//                                    branch_str.push_str(", ")
//                                }
//                                branch_str.push_str(&format!(
//                                    "?{}.{}",
//                                    m_i.to_string(),
//                                    s_i.to_string()
//                                ));
//                                first = false
//                            }
//                            branch_str.push_str(" }");
//                        }
//                    };
//
//                    branch_str
//                }
//                Type::Select { ref p, ref s } => {
//                    let mut select_str = format!("{}", p.to_string());
//                    match s.len() {
//                        1 => {
//                            for (m_i, s_i) in s {
//                                select_str.push_str(&format!(
//                                    "!{}.{}",
//                                    m_i.to_string(),
//                                    s_i.to_string()
//                                ))
//                            }
//                        }
//                        _ => {
//                            select_str.push_str("⊕{ ");
//                            let mut first = true;
//                            for (m_i, s_i) in s {
//                                if !first {
//                                    select_str.push_str(", ");
//                                }
//                                select_str.push_str(&format!(
//                                    "!{}.{}",
//                                    m_i.to_string(),
//                                    s_i.to_string()
//                                ));
//                                first = false
//                            }
//                            select_str.push_str(" }");
//                        }
//                    };
//
//                    select_str
//                }
//                Type::Recur { ref t, ref s } => format!("μ{}.{}", t, s.to_string()),
//                Type::TypeVar { ref t } => format!("{}", t),
//                Type::End => String::from("end"),
//            }
//        }
//    }
//
//    impl PartialEq for Type {
//        /// Returns true if two local::Type are structurally equivalent.
//        ///
//        /// Roles are considered the same if their String name are equal,
//        /// as this is the more common use of equality in this library.
//        /// For equality where Roles are identical references, use the
//        /// deep_eq() method.
//        ///
//        fn eq(&self, other: &Type) -> bool {
//            match (self, other) {
//                (&Type::Branch { p: ref p1, s: ref s1 }, &Type::Branch { p: ref p2, s: ref s2 })
//                    => { let mut equal = p1.name() == p2.name();
//                         if s1.len() == s2.len() {
//                            for (m_i, s_i) in s1 {
//                                let mut found = false;
//                                for (m2_i, s2_i) in s2 {
//                                    if m_i.label() == m2_i.label() && m_i.payload == m2_i.payload {
//                                        found = true;
//                                        equal = equal && s_i == s2_i;
//                                    }
//                                }
//                                equal = equal && found;
//                            }
//                         } else {
//                             equal = false
//                         }
//                         equal
//                       },
//                (&Type::Select { p: ref p1, s: ref s1 }, &Type::Select { p: ref p2, s: ref s2 })
//                    => { let mut equal = p1.name() == p2.name();
//                         if s1.len() == s2.len() {
//                            for (m_i, s_i) in s1 {
//                                let mut found = false;
//                                for (m2_i, s2_i) in s2 {
//                                    if m_i.label() == m2_i.label() && m_i.payload == m2_i.payload {
//                                        found = true;
//                                        equal = equal && s_i == s2_i;
//                                    }
//                                }
//                                equal = equal && found;
//                            }
//                         } else {
//                             equal = false
//                         }
//                         equal
//                       },
//                (&Type::Recur { t: ref t1, s: ref s1 }, &Type::Recur { t: ref t2, s: ref s2 })
//                    => *t1 == *t2 && &*s1 == &*s2,
//                (&Type::TypeVar { t: ref t1 }, &Type::TypeVar { t: ref t2 })
//                    => *t1 == *t2,
//                (&Type::End, &Type::End) => true,
//                _ => false
//            }
//        }
//    }
//}
