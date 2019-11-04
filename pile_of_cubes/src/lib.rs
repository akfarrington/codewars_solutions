fn find_nb(n: u64) -> i32 {
    let mut i: u64 = 1;
    let mut total: u64 = 0;

    loop {
        total += i.pow(3);

        if total == n {
            return i as i32;
        } if total > n {
            return -1_i32;
        } else {
            i += 1;
        }
    }
}
