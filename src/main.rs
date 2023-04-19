use std::io;
fn main() {
    let arr = [1, 23, 4, 5, 6, 7];

    let mut index = String::new();
    let  ele;
    println!("enter the number ");

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    if index < arr.len() - 1 {
        ele = arr[index];
        println!("the element is {}", ele);
    } else {
        println!("out of boundaryyy");
    }
}
