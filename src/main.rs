mod area;
mod guess;
mod square;
mod explore_enum;
mod two_sum;



fn main() {
    // guess::new_game();
    // square::draw(32, String::from("#"));
    // area::calculate(10, 2);
    two_sum::two_sum([2,7,11,15].to_vec(), 9);
}
