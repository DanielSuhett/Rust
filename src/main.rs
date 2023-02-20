mod guess;
mod square;

fn main() {
    guess::new_game();
    square::draw(6, 12, "#");
}
