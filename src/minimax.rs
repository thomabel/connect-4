use slab_tree::{Tree, TreeBuilder, NodeId, NodeMut, NodeRef};

use crate::{state::State, piece::Piece};

type V = f32;
pub type A = u8;
type D = i32;

pub struct MinimaxAgent {
    //tree: Tree<(A, Option<V>)>,
    //stack: Vec<(NodeId, D)>,
    //root2: NodeId,

    player: Piece,
    state: State,
    alpha: V,
    beta: V,
}

impl MinimaxAgent {
    pub fn new(player: Piece) -> MinimaxAgent {
        // Create the tree with a root node.
        //let root = (A::MAX, None);
        //let tree = TreeBuilder::new().with_capacity(1024).with_root(root).build();
        //let stack = Vec::with_capacity(usize::pow(7, 4));
        //let root2 = tree.root_id().unwrap();
        
        let state = State::new();
        let alpha = V::MIN;
        let beta = V::MAX;

        MinimaxAgent { //tree, stack, root2, 
            player, state, alpha, beta }
    }

    pub fn run(&mut self, depth: D, last_move: Option<A>) -> A {
        // Apply our opponent's move if applicable.
        if let Some(action) = last_move {
            self.state.make_move(action);
        }

        // Set up for inital action loop
        let moves = self.state.moves();
        let mut best_action = moves[0];

        // Utility values
        let mut value = {
            if self.player == Piece::P1 { V::NEG_INFINITY }
            else { V::INFINITY }
        };
        self.alpha = V::NEG_INFINITY;
        self.beta = V::INFINITY;

        // Use negamax on each initial action
        for action in moves.into_iter() {
            // Initial recursive call
            let result = Self::negamax(
                &mut self.state, action, depth, 
                self.alpha, 
                self.beta
            );
            self.state.undo_move();

            // Tracking best action
            if self.player == Piece::P1 && result > value || 
                self.player == Piece::P2 && result < value {
                value = result;
                best_action = action;
                println!("(action, value) {}, {}", best_action, value);
            }
        }
        self.state.make_move(best_action);
        best_action
    }
    
    fn negamax(state: &mut State, action: A, depth: D, mut alpha: V, beta: V) -> V {
        // Apply move to the board and evaluate the resulting state.
        state.make_move(action);
        if let Some(value) = Self::terminal(state, depth) {
            return value;
        }

        // Evaluate children
        let mut value = V::NEG_INFINITY;
        let moves = state.moves();
        for action in moves.into_iter() {
            // Recursive call
            let result = -Self::negamax(
                state, action, 
                depth - 1, 
                -beta, 
                -alpha
            );
            state.undo_move();
            
            // Pruning
            value = V::max(value, result);
            alpha = V::max(alpha, value);
            if alpha >= beta {
                break;
            }
        }

        value
    }

    /// Value for when the game either has a winner or there are no moves left.
    fn utility(winner: Piece) -> V {
        match winner {
            Piece::Empty  =>  0.0,
            Piece::Draw   =>  0.5,
            Piece::P1     =>  1.0,
            Piece::P2     => -1.0,
        }

    }

    /// Check for terminal states
    fn terminal(state: &State, depth: D) -> Option<V> {
        let winner = state.winner();
        // Either the game is over
        if winner != Piece::Empty {
            Some(Self::utility(winner))
        }
        // Or the search depth has been reached.
        else if depth == 0 {
            Some(state.heuristic())
        }
        else {
            None
        }
    }
}
