fn accum(s:&str) -> String {
    s.chars()
    .enumerate()
    .map(|(index, value)|{
        let mut return_string = String::new();
        for times in 0..=index {
            if times == 0 {
                return_string = [return_string, value.to_uppercase().to_string()].join("");
            } else {
                return_string = [return_string, value.to_lowercase().to_string()].join("");
            }
        }
        return_string
    }).collect::<Vec<String>>().join("-")
}
