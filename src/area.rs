struct Retangle {
    width: i32,
    heigth: i32,
}

pub fn calculate(width: i32, heigth: i32) {
    let instance = Retangle { width, heigth };

    println!(
        "Area for retangle with {} width and {} heigh is: {}",
        instance.width,
        instance.heigth,
        area(&instance)
    )
}

fn area(retangle: &Retangle) -> i32 {
    retangle.width * retangle.heigth
}
