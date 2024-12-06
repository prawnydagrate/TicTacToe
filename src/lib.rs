pub mod mech;

use fastrand;
use mech::{Game, GameState, Player};
use std::cmp;

/// Gets the best move in the position, **assuming** the game
/// is ongoing. This function **will panic** if the game has already
/// ended.
pub fn get_best_move(game: &Game, depth: usize) -> (usize, usize) {
    let maxmoves = game.grid().n().pow(2);
    if game.empty().len() == maxmoves {
        return game.empty()[fastrand::usize(0..maxmoves)];
    }
    let isx = game.turn() == Player::X;
    let mut besteval = if isx { isize::MIN } else { isize::MAX };
    let getbesteval = if isx {
        cmp::max::<isize>
    } else {
        cmp::min::<isize>
    };
    let mut evals = Vec::new();
    for &mv in game.empty() {
        let mut game = game.clone();
        game.play(mv.0, mv.1).unwrap();
        let eval = minimax(&game, depth - 1, isize::MIN, isize::MAX);
        evals.push((eval, mv));
        besteval = getbesteval(besteval, eval);
    }
    let best_moves: Vec<_> = evals
        .into_iter()
        .filter(|&(eval, _)| eval == besteval)
        .map(|(_, mv)| mv)
        .collect();
    best_moves[fastrand::usize(0..best_moves.len())]
}

fn minimax(game: &Game, depth: usize, mut alpha: isize, mut beta: isize) -> isize {
    let state = game.state();
    if depth == 0 {
        return if fastrand::bool() { 1 } else { -1 };
    }
    if let GameState::Decisive(w) = state {
        return w.into();
    }
    if state == GameState::Tied {
        return 0;
    }
    if game.turn() == Player::X {
        // maximizing player
        let mut eval = isize::MIN;
        for &(row, col) in game.empty() {
            let mut game = game.clone();
            game.play(row, col).unwrap();
            eval = cmp::max(eval, minimax(&game, depth - 1, alpha, beta));
            alpha = cmp::max(alpha, eval);
            if beta <= alpha {
                break;
            }
        }
        eval
    } else {
        // minimizing player
        let mut eval = isize::MAX;
        for &(row, col) in game.empty() {
            let mut game = game.clone();
            game.play(row, col).unwrap();
            eval = cmp::min(eval, minimax(&game, depth - 1, alpha, beta));
            beta = cmp::min(beta, eval);
            if alpha >= beta {
                break;
            }
        }
        eval
    }
}
