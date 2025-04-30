use std::env;



fn main() {

    let mut n: i32 = 10;
    let mut field: char = '█';
    let empty: char = ' ';

    // parse input arguments
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        n = args[1].parse().unwrap();
    }
    if args.len() > 2 {
        field = args[2].parse().unwrap();
    }


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

}
