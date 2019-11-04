fn get_middle(s:&str) -> String {
    let chars = s.chars()
    .map(|c| {
        c.to_string()
    })
    .collect::<Vec<String>>();

    match s.len() % 2 {
        0 => {
            let first = (s.len() / 2) - 1;
            let second = first + 1;
            format!("{}", &[format!("{}", chars[first]), format!("{}", chars[second])].join(""))
        }
        _ => {
            let middle = s.len() / 2;
            format!("{}", &chars[middle])
        }
    }
}
