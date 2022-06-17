#[derive(Clone, Copy, PartialEq)]
enum Square {
    x,
    o,
    empty,
}
#[derive(Clone, PartialEq)]
struct tictactoe_board {
    h1: Vec<Square>,
    h2: Vec<Square>,
    h3: Vec<Square>,
}

impl tictactoe_board {
    pub fn new_board() -> tictactoe_board{
        return tictactoe_board {
            h1: vec![Square::empty; 3],
            h2: vec![Square::empty; 3],
            h3: vec![Square::empty; 3],
        }       
    }

    fn square_to_string(s: &Square) -> char {
        match s {
            Square::x => 'X',
            Square::o => 'O',
            Square::empty => ' ',
        }
    }

    pub fn print_board(self) -> tictactoe_board{
        for i in &self.h1 {
            print!("|");
            print!("{}", tictactoe_board::square_to_string(i));
        }
        println!("|");
        for i in &self.h2 {
            print!("|");
            print!("{}", tictactoe_board::square_to_string(i));
        }
        println!("|");
        for i in &self.h3 {
            print!("|");
            print!("{}", tictactoe_board::square_to_string(i));
        }
        println!("|");
        return self;
    }

    pub fn mark_square(
        mut self, 
        s: Square, 
        coordinates: (usize, usize)) 
        -> Result<tictactoe_board, &'static str>{
        if s == Square::empty {
            return Err("Cannot mark square as empty");
        }
        match coordinates {
            (0, x) => {
                if x > 2 {
                    return Err("Outside bounds");
                }
                let v = self.h1[x];
                if v == Square::empty {
                    self.h1[x] = s;
                }else{
                    return Err("Square already marked !")
                }
                return Ok(self);
            },
            (1, x) => {
                if x > 2 {
                    return Err("Outside bounds");
                }
                let v = self.h2[x];
                if v == Square::empty {
                    self.h2[x] = s;
                }else{
                    return Err("Square already marked !")
                }
                return Ok(self);
            },
            (2, x) => {
                if x > 2 {
                    return Err("Outside bounds");
                }
                let v = self.h3[x];
                if v == Square::empty {
                    self.h3[x] = s;
                }else{
                    return Err("Square already marked !")
                }
                return Ok(self);
            },
            _ => Err("Outside bounds")
        }
    }

    pub fn check_for_win(self) -> Option<(tictactoe_board, &'static str)> {
        let row1 = self.h1.as_slice();
        let row2 = self.h2.as_slice();
        let row3 = self.h3.as_slice();
        let bvec = vec![row1, row2, row3];
        let board = bvec.as_slice();

        let x_victory = "Victory for Xs";
        let o_victory = "Victory for Os";

        // All victory conditions for Xs and Os, horizontal, vertical, diagonal
        match board {
            // Single row victories for Xs and Os
            [   [Square::x, Square::x, Square::x],
                [_,_,_],
                [_,_,_]] => Some((self, x_victory)),
            [   [__,_,_],
                [Square::x, Square::x, Square::x],
                [_,_,_]] => Some((self, x_victory)),
            [   [_,_,_],
                [_,_,_],
                [Square::x, Square::x, Square::x]] => Some((self, x_victory)),
            [   [Square::o, Square::o, Square::o],
                [_,_,_],
                [_,_,_]] => Some((self, o_victory)),
            [   [_,_,_],    
                [Square::o, Square::o, Square::o],
                [_,_,_]] => Some((self, o_victory)),
            [   [_,_,_],    
                [_,_,_],
                [Square::o, Square::o, Square::o]] => Some((self, o_victory)),

            // Single colums victories for Xs and Os
            [   [Square::x,_,_],
                [Square::x,_,_],
                [Square::x,_,_]] => Some((self, x_victory)),
            [   [_,Square::x,_],
                [_,Square::x,_],
                [_,Square::x,_]] => Some((self, x_victory)),
            [   [_,_,Square::x],
                [_,_,Square::x],
                [_,_,Square::x]] => Some((self, x_victory)),
            [   [Square::o,_,_],    
                [Square::o,_,_],    
                [Square::o,_,_]] => Some((self, o_victory)),
            [   [_,Square::o,_],    
                [_,Square::o,_],    
                [_,Square::o,_]] => Some((self, o_victory)),
            [   [_,_,Square::o],    
                [_,_,Square::o],    
                [_,_,Square::o]] => Some((self, o_victory)),

            // Diagonal victories for Xs and Os
            [   [_,_,Square::x],
                [_,Square::x,_],
                [Square::x,_,_]] => Some((self, x_victory)),
            [   [Square::x,_,_],
                [_,Square::x,_],
                [_,_,Square::x]] => Some((self, x_victory)),
            [   [_,_,Square::o],
                [_,Square::o,_],
                [Square::o,_,_]] => Some((self, o_victory)),
            [   [Square::o,_,_],
                [_,Square::o,_],
                [_,_,Square::o]] => Some((self, o_victory)),

            _ => None
        }
    }
}


fn main() {
    let board = tictactoe_board::new_board()
        .mark_square(Square::o, (0,1)).unwrap()
        .mark_square(Square::x, (0,0)).unwrap()
        .mark_square(Square::o, (1,2)).unwrap()
        .mark_square(Square::x, (1,1)).unwrap()
        .mark_square(Square::o, (2,0)).unwrap()
        .mark_square(Square::x, (2,2)).unwrap()
        .print_board();

    match board.check_for_win() {
        Some((_board, victory_message)) => println!("{}", victory_message),
        None => println!("No victory")
    }

    return;
}
