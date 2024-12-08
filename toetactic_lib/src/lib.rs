pub mod mech;

use mech::{Game, GameState, Move, Player};
use std::cmp;

/// Generates the best move in the position, **assuming** the game
/// is ongoing. This function **will panic** if the game has already
/// ended.
///
/// `depth` is the number of plies to search, not the number of fullmoves.
pub fn get_best_move(game: &Game, depth: usize) -> Move {
    let maxmoves = game.grid().n().pow(2);
    if game.empty().len() == maxmoves {
        return game.empty()[fastrand::usize(0..maxmoves)];
    }
    let obv = game.undoubted();
    if let Some((_, mv)) = obv {
        return mv;
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
        game.play(mv).unwrap();
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
    if let GameState::Decisive(w) = state {
        return w.into();
    }
    if state == GameState::Tied || depth == 0 {
        return 0;
    }
    let obv = game.undoubted();
    if game.turn() == Player::X {
        // maximizing player
        if let Some((Player::X, _)) = obv {
            return 1;
        }
        let mut eval = isize::MIN;
        for &mv in game.empty() {
            let mut game = game.clone();
            game.play(mv).unwrap();
            eval = cmp::max(eval, minimax(&game, depth - 1, alpha, beta));
            alpha = cmp::max(alpha, eval);
            if beta <= alpha {
                break;
            }
        }
        eval
    } else {
        // minimizing player
        if let Some((Player::O, _)) = obv {
            return -1;
        }
        let mut eval = isize::MAX;
        for &mv in game.empty() {
            let mut game = game.clone();
            game.play(mv).unwrap();
            eval = cmp::min(eval, minimax(&game, depth - 1, alpha, beta));
            beta = cmp::min(beta, eval);
            if alpha >= beta {
                break;
            }
        }
        eval
    }
}
