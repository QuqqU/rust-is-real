mod automata;

use automata::FloatingPoint;

pub fn is_real(s: &String) -> bool {
    FloatingPoint::accept(s)
}
