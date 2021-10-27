#[path = "pawn.rs"]
pub mod pawn;

pub struct Game {
    pub password: [pawn::Pawn; 4],
    try_counter: i32,
    state: bool,
}

impl Game {
    pub fn new(
        first_color: pawn::PawnColor,
        second_color: pawn::PawnColor,
        third_color: pawn::PawnColor,
        fourth_color: pawn::PawnColor,
    ) -> Game {
        Game {
            password: [
                pawn::Pawn::new(first_color, 0),
                pawn::Pawn::new(second_color, 1),
                pawn::Pawn::new(third_color, 2),
                pawn::Pawn::new(fourth_color, 3),
            ],
            try_counter: 0,
            state: true,
        }
    }

    pub fn is_lost(&mut self, try_counter: i32) -> bool {
        if try_counter > 10 {
            self.state = false;
        } else {
            self.state = true;
        }

        self.state
    }
}
