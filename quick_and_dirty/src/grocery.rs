use std::io;

const MAX_LIST_ITEMS: u8 = 6;

pub fn list(){
    let mut items = Vec::new();
    let mut count = String::new();

    println!("How many items are in your list?");
    io::stdin().read_line(&mut count).expect("unable to read item count.");

    let count: usize = count.trim().parse().expect("please enter an integer.");
    if count > (MAX_LIST_ITEMS as usize) {
        println!("Sorry, the max list size is 6.");
        return
    }

    for _ in 0..count {
        let mut item = String::new();
        io::stdin().read_line(&mut item).expect("unable to read guess.");
        items.push(item);
    }

    println!("\nYour list:");
    // for idx in 0..count {
    //     let len = (items[idx].len())-1;
    //     let sub = &items[idx][..len];
    //     println!("{}", sub);
    // }
    println!("{:?}", items);
    return items;
}