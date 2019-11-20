use std::io;

const MAX_LIST_ITEMS: usize = 6;

pub fn list() -> Vec<String>{
    let mut items = Vec::new();
    let mut read_in_str = String::new();
    let mut count: usize = MAX_LIST_ITEMS + 1;

    while count > MAX_LIST_ITEMS {
        println!("How many items are in your list? (Max size is {})", MAX_LIST_ITEMS);
        io::stdin().read_line(&mut read_in_str).expect("unable to read item count.");

        count = read_in_str.trim().parse().expect("please enter an integer.");
    }

    for _ in 0..count {
        let mut item = String::new();
        io::stdin().read_line(&mut item).expect("unable to read guess.");
        items.push(item.trim().to_string());
    }
    return items;
}