// 1 line comment
/*
    Multi line comment
    Multi line comment
*/

fn main() {
    println!("Hello, world!");

    let a = 1; // lock
    let b = &a; // reference to a (not move ownership, not editable and can make many)

    let mut a2 = 1; // lock ()
    let b2 = &mut a2; // reference to a2 (move ownership, editable and can make one)

    *b2 = 2; // dereference and edit
}
