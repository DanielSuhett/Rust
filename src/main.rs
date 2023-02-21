mod guess;
mod square;
mod area;

fn main() {
    guess::new_game();
    square::draw(6, 12, "#");
    area::calculate(10, 2)
}
