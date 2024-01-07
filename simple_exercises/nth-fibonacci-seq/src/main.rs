fn main() {
    println!("{}", r_nth_fibonacci(20));
    println!("{}", nth_fibonacci(20))
}

fn r_nth_fibonacci(n: i32) -> i64 {
    if n < 3 {
        1
    } else {
        r_nth_fibonacci(n - 1) + r_nth_fibonacci(n - 2)
    }
}

fn nth_fibonacci(n: i32) -> i64 {
    let mut first = 0;
    let mut second = 1;

    let mut i = 0;
    while i < n - 1 {
        let tmp = second;
        second += first;
        first = tmp;
        i += 1;
    }
    second
}
