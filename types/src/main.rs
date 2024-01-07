fn main() {
    let a: u8 = 200;
    let b: u8 = 150;

    // Wrapping add
    let wrapping_result = a.wrapping_add(b);
    println!("Wrapping Result: {}", wrapping_result);

    // Checked add
    let checked_result = a.checked_add(b);
    println!("Checked Result: {:?}", checked_result);

    // Overflowing add
    let overflowing_result = a.overflowing_add(b);
    println!("Overflowing Result: {:?}", overflowing_result.1);

    // Saturating add
    let saturating_result = a.saturating_add(b);
    println!("Saturating Result: {}", saturating_result);

    let tup: (char, i32, f64, &str) = ('a', 10, 1.0, "Hello");
    println!("Tuple: {:?}", tup);
    const UNIT: () = ();
    println!("Unit: {:?}", UNIT);
    let mut arr = [434; 10];
    println!("Array: {:?}", arr);
    arr[0] = 1009;
    println!("Array: {:?}", arr[100]);
    println!("Array: {:?}", arr);
}
