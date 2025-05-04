use std::env;
use std::process;



fn create_diagram(width: i32, height: i32) {

    let padding_top: i32 = 2;
    let padding_bottom: i32 = 2;
    let padding_left: i32 = 2;
    let padding_right: i32 = 2;
    let diagram_width: i32 = width - padding_left - padding_right;
    let diagram_height: i32 = height - padding_top - padding_bottom;
    // let diagram = [mut [mut char, x], y];
    
    // let mut diagram: Vec<Vec<char>> = Vec::new();
    // Populate the 2D vector.
    // for i in 0..height {
    //     // Create a new row (a Vec of chars).
    //     let mut row: Vec<char> = Vec::new();
    //     for j in 0..width {
    //         // Add characters to the row.  Example: 'A', 'B', 'C', etc.
    //         // row.push(('A' as u8 + (i * num_cols + j) as u8) as char); // Fill with A, B, C, ...
    //         row.push(' ')
    //     }
    //     // Add the row to the matrix.
    //     diagram.push(row);
    // }
    let mut diagram: Vec<Vec<char>> = vec![vec![' '; width as usize]; height as usize]; // Initialize with spaces
                                                                   //
    diagram[(padding_top+1) as usize][(padding_left+1) as usize] = '^';
    diagram[(height-padding_top-1) as usize][(padding_left+1) as usize] = '+';
    diagram[(height-padding_top-1) as usize][(width-padding_right) as usize] = '>';

    for i in (padding_top+2)..(height-padding_bottom-1) {
        diagram[i as usize][(padding_left+1) as usize] = '|'
    }
    for i in (padding_left+2)..(width-padding_right) {
        diagram[(height-padding_top-1) as usize][i as usize] = '-'
    }

    for i in 0..height {
        for j in 0..width {
            print!("{}", diagram[i as usize][j as usize]);
        }
        println!();
    }

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

    create_diagram(x, y);

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
