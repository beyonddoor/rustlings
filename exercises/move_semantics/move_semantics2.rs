// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// TODO 有点复杂

fn main() {
    let mut vec0 = Vec::new();

    let vec1 = fill_vec(&mut vec0);

    // let mut vec0 = Vec::new();
    // vec0.push(1);

    let vec0 = &vec1;

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    let vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
