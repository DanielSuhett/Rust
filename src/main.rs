mod area;
mod guess;
mod square;

fn main() {
    guess::new_game();
    square::draw(32, String::from("#"));
    area::calculate(10, 2)
}
