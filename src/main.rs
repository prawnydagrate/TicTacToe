use std::time::{Duration, Instant};
use tic_tac_toe::{
    get_best_move,
    mech::{Game, GameState},
};

fn main() {
    let (n, depth) = (7, 3);
    let mut game = Game::new(n);
    let mut mv = 0;
    let mut max_dur = Duration::from_nanos(0);
    while game.state() == GameState::Ongoing {
        mv += 1;
        println!("MOVE {mv}");
        let start = Instant::now();
        let (r, c) = get_best_move(&game, depth);
        let end = Instant::now();
        let dur = end.duration_since(start);
        max_dur = max_dur.max(dur);
        game.play(r, c).unwrap();
        println!(
            "BEST MOVE TOOK {}s ({n}x{n} grid, depth {depth})",
            dur.as_secs_f64(),
        );
        println!("{}\n", game.grid());
    }
    println!("RESULT: {:?}", game.state());
    println!("MAX DURATION: {}s", max_dur.as_secs_f64());
}
