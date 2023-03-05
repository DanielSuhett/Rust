use std::collections::HashMap;

pub fn hash_map () {
    let mut dictonary = HashMap::new();
    let key = String::from("Texting");
    
    dictonary.insert(&key, 50);
    
    let second_key = String::from("Team B");
   
    dictonary.insert(&second_key, 10);

    println!("{:?}", dictonary);

    let find = dictonary.get(&key).copied().unwrap();
    
    println!("{:?}", find);

    for (key, value) in &dictonary {
        println!("{key} - {value}");
    };

    for word in "Texting or testing".split_whitespace () {
        let count = dictonary.entry(&key).or_insert(0);
        *count += 1;
    }


    println!("{:?}", dictonary);
}

