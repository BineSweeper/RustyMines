mod slot;
mod game;

fn main() {
    println!("Hello, world!");
    loop {
        println!("\u{001B}[2J");
        println!("Welcome to RustyMines!");
        println!("If you want to exit, type 'exit' at any point of time.");
        println!("Please enter the level (default is easy): (easy, medium, hard, custom)");
        println!("  Easy   - 9x9 board with 10 mines");
        println!("  Medium - 16x16 board with 40 mines");
        println!("  Hard   - 30x16 board with 99 mines");
        println!("  Custom - You specify the board size and mine count");
        let mut level = String::new();
        std::io::stdin().read_line(&mut level).expect("Failed to read line");
        level = level.trim().to_ascii_lowercase().to_string();
        if level == "exit" {
            break;
        }
        let mut width = 0;
        let mut height = 0;
        let mut mine_count = 0;
        if level == "medium" {
            width = 16;
            height = 16;
            mine_count = 40;
        } else if level == "hard" {
            width = 30;
            height = 16;
            mine_count = 99;
        } else if level == "custom" {
            println!("Please enter the width of the board:");
            let mut width_str = String::new();
            std::io::stdin().read_line(&mut width_str).expect("Failed to read line");
            width_str = width_str.trim().to_string();
            width = width_str.parse::<i32>().expect("Failed to parse width");
            println!("Please enter the height of the board:");
            let mut height_str = String::new();
            std::io::stdin().read_line(&mut height_str).expect("Failed to read line");
            height_str = height_str.trim().to_string();
            height = height_str.parse::<i32>().expect("Failed to parse height");
            println!("Please enter the mine count:");
            let mut mine_count_str = String::new();
            std::io::stdin().read_line(&mut mine_count_str).expect("Failed to read line");
            mine_count_str = mine_count_str.trim().to_string();
            mine_count = mine_count_str.parse::<i32>().expect("Failed to parse mine count");
        } else {
            width = 9;
            height = 9;
            mine_count = 10;
        }

        let mut game = game::Game::new(width, height, mine_count);

        loop {
            println!("\u{001B}[2J");
            println!("RustyMines");
            game.print_board();
            println!("Enter the coordinates of the slot you want to reveal: ");
            let mut coords = String::new();
            std::io::stdin().read_line(&mut coords).expect("Failed to read line");
            coords = coords.trim().to_string();
            if coords == "exit" {
                break;
            }
            let coords: Vec<&str> = coords.split(' ').collect();
            let x = coords[1].parse::<i32>().expect("Failed to parse x");
            let y = coords[0].parse::<i32>().expect("Failed to parse y");
            game.reveal(x, y);
            game.check_win();
            if game.is_won || game.is_lost {
                println!("\u{001B}[2J");
                println!("RustyMines");
                game.print_board();
                println!("Time taken: {} seconds", game.start_time.elapsed().as_secs());
                if game.is_won {
                    println!("You won!");
                } else if game.is_lost {
                    println!("You lost!");
                }
                println!("Press enter to continue...");
                let mut enter = String::new();
                std::io::stdin().read_line(&mut enter).expect("Failed to read line");
                break;
            }
        }
    }
}
