use core::num;

use rand::prelude::*;

const ROW_MAX: i32 = 8;
const COL_MAX: i32 = 10;

fn main() {
    let mut rng = thread_rng();
    for i in 1..=6 {
        print!( "{:4}: {:<6}", i, rng.gen_range( 1..=6));
    }
    println!();
    println!( "1<<15 = {}", 1<<15);
    generate_systems( &mut rng);
    draw_grid(8, 10, 5, 3, &mut rng);
}

/// Draw a hex grid
fn draw_grid(num_rows: usize, num_cols: usize, a_horizontal_length: usize, a_diagonal_length: usize, mut rng: &mut ThreadRng) {

    enum DrawState {
        /// Currently drawing top edges of hexes
        TopEdge,

        /// Current drawing diagonals of top halves of (leftmost) hexes
        DiagonalsTop{ 
            /// Which row number are we currently drawing (0-based)?
            diag_row_num: usize
        },

        /// Currently drawing the middles of each hex (includes row headers, star symbol (if star is present), bottom
        /// edges of neighboring hexes).
        Middles,

        /// Current drawing diagonals of bottom halves of (leftmost) hexes
        DiagonalsBottom{ 
            /// Which row number are we currently drawing (0-based)? Reset to zero when transitioning from top to middle
            /// to bottom.
            diag_row_num: usize
        },

        /// Currently the bottom edges of hexes
        BottomEdge
    }

    draw_column_headers(num_cols, a_diagonal_length, a_horizontal_length);
    draw_very_top_edge(num_cols, a_diagonal_length, a_horizontal_length);

    let mut draw_state = DrawState::DiagonalsTop { diag_row_num: 0 };
    for row in 1..=num_rows {
        // Diagonals up, diagonals down: *2
        for i in 0.. 2 * a_diagonal_length {
            match draw_state {
                DrawState::DiagonalsTop { diag_row_num } => {
                    draw_top_hex_halves( a_diagonal_length, a_horizontal_length, diag_row_num, num_cols, row == 1);
                    if diag_row_num >= a_diagonal_length-2 {        // -2 because the last "top diagonal" row is really the middle row
                        draw_state = DrawState::Middles;
                    }
                    else {
                        draw_state = DrawState::DiagonalsTop { diag_row_num: diag_row_num+1 };
                    }
                }
                DrawState::Middles => {
                    draw_hex_middles( row, num_cols, a_diagonal_length, a_horizontal_length, row == 1, &mut rng);
                    draw_state = DrawState::DiagonalsBottom { diag_row_num: 0 };
                }
                DrawState::DiagonalsBottom { diag_row_num } => {
                    draw_bottom_hex_halves( a_diagonal_length, a_horizontal_length, diag_row_num, num_cols);
                    if diag_row_num >= a_diagonal_length-2 {        // -2 because the last "bottom diagonal" row is really the bottoms of (leftmost) hexes.
                        draw_state = DrawState::BottomEdge;
                    }
                    else {
                        draw_state = DrawState::DiagonalsBottom { diag_row_num: diag_row_num+1 };
                    }
                }
                DrawState::BottomEdge => {
                    draw_hex_bottoms( num_cols, a_diagonal_length, a_horizontal_length);
                    draw_state = DrawState::DiagonalsTop { diag_row_num: 0 };
                }
                _ => {unreachable!()}
            }
            println!();     // Tie off the row.
        }
        // break;
    }
}

fn draw_column_headers(num_cols: usize, a_diagonal_length: usize, a_horizontal_length: usize) {
    print!( "    "); // Width of row labels + margin (2 digits, 2 spaces).
    // Header: column numbers
    print!( "{:width$}{:02}", "", 1, width = a_diagonal_length + (a_horizontal_length-2)/2);    // -2 to account for 2-digit number
    for col in 2..=num_cols {
        print!( "{:trail_width$}{:02}", "", col
            ,trail_width = a_horizontal_length - 2 - (a_horizontal_length-2)/2 + a_diagonal_length + (a_horizontal_length-2)/2
            // ,width = a_diagonal_length + a_horizontal_length/2
        );
        // print!( "{:width$}", " ", width= a_diagonal_length + a_horizontal_length/2 );
        // print!( "{:02}", 2*col-1);
        // print!( "{:width$}", " ", width= a_horizontal_length/2 + a_diagonal_length + a_horizontal_length/2);
        // print!( "{:02}", 2*col);
    }
    println!();
}

fn draw_very_top_edge(num_cols: usize, a_diagonal_length: usize, a_horizontal_length: usize) {
    // Special case: very top edge
    print!( "    ");
    // row headers plus 2-space margin
    for c in 0..num_cols/2 {
        print!( "{:diag_width$}{:_<horiz_width$}{:diag_width$}{:horiz_width$}", "", "", "", ""
            ,diag_width = a_diagonal_length   // space to account for width of diagonal
            ,horiz_width = a_horizontal_length);
    }
    println!();
}

fn draw_top_hex_halves(a_diag_length: usize, a_horiz_width: usize, diag_row_num: usize, num_cols: usize, is_first_row: bool) {
    print!( "    ");         // row header
    for i in 0..num_cols/2 {
        print!( "{:>diag_width$}{:<center1_width$}{:<diag_width$}{:<center2_width$}", "/", "", "\\", ""
            ,diag_width = a_diag_length - diag_row_num
            ,center1_width = a_horiz_width + diag_row_num*2     // *2 because need to expand center on BOTH sides.
            ,center2_width = a_horiz_width
        );
        // Oops, following is for hex top edges, already drawn.
        // print!( "{:diag_width$}{:_<horiz_width$}{:diag_width$}{:horiz_width$}", "", "", "", ""
        //     , diag_width = a_diag_length, horiz_width = a_horiz_width);
    }
    if ! is_first_row {
        print!("{:diag_width$}{}", "", "/"       // Trailing "/" for all but first row
        ,diag_width = a_diag_length - diag_row_num - 1      // -1 for "/" char
        );
    }
}

fn draw_hex_middles(row: usize, num_cols: usize, a_diag_length: usize, a_horizontal_length: usize, is_first_row: bool, rng: &mut ThreadRng) {
    print!( "{:02}  ", row);
    for i in 0..num_cols/2 {
        if rng.gen_range(1..=6) < 4 {
            print!( "{}{:<space_width$}{}{:_<edge_width$}", "/", "", "\\", ""
                ,space_width = a_horizontal_length + 2*(a_diag_length - 1)      // 2* for both sides, -1 for '/' char
                ,edge_width = a_horizontal_length
            );
        }
        else {
            let starchar = if a_diag_length <= 2 {"o"} else {"O"};      // big hexes ==> bigger symbol, for looks
            print!( "{}{:<space_width$}{}{:<space_width$}{}{:_<edge_width$}", "/", "", starchar, "", "\\", ""
                ,space_width = (a_horizontal_length - 1)/2 + a_diag_length - 1      // -1 for starchar, /2 to split in half (both sides)
                ,edge_width = a_horizontal_length
            );
        }
    }
    if ! is_first_row {
        print!( "/");       // Trailing "/"
    }
}

fn draw_bottom_hex_halves(a_diag_length: usize, a_horiz_width: usize, diag_row_num: usize, num_cols: usize) {
    print!( "    ");         // row header
    for i in 0..num_cols/2 {
        print!( "{:>diag_width$}{:<center1_width$}{}{:<center2_width$}", "\\", "", "/", ""
            ,diag_width = diag_row_num + 1
            ,center1_width = a_horiz_width + (a_diag_length - diag_row_num - 1)*2  // -1 for char, *2 for both sides
            ,center2_width = a_horiz_width + 2*diag_row_num - diag_row_num
        );
    }
    print!("{:>diag_width$}", "\\"       // Trailing "\"
        ,diag_width = diag_row_num + 1
    );
}

fn draw_hex_bottoms(num_cols: usize, a_diag_width: usize, a_horiz_width: usize) {
    print!( "    ");         // row header
    for i in 0..num_cols/2 {
        print!("{:>diag_width$}{:_<a_horiz_width$}{:<diag_width$}{:<a_horiz_width$}", "\\", "", "/", ""
            ,diag_width = a_diag_width
        );
    }
    print!("{:>diag_width$}", "\\"       // Trailing "\"
        ,diag_width = a_diag_width
    );
}

/// Generate star systems.
fn generate_systems(rng: &mut ThreadRng) {
    for row in 1..ROW_MAX {
        println!( "row {}", row);
        for col in 1..COL_MAX {
            if rng.gen_range(1..=6) >= 4 {
                println!( "  col {}", col);
            }
        }
    }
}
