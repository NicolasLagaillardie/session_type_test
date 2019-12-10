// Copyright 2017 The libsesstype Contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::rc::Rc;
use super::global::Type as G;
use super::local::Type as S;
use super::Role;

/// Converts a global::Type into a local::Type.
///
/// This implements the projection G↾q defined in Definition A.1.
///
pub fn project(global_type: &Box<G>, role: &Rc<Role>) -> Option<Box<S>> {
    match **global_type {
        G::Interact {
            ref p,
            ref q,
            ref g,
        } => {
            if Rc::ptr_eq(&role, &p) {
                let mut sel = S::select(q);
                for (m_i, g_i) in g {
                    let projected_s = project(&g_i, role);
                    match projected_s {
                        Some(projected_s) => {
                            sel = S::add_message(sel, m_i.clone(), projected_s);
                            ()
                        }
                        None => (),
                    }
                }
                if sel.len() > 0 { Some(sel) } else { None }
            } else if Rc::ptr_eq(&role, &q) {
                let mut br = S::branch(p);
                for (m_i, g_i) in g {
                    let projected_s = project(&g_i, role);
                    match projected_s {
                        Some(projected_s) => {
                            br = S::add_message(br, m_i.clone(), projected_s);
                            ()
                        }
                        None => (),
                    }
                }
                if br.len() > 0 { Some(br) } else { None }
            } else {
                // p != role != q
                match g.len() {
                    1 => {
                        let item = g.iter().next();
                        match item {
                            Some(ref item) => project(item.1, role),
                            None => None,
                        }
                    }
                    _ => {
                        let mut iter = g.iter();
                        let first_item = iter.next();
                        match first_item {
                            Some(ref first_item) => {
                                let mut merged = project(first_item.1, role);
                                for (_, g_i) in iter {
                                    let projected_s = project(&g_i, role);
                                    match projected_s {
                                        Some(ref projected_s) => {
                                            match merged {
                                                Some(merged_) => {
                                                    merged = merge(&merged_, projected_s);
                                                    ()
                                                }
                                                None => (),
                                            }
                                            ()
                                        }
                                        None => (),
                                    }
                                }
                                match merged {
                                    Some(s) => if s.len() > 0 { Some(s) } else { None }
                                    None => None,
                                }
                            }
                            None => None,
                        }
                    }
                }
            }
        }
        G::Recur { ref t, ref g } => {
            let projected_s = project(g, role);
            match projected_s {
                Some(ref projected_s) => Some(S::recur(t, projected_s.clone())),
                None => None,
            }
        }
        G::TypeVar { ref t } => Some(S::typevar(t)),
        G::End => Some(S::end()),
    }
}

/// Merges two local session types.
///
/// This implements the merge operator Π on session types defined in Definition A.1.
///
fn merge(l: &Box<S>, r: &Box<S>) -> Option<Box<S>> {
    let (ref box_l, ref box_r) = (&*l, &*r);
    match (box_l.as_ref(), box_r.as_ref()) {
        (&S::Branch { ref p, ref s },
         &S::Branch {
             p: ref p_r,
             s: ref s_r,
         }) => {
            if Rc::ptr_eq(p, p_r) {
                let mut br = S::branch(p);
                for (m_i, s_i) in s {
                    if s_r.contains_key(m_i) {
                        // Intersect case
                        let s_j = s_r.get(m_i);
                        match s_j {
                            Some(s_j) => {
                                let merged_br = merge(s_i, s_j);
                                match merged_br {
                                    Some(merged_br) => {
                                        br = S::add_message(br, m_i.clone(), merged_br);
                                        ()
                                    }
                                    None => (),
                                }
                            }
                            None => (),
                        }
                    } else {
                        // Only in s case
                        br = S::add_message(br, m_i.clone(), s_i.clone())
                    }
                }
                for (m_j, s_j) in s_r {
                    if !s.contains_key(m_j) {
                        // Only in s_r case
                        br = S::add_message(br, m_j.clone(), s_j.clone())
                    }
                }
                Some(br)
            } else {
                None
            }
        }
        (&S::Select { ref p, ref s },
         &S::Select {
             p: ref p_r,
             s: ref s_r,
         }) => {
            if Rc::ptr_eq(p, p_r) {
                let mut sel = S::select(p);
                for (m_i, s_i) in s {
                    if s_r.contains_key(m_i) {
                        // Note: s_i must be equal to s_r.entry(m_i)
                        sel = S::add_message(sel, m_i.clone(), s_i.clone())
                    }
                }
                Some(sel)
            } else {
                None
            }
        }
        (&S::Recur { ref t, ref s },
         &S::Recur {
             t: ref t_r,
             s: ref s_r,
         }) => {
            if t == t_r {
                let s_merged = merge(s, s_r);
                match s_merged {
                    Some(s_) => Some(S::recur(t, s_)),
                    None => None,
                }
            } else {
                None
            }
        }
        (&S::TypeVar { ref t }, &S::TypeVar { t: ref t_r }) => {
            if t == t_r { Some(S::typevar(t)) } else { None }
        }
        (&S::End, &S::End) => Some(S::end()),
        _ => None,
    }
}
