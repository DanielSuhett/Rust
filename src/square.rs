pub fn draw(cols: u8, rows: u8, print: &str) {
    for x in 1..=rows {
        print!("{} ", print);

        for y in 1..=cols {
            match x {
                1 => print!("{} ", print),
                _ if x == rows => print!("{} ", print),
                _ => print!("  "),
            };

            if y == cols {
                println!("{}", print)
            }
        }
    }
}
