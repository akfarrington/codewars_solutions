fn parse(code: &str) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    let mut this_value: i32 = 0;

    for character in code.chars() {
        match character {
            'i' => this_value += 1,
            'd' => this_value -= 1,
            's' => this_value = this_value.pow(2),
            'o' => output.push(this_value),
            _ => {}
        }
    }

    output
}
