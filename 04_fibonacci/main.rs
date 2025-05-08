use std::env;
// use std::process;




fn fib(n: i64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n-1) + fib(n-2),
    }
}






fn main() {

    let args: Vec<String> = env::args().collect();

    // if args.len() != 2 {
    //     println!("You have to specify a number.");
    // } else {
    //     let n = args[1].parse().unwrap();
    //     let result = fib(n);
    //     println!("fib({}) = {}", n, result);
    // }

    let result = match args.len() {
        2 => fib(args[1].parse().unwrap()),
        _ => panic!("Integer n not specified"),
        // _ => Err("Integer n not specified"),
    };
    println!("result: {}", result);

    // match args[1].parse().unwrap() {
    //     Ok(n) => println!("fib({}) = {}", n, fib(n)),
    //     Err(_) => println!("Integer n not specified"),
    // };


}
