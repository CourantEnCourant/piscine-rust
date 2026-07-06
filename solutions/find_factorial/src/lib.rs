pub fn factorial(num: u64) -> u64 {
    if num == 0 {
        return 1;
    }
    let mut acc = 1;
    for n in 1..=num {
        acc *= n;
    }
    acc
}
