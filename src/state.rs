/*
Source: https://github.com/denkspuren/BitboardC4/blob/master/BitboardDesign.md
How the board is stored in the bit map.

  6 13 20 27 34 41 48   55 62     Additional row
+---------------------+ 
| 5 12 19 26 33 40 47 | 54 61     top row
| 4 11 18 25 32 39 46 | 53 60
| 3 10 17 24 31 38 45 | 52 59
| 2  9 16 23 30 37 44 | 51 58
| 1  8 15 22 29 36 43 | 50 57
| 0  7 14 21 28 35 42 | 49 56 63  bottom row
+---------------------+
*/

use crate::piece::Piece;

#[derive(Clone, PartialEq, Eq)]
pub struct State {
    player: Vec<u64>,
    height: Vec<u8>,
    counter: u8,
    moves: Vec<u8>,
}

impl State {
    pub fn new() -> State {
        let mut player = Vec::with_capacity(2);
        for _p in 0..2 {
            player.push(0);
        }
        let mut height = Vec::with_capacity(7);
        for i in 0..7 {
            height.push(i * 7);
        }
        let counter = 0;
        let moves = Vec::new();

        State { player, height, counter, moves }
    }

    pub fn make_move(&mut self, column: u8) {
        let col = column as usize;
        if self.height[col] % 7 == 6 {
            return;
        }
        let mask: u64 = 1 << self.height[col];
        self.height[col] += 1;

        let c = self.counter as usize;
        self.player[c & 1] ^= mask;
        self.moves.push(column);
        self.counter += 1;
    }

    pub fn undo_move(&mut self) {
        if self.moves.is_empty() {
            return;
        }
        let col = self.moves.pop().unwrap() as usize;
        self.counter -= 1;
        let c = self.counter as usize;
        let h = &mut self.height[col];
        *h -= 1;
        let move_: u64 = 1 << *h;
        self.player[c & 1] ^= move_;
    }

    pub fn winner(&self) -> Piece {
        if Self::win_check(&self.player[0]) {
            Piece::P1
        }
        else if Self::win_check(&self.player[1]) {
            Piece::P2
        }
        else if self.moves_left() == 0 {
            Piece::Draw
        }
        else {
            Piece::Empty
        }
    }

    fn win_check(board: &u64) -> bool {
        let directions: Vec<u8> = vec![1, 7, 6, 8];
        let mut bitboard: u64;
        for direction in directions.into_iter() {
            bitboard = *board & (*board >> direction);
            let d2 = direction * 2;
            if bitboard & (bitboard >> d2) != 0 {
                return true;
            }
        }
        false
    }

    pub fn moves(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        for (i, h) in self.height.iter().enumerate() {
            let modulus = *(h) % 7;
            if modulus != 6 {
                vec.push(i as u8);
            }
        }
        vec
    }

    pub fn moves_left(&self) -> u8 {
        42 - self.counter
    }

    pub fn moves_performed(&self) -> u8 {
        self.counter
    }

    pub fn heuristic(&self) -> f32 {
        let p1_twos = Self::count_rows_of_len(&self.player[0], 2);
        let p1_threes = Self::count_rows_of_len(&self.player[0], 3);
        
        let p2_twos = Self::count_rows_of_len(&self.player[1], 2);
        let p2_threes = Self::count_rows_of_len(&self.player[1], 3);
        //println!("{}, {}, {}, {}", p1_twos, p1_threes, p2_twos, p2_threes);

        let twos: f32 = 0.010;
        let threes: f32 = 0.020;

        twos * (p1_twos - p2_twos) as f32 + threes * (p1_threes - p2_threes) as f32
    }

    fn count_rows_of_len(board: &u64, len: u8) -> i32 {
        let mut accumulator = 0;
        let direction = vec![1, 7, 6, 8];
        let mut bb: u64;
        for d in direction.into_iter() {
            let shift = d * (len - 1);
            bb = board & (board >> shift);
            accumulator += bb.count_ones() as i32;
        }
        
        accumulator
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}

impl ToString for State {
    fn to_string(&self) -> String {
        let mut string = String::new();
        for i in (0..6).rev() {
            for j in 0..7 {
                let factor = i + 7 * j;
                let p1 = 1 & (self.player[0] >> factor);
                let p2 = 1 & (self.player[1] >> factor);
                
                let temp = {
                    if p1 == 1 {
                        "-X-"
                    }
                    else if p2 == 1 {
                        "-O-"
                    }
                    else {
                        "|_|"
                    }
                };

                string.push_str(temp);
            }
            string.push('\n');
        }
        string
    }
}


#[test]
fn string_test() {
    let mut board = State::new();
    board.make_move(0);
    board.make_move(1);
    println!("{}", board.to_string());

    board.undo_move();
    println!("{}", board.to_string());

    board.undo_move();

    board.make_move(0); // 1
    board.make_move(0); // 2
    board.make_move(1); // 1
    board.make_move(1); // 2
    board.make_move(2); // 1
    board.make_move(2); // 2
    board.make_move(3); // 1

    println!("{}", board.to_string());
    println!("Winner: {}, Heuristic: {:0.3}", board.winner().to_string(), board.heuristic());
}

#[test]
fn moves_test() {
    let mut board = State::new();

    // Set up test moves on one column.
    for _i in 0..6 {
        board.make_move(0);
    }
    
    board.print();
    let mut moves = board.moves();
    for m in moves.into_iter() {
        print!("{}, ", m);
    }
    println!();

    board.undo_move();
    board.print();
    moves = board.moves();
    for m in moves.into_iter() {
        print!("{}, ", m);
    }
    println!();
}
