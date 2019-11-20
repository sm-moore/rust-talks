#![crate_name = "quick_and_dirty"]

mod merge;
mod grocery;

fn main() {
    let grocery_list = grocery::list();
    println!("Original list: {:?}", grocery_list);
    let result = merge::merge_sort(grocery_list, 5);
    println!(" Sorted  list: {:?}", result);
}






























// let my_vec = vec!(1, 5, 2, 3);
// let len = my_vec.len();
// let result = merge::merge_sort(my_vec, len);
// println!(" Sorted  list: {:?}", result);


// let my_vec = vec![1, 5, 7, 2]; // Note that I'm not being explicit about the type here.
// //   let my_vec: Vec<String> = vec![...]
//  // let my_vec = Vec::new(); my_vec.push(1) ...

// let sorted_list: Vec<i32> = merge::merge_sort(my_vec.clone(), my_vec.len()); // <--- this is why rust makes you a better programer
// println!("{:?}", sorted_list); // String formatters, map over args and print debug version.