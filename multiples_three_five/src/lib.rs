fn solution(num: i32) -> i32 {
    (1..num)
        .filter(|i| i % 3 == 0 || i % 5 == 0)
        .sum()
}
