use std::arch::x86_64::_rdrand16_step;

const ROW_MAX: i32 = 8;
const COL_MAX: i32 = 10;

fn main() {
    draw_grid(8, 10, 3, 2);
    // println!( "1<<15 = {}", 1<<15);
    // for row in 1..ROW_MAX {
    //     println!( "row {}", row);
    //     for col in 1..COL_MAX {
    //         let mut t: u16 = 0;
    //         unsafe {if _rdrand16_step(&mut t) == 0 { 
    //             panic!("unable to generate random number from hardware")}
    //         }
    //         if t < 1<<15 {
    //             println!( "  col {}: {}", col, t);
    //         }
    //     }
    // }
}

/// Draw a hex grid
fn draw_grid(num_rows: usize, num_cols: usize, a_horizontal_length: usize, a_diagonal_length: usize) {
    print!( "    "); // Width of row labels + margin (2 digits, 2 spaces).
    for col in 1..num_cols/2 {
        print!( "{:width$}", " ", width= a_diagonal_length + a_horizontal_length/2 );
        // print!( " ".repeat( a_diagonal_length       // Diagonal length == horizontal width of side
        //     + a_horizontal_length/2                 // Center over horizontal side
        //     - 1) );                                 // Minus half width of two-digit coordinate
        print!( "{:02}", 2*col-1);
        print!( "{:width$}", " ", width= a_horizontal_length/2 + a_diagonal_length + a_horizontal_length/2);
        // print!( " ".repeat( a_horizontal_length/2 + a_diagonal_length + a_horizontal_length/2 - 1));
        print!( "{:02}", 2*col);
    }
    println!();
    for row in 1..num_rows {
        print!( "{:02}  ", row);
        for col in 1..num_cols {
            
        }
        println!();
    }
}