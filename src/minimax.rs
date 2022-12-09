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
        let max = Self::maxbool(self.player);
        let moves = self.state.moves();
        let mut value = if max { V::NEG_INFINITY } else { V::INFINITY };
        let mut action_found = 0;

        // Use negamax on each initial action
        for action in moves.into_iter() {
            let neg = Self::negamax(&mut self.state, action, depth, max);
            if  max && neg > value || !max && neg < value {
                value = neg;
                action_found = action; // Stores the action with the best score.
            }
        }

        action_found
    }

    fn negamax(state: &mut State, action: A, depth: D, max: bool) -> V {
        // Check for terminal states
        let winner = state.winner();
        // Either the game is over
        if winner != Piece::Empty {
            return Self::utility(winner, Self::player(max));
        }
        // Or the search depth has been reached.
        if depth == 0 {
            return state.heuristic();
        }

        // Evaluate children
        let mut value = V::NEG_INFINITY;
        state.make_move(action);
        let moves = state.moves();
        for child in moves.into_iter() {
            value = V::max(value, -Self::negamax(state, child, depth - 1, !max));
            state.undo_move();
        }
        state.undo_move();
        value
    }

    fn maxbool(player: Piece) -> bool {
        player == Piece::P1
    }
    fn player(max: bool) -> Piece {
        if max { Piece::P1 }
        else { Piece::P2 }
    }

    /// Value for when the game either has a winner or there are no moves left.
    fn utility(winner: Piece, player: Piece) -> V {
        match winner {
            Empty  => 0.0,
            Draw   => 0.5,
            P1     => {
                if player == P1 { 1.0 }
                else { 0.0 }
            },
            P2     => {
                if player == P2 { 1.0 }
                else { 0.0 }
            },
        }

    }

}
