pub mod d1;
pub mod d2;

pub fn run_day(day: u32) {
    match day {
        1 => d1::run(),
        2 => d2::run(),
        day => eprintln!("Unimplemented day {}", day),
    }
}
