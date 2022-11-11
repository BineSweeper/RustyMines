use std::cmp::{max, min};
use std::time::Instant;
use rand::distributions::{Distribution, Uniform};

pub(crate) struct Game {
    pub width: i32,
    pub height: i32,
    pub mine_count: i32,
    pub slots: Vec<Vec<Slot>>,
    pub is_won: bool,
    pub is_lost: bool,
    pub start_time: Instant,
}

impl Game {
    pub(crate) fn new(width: i32, height: i32, mine_count: i32) -> Game {
        let mut game = Game {
            width,
            height,
            mine_count,
            slots: vec![],
            is_won: false,
            is_lost: false,
            start_time: Instant::now(),
        };

        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                row.push(Slot::new());
            }
            game.slots.push(row);
        }

        let mut rng = rand::thread_rng();
        let die = Uniform::from(0..10);

        let mut placed_mines = 0;

        loop {
            let x = die.sample(&mut rng);
            let y = die.sample(&mut rng);

            if !game.slots[y as usize][x as usize].is_mine {
                game.slots[y as usize][x as usize].set_mine();
                placed_mines += 1;
            }

            if placed_mines == mine_count {
                break;
            }
        }

        game
    }

    pub fn check_win(&mut self) {
        if self.is_lost || self.is_won {
            return;
        }
        let mut revealed_slots = 0;
        for row in &self.slots {
            for slot in row {
                if slot.is_revealed {
                    revealed_slots += 1;
                }
            }
        }

        if revealed_slots == (self.width * self.height) - self.mine_count {
            self.is_won = true;
        }
    }

    pub fn reveal(&mut self, x: i32, y: i32) {
        if self.is_lost || self.is_won {
            return;
        }
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return;
        }
        if self.slots[y as usize][x as usize].is_revealed {
            return;
        }

        self.slots[y as usize][x as usize].reveal();

        if self.slots[y as usize][x as usize].is_mine {
            self.is_lost = true;
            for row in &mut self.slots {
                for slot in row {
                    if slot.is_mine {
                        slot.reveal();
                    }
                }
            }
            self.end_time = Some(Instant::now());
        }

        if self.slots[y as usize][x as usize].is_blank() {
            for y2 in max(0, y - 1)..=min(height - 1, y + 1) {
                for x2 in max(0, x - 1)..=min(width - 1, x + 1) {
                    reveal(x2, y2);
                }
            }
        }
    }

    pub fn print_board(&self) {
        print("  ");
        for x in 0..self.width {
            print("{} ", x);
            if x < 10 && self.width >= 10 {
                print(" ");
            }
        }
        println!();
        for y in 0..self.height {
            print("{} ", y);
            if y < 10 && self.height >= 10 {
                print(" ");
            }
            for x in 0..self.width {
                print("{} ", self.slots[y as usize][x as usize].description());
                if self.width >= 10 {
                    print(" ");
                }
            }
            println!();
        }
    }
}
