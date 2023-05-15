// move_semantics2.rs
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// Expected output:
// vec0 has length 3 content `[22, 44, 66]`
// vec1 has length 4 content `[22, 44, 66, 88]`

fn main() {
    let vec0 = fill_vec([22, 44, 66]);

    // Do not move the following line!
    let mut vec1 = fill_vec([22, 44, 66]);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(data: [i32; 3]) -> Vec<i32> {
    let mut vec = Vec::new();

    vec.push(data[0]);
    vec.push(data[1]);
    vec.push(data[2]);

    vec
}
