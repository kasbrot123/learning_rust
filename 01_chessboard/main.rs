use std::env;



fn main() {

    let n: i32 = 10;
    // for argument in env::args() {
    //     n = argument;
    //     break
    // }

    let field: char = 'x';
    let empty: char = ' ';

    for i in 1..=n {
        for _j in 1..=n {
            if (i % 2) == 1 {
                print!("{}", field);
                print!("{}", empty);
            } else {
                print!("{}", empty);
                print!("{}", field);
            }
        }
        println!();
    }

    // println!("Hello world");
    //
    // let mut number_input = String::new();
    //
    // io::stdin().read_line(&mut number_input).expect("Fail to read line");
    //
    // println!("this is the input: {}", number_input);
    // println!("Length: {}", number_input.len());

}
