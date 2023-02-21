struct Square {
    size: u8,
    char: String,
}

impl Square {
    fn draw_char(&self, ln: bool) {
        if ln {
            println!("{} ", &self.char)
        } else {
            print!("{} ", &self.char)
        }
    }

    fn draw_blank(&self) {
        print!("  ")
    }
}

pub fn draw(size: u8, char: String) {
    let square = Square { size, char };

    for x in 1..=size {
        square.draw_char(false);

        for y in 1..=square.size {
            match x {
                1 => square.draw_char(false),
                _ if x == square.size => square.draw_char(false),
                _ => square.draw_blank(),
            };

            if y == square.size {
                square.draw_char(true)
            }
        }
    }
}
