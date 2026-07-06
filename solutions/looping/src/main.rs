use std::io;

fn main() {
    let mut count = 0;
    loop {
        println!(
            "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?"
        );
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        count += 1;
        if answer.trim() == "The letter e" {
            break;
        }
    }
    println!("Number of trials: {}", count);
}
