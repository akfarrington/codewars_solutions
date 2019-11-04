fn dont_give_me_five(start: isize, end: isize) -> isize {
    (start..=end).filter(|num| {
        !format!("{}", num)
            .chars()
            .map(|char| char.to_string())
            .collect::<Vec<String>>()
            .contains(&"5".to_string())
    })
        .collect::<Vec<isize>>().len() as isize
}
