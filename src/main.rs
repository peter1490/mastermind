#[path = "lib/game.rs"]
mod game;

fn main() {
    println!("Hello, world!");
    let new_game = game::Game::new(
        game::pawn::PawnColor::Black,
        game::pawn::PawnColor::Green,
        game::pawn::PawnColor::Orange,
        game::pawn::PawnColor::Orange,
    );

    println!("{:?}", new_game.password);
}
