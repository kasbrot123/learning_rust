use std::env;
use std::process;



fn diagram(width: usize, height: usize) {

    let padding_top: i32 = 2;
    let padding_bottom: i32 = 2;
    let padding_left: i32 = 2;
    let padding_right: i32 = 2;
    let diagram_width: i32 = width - padding_left - padding_right;
    let diagram_height: i32 = height - padding_top - padding_bottom;
    // let diagram = [mut [mut char, x], y];

    // Base 1d array
    let mut grid_raw = vec![0; width * height];

    // Vector of 'width' elements slices
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();

    // Final 2d array `&mut [&mut [_]]`
    let grid = grid_base.as_mut_slice();

    // // Accessing data
    // grid[0][0] = 4;

    diagram[padding_top+1, padding_left+1] = '^';
    for i in (padding_top+2)..(height-padding_bottom-1) {
        grid[i][padding_left+1] = '|'
    }

    for i in 0..height {
        for j in 0..width {
            print!(grid[i][j]);
        }
        println!();
    }
    // for i in 0..padding_top {
    //     println!();
    // }
    // for i in 0..padding_left {
    //     print!(" ");
    // }
    // println!("^");
    // for 
    // for i in 

}


fn main() {


    let mut n: i32 = 10;
    let mut field: char = 'x';
    let empty: char = ' ';
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    // parse input arguments
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        x = args[1].parse().unwrap();
        y = args[2].parse().unwrap();
    } else {
        process::exit(1);
    }

    diagram(x, y);

    // for i in 1..=n {
    //     for _j in 1..=n {
    //         if (i % 2) == 1 {
    //             print!("{}", field);
    //             print!("{}", empty);
    //         } else {
    //             print!("{}", empty);
    //             print!("{}", field);
    //         }
    //     }
    //     println!();
    // }
    //
}
