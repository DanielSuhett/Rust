

pub fn vec() {
    let list = vec![1, 33, 3, 4, 5];

    let second = &list[1];
    
    println!("{}", second);

    let second = &list.get(1);

    match second {
        None => println!(""),
        Some(item) => println!("{}", item),
    };


    let text = String::from("Testing the string");


    println!("{}", &text[0..1]);
}
