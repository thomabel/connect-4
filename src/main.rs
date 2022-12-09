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
    let mut agent1 = minimax::MinimaxAgent::new(piece::Piece::P1);
    let mut agent2 = minimax::MinimaxAgent::new(piece::Piece::P2);

    let mut board = state::State::new();

    let mut mov1: Option<minimax::A>;
    let mut mov2: Option<minimax::A> = None;
    let depth = 7;

    while board.winner() == piece::Piece::Empty {
        mov1 = Some(agent1.run(depth, mov2));
        board.make_move(mov1.unwrap());
        println!("{}\n", board.to_string());

        mov2 = Some(agent2.run(depth, mov1));
        board.make_move(mov2.unwrap());
        println!("{}\n", board.to_string());
    }

    println!("{}", board.to_string());
    println!("Winner: {}", board.winner().to_string());
}
