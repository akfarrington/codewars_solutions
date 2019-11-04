#[derive(PartialEq)]
enum CellType {
    White,
    Black,
}

fn chessboard_cell_color(cell1: &str, cell2: &str) -> bool {
    convert_cell(cell1) == convert_cell(cell2)
}

fn convert_cell(coord: &str) -> CellType {
    let mut x = 0;
    let mut y = 0;
    let chars: Vec<char> = coord.chars().collect();

    match chars[0] {
        'A' => x = 1,
        'B' => x = 2,
        'C' => x = 3,
        'D' => x = 4,
        'E' => x = 5,
        'F' => x = 6,
        'G' => x = 7,
        'H' => x = 8,
        _ => x = 0,
    };

    y = chars[1].to_string().parse().unwrap();

    match (x % 2 == 0, y % 2 == 0){
        (true, true) => CellType::Black,
        (false, false) => CellType::Black,
        (false, true) => CellType::White,
        (true, false) => CellType::White,
    }
}
