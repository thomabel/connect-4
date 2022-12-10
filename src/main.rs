/*
Artificial Intelligence
Thomas Abel
Final Project

Connect 4 adversarial AI comparison
- minimax
- monte carlo

Sources
- https://github.com/denkspuren/BitboardC4/blob/master/BitboardDesign.md
- https://towardsdatascience.com/creating-the-perfect-connect-four-ai-bot-c165115557b0
- https://medium.com/analytics-vidhya/artificial-intelligence-at-play-connect-four-minimax-algorithm-explained-3b5fc32e4a4f

*/

mod state;
mod minimax;
mod piece;

fn main() {
    experiment();
}

fn experiment() {
    println!("---------- START ----------");

    let mut board = state::State::new();

    let mut agent = vec![
        minimax::MinimaxAgent::new(piece::Piece::P1), 
        minimax::MinimaxAgent::new(piece::Piece::P2),
    ];

    let mut mov: Vec<Option<minimax::A>> = vec![
        None, None
    ];

    let depth = 9;

    while board.winner() == piece::Piece::Empty {
        // Get indicies
        let p = (board.moves_performed() % 2) as usize;
        let o = (p + 1) % 2;

        // Print information
        println!("---------- Player {} Move ----------", p + 1);
        print!("move_o: {}, ", mov[o].unwrap_or(8));
        println!("move_p: {}", mov[p].unwrap_or(8));

        mov[p] = Some(agent[p].run(depth, mov[o]));
        board.make_move(mov[p].unwrap());
        board.print();

    }

    println!("Winner: {}\n", board.winner().to_string());

    println!("---------- END ----------");
}
