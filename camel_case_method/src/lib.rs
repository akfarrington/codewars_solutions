fn camel_case(input: &str) -> String {
    let words = input.trim().split_whitespace().collect::<Vec<&str>>();

    let return_value = words.iter().map(|word| {
        word.to_lowercase().chars().enumerate().map(|(index, char)|{
            match index {
                0 => char.to_uppercase().to_string(),
                _ => char.to_string()
            }
        }).collect::<String>()
    }).collect::<String>();

    return_value
}
