fn main() {
    println!("Hello, world!");
    print_with_while_loop_n_times(255)
}

fn print_with_while_loop_n_times(number: u8) {
    let mut count = 0;
    while count < number {
        println!("count: {}", count);
        count = count + 1;
    }
}
