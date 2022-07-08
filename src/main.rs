
fn wipe_console()
{
    println!("{}[2J", 27 as char);
}

#[derive(PartialEq)]
enum Cell
{
    Empty,
    Cross,
    Circle
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Cell::Empty => " ",
            Cell::Circle => "O",
            Cell::Cross => "X"
        })
    }
}

fn get_index(index: char) -> usize
{
    return match index {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        '1' => 0,
        '2' => 1,
        '3' => 2,
        _ => 0
    };
}

fn print_grid(grid: &[[Cell; 3]; 3])
{
    // rows header
    println!("   {} {} {}", 1, 2, 3);

    for r in 0..3
    {
        println!("{}  {}|{}|{}", match r {
            0 => "A",
            1 => "B",
            2 => "C",
            _ => "?"
        }, &grid[r][0], &grid[r][1], &grid[r][2]);

        if r != 2
        {
            println!("   -----");
        };
    }
}

fn win_cmp(c1: &Cell, c2: &Cell, c3: &Cell) -> bool
{
    return c1 != &Cell::Empty && c1 == c2 && c2 == c3;
}

fn check_winner(grid: &[[Cell; 3]; 3]) -> &Cell
{
    // colummns & rows
    for i in 0..3
    {
        if win_cmp(&grid[i][0], &grid[i][1], &grid[i][2]) { return &grid[i][0]; }
        if win_cmp(&grid[0][i], &grid[1][i], &grid[2][i]) { return &grid[0][i]; }
    }

    // diagonal
    if win_cmp(&grid[0][0], &grid[1][1], &grid[2][2]) { return &grid[0][0]; }
    if win_cmp(&grid[0][2], &grid[1][1], &grid[2][0]) { return &grid[0][2]; }
    if win_cmp(&grid[2][0], &grid[1][1], &grid[0][2]) { return &grid[2][0]; }
    
    return &Cell::Empty;
}

fn main() {
    let mut grid: [[Cell; 3]; 3] = [[Cell::Empty, Cell::Empty, Cell::Empty], [Cell::Empty, Cell::Empty, Cell::Empty], [Cell::Empty, Cell::Empty, Cell::Empty]];

    // 1 or 0 / player 1 or player 2
    let mut turn: bool = true;

    let mut winner: &Cell = &Cell::Empty;

    while winner == &Cell::Empty
    {
        wipe_console();
        println!("Player {}'s turn", if turn { "1" } else { "2" });
        print_grid(&grid);

        let mut input = String::new();
        let mut bytes;
        let mut col: usize = 0;
        let mut row: usize = 0;
   
        // 3 bytes including the null terminator
        // <row><col><null>
        loop
        {
            bytes = std::io::stdin().read_line(&mut input).unwrap();


            if bytes <= 2 { input.clear(); continue; }

            let r: char = input.as_bytes()[0].to_ascii_lowercase() as char;
            let c: char = input.as_bytes()[1].to_ascii_lowercase() as char;

            if 
                (!match r {'a' => true, 'b' => true, 'c' => true, _ => false}) || 
                (!match c {'1' => true, '2' => true, '3' => true, _ => false})
            {
                // force the condition to fail
                bytes = 0;
            }

            match bytes
            {
                3 => {
                    col = get_index(c);
                    row = get_index(r);
                    if grid[row][col] == Cell::Empty
                    {
                        break;
                    }
                    input.clear();
                    println!("This cell is occupied!");
                },
                _ => {
                    input.clear();
                    println!("You must enter a colum and row in the format CR");
                }
            };

            
        }   
        
        grid[row][col] = if turn { Cell::Circle } else { Cell::Cross };
        winner = check_winner(&grid);

        turn = !turn;
    }

    // turn will be 1 'flip' ahead here so we'll just flip it back
    turn = !turn;

    wipe_console();
    println!("Player {} won!", if turn { "1" } else { "2" });
    print_grid(&grid);
}
