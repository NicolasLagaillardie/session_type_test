extern crate session_types_extension;

use session_types_extension::HasDual;

struct CustomProto;

impl HasDual for CustomProto { //~ ERROR the trait bound `CustomProto: session_types_extension::private::Sealed` is not satisfied
    type Dual = CustomProto;
}

fn main() {}
