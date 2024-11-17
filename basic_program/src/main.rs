use std::io;

const THREE_MONTH_IN_SECONDS: u32 = 60 * 60 * 3;
fn main() {
    println!("{}", THREE_MONTH_IN_SECONDS);
    let arr = [1,2,3,4,5,6,7,8,9,10];

    loop {
        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index.trim().parse().expect("Please type a number!");

        println!("{}", arr[index]);
    }
}
