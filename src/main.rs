use tic_tac_toe::{
    get_best_move,
    mech::{Game, GameState},
};

fn main() {
    let mut game = Game::new(3);
    let mut mv = 0;
    while game.state() == GameState::Ongoing {
        mv += 1;
        println!("MOVE {mv}");
        let (r, c) = get_best_move(&game, usize::MAX);
        game.play(r, c).unwrap();
        println!("{}\n", game.grid());
    }
}
