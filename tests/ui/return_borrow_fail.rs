use k4rust::*;

k4rust_api! {
    pub fn return_borrow_fail(_x: K, y: K) -> K {
        y // should fail because y is a borrowed reference (&K) but signature returns K
    }
}

fn main() {}
