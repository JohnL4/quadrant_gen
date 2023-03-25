use std::{collections::HashMap};

use clap::{Parser, ValueEnum};
use rand::prelude::*;

use quadrant_gen::star_map::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of rows in map.  Conventional values are 10, 20, 14.
    #[arg(short, long, default_value_t = 10)]
    rows: usize,

    /// Number of columns in map.  Conventional values are 8, 16, 32.
    #[arg(short, long, default_value_t = 8)]
    cols: usize,

    /// Axis style.
    #[arg( short, long, value_enum, default_value_t = AxisStyle::XY )]
    axis_style: AxisStyle,

    /// DM (die modifier) to be applied to the d6 throw to determine whether a system is present in a hex.  The default
    /// (0) will result in a 50% chance of a star system being present.  -2 ==> rift sector; -1 ==> sparse sector;
    /// +1 ==> dense sector.
    #[arg(short, long, default_value_t = 0)]
    world_chance_die_modifier: isize,

    /// Number of characters in horizontal hex cell.
    #[arg(short('e'), long, default_value_t = 5)]
    horizontal_edge_length: usize,

    /// Number of characters in diagonal hex cell.
    #[arg(short('d'), long, default_value_t = 2)]
    diagonal_edge_length: usize
}

#[derive( Copy, Clone, Debug, ValueEnum)]
enum AxisStyle {
    /// The first two characters are the row number and the last two characters are the column number.
    RowCol,

    /// The first two characters are the x-coordinate (column number), and the last two characters are the y-coordinate
    /// (row number)
    XY
}

fn main() {

    // General implementation note: my initial implementation of this assumed hexes laid out in rows and columns, and
    // addressed that way, meaning the first two characters of the coordinates (upon display) are the ROW number, and
    // the last two characters are the COLUMN number.  Only after I was done did I realize (I had forgotten) that the
    // standard coordinate system is transposed, and it's more like x-y coordinates (with the y-axis reversed).  So, I
    // hacked in a fix for that, but the concept in this code is still row-column.

    let args = Args::parse();
    let mut rng = thread_rng();
    let mut starmap: StarMap = HashMap::new();

    draw_grid( args.rows, args.cols, args.world_chance_die_modifier, args.horizontal_edge_length, args.diagonal_edge_length, &mut rng, &mut starmap);
    generate_systems( &mut starmap, &mut rng, args.rows, args.cols, args.axis_style);
}

/// Draw a hex grid
fn draw_grid(num_rows: usize, num_cols: usize, density_dm: isize, a_horizontal_length: usize, a_diagonal_length: usize, mut rng: &mut ThreadRng, mut starmap: &mut StarMap) {

    enum DrawState {
        /// Currently drawing top edges of hexes
        TopEdge,

        /// Current drawing diagonals of top halves of (leftmost) hexes
        DiagonalsUpper{ 
            /// Which row number are we currently drawing (0-based)?
            diag_row_num: usize
        },

        /// Currently drawing the middles of each hex (includes row headers, star symbol (if star is present), bottom
        /// edges of neighboring hexes).
        Middles,

        /// Current drawing diagonals of bottom halves of (leftmost) hexes
        DiagonalsLower{ 
            /// Which row number are we currently drawing (0-based)? Reset to zero when transitioning from top to middle
            /// to bottom.
            diag_row_num: usize
        },

        /// Currently the bottom edges of hexes
        BottomEdge
    }

    draw_column_headers(num_cols, a_diagonal_length, a_horizontal_length);
    draw_very_top_edge(num_cols, a_diagonal_length, a_horizontal_length);

    let mut draw_state = DrawState::DiagonalsUpper { diag_row_num: 0 };
    for row in 1..=num_rows + 1 {
        // Diagonals up, diagonals down: *2
        for i in 0.. 2 * a_diagonal_length {
            match draw_state {
                DrawState::DiagonalsUpper { diag_row_num } => {
                    draw_top_hex_halves( a_diagonal_length, a_horizontal_length, diag_row_num, num_cols, row == 1, row == num_rows+1);
                    if diag_row_num >= a_diagonal_length-2 {        // -2 because the last "top diagonal" row is really the middle row
                        draw_state = DrawState::Middles;
                    }
                    else {
                        draw_state = DrawState::DiagonalsUpper { diag_row_num: diag_row_num+1 };
                    }
                }
                DrawState::Middles => {
                    draw_hex_middles( row, num_cols, density_dm, a_diagonal_length, a_horizontal_length, row == 1, row == num_rows+1, &mut rng, &mut starmap);
                    draw_state = DrawState::DiagonalsLower { diag_row_num: 0 };
                }
                DrawState::DiagonalsLower { diag_row_num } => {
                    if row <= num_rows {draw_bottom_hex_halves( a_diagonal_length, a_horizontal_length, diag_row_num, num_cols);}
                    if diag_row_num >= a_diagonal_length-2 {        // -2 because the last "bottom diagonal" row is really the bottoms of (leftmost) hexes.
                        draw_state = DrawState::BottomEdge;
                    }
                    else {
                        draw_state = DrawState::DiagonalsLower { diag_row_num: diag_row_num+1 };
                    }
                }
                DrawState::BottomEdge => {
                    if row <= num_rows { draw_hex_bottoms( row, num_cols, density_dm, a_diagonal_length, a_horizontal_length, &mut rng, &mut starmap); }
                    draw_state = DrawState::DiagonalsUpper { diag_row_num: 0 };
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

fn draw_top_hex_halves(a_diag_length: usize, a_horiz_width: usize, diag_row_num: usize, num_cols: usize, is_first_row: bool, is_last_row: bool) {
    print!( "    ");         // row header
    for i in 0..num_cols/2 {
        let leading_slash = if i == 0 && is_last_row {" "} else {"/"};
        print!( "{:>diag_width$}{:<center1_width$}{:<diag_width$}{:<center2_width$}", leading_slash, "", "\\", ""
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

fn draw_hex_middles(row: usize, num_cols: usize, density_dm: isize, a_diag_length: usize, a_horizontal_length: usize, is_first_row: bool, is_last_row: bool, rng: &mut ThreadRng, starmap: &mut StarMap) {
    if is_last_row {
        print!( "    ");
    }
    else {
        print!( "{:02}  ", row);
    }
    for i in 0..num_cols/2 {
        if is_last_row || rng.gen_range(1..=6) + density_dm < 4 {
            // No star system
            let leading_slash = if is_last_row && i == 0 {" "} else {"/"};
            print!( "{}{:<space_width$}{}{:_<edge_width$}", leading_slash, "", "\\", ""
                ,space_width = a_horizontal_length + 2*(a_diag_length - 1)      // 2* for both sides, -1 for '/' char
                ,edge_width = a_horizontal_length
            );
        }
        else {
            // Star system! Draw symbol.
            starmap.insert((row, 2*i+1), StarSystem::generate( rng));
            let starchar = if a_diag_length <= 2 {"o"} else {"O"};      // big hexes ==> bigger symbol, for looks
            print!( "{}{:<space1_width$}{}{:<space2_width$}{}{:_<edge_width$}", "/", "", starchar, "", "\\", ""
                ,space1_width = (a_horizontal_length - 1)/2 + a_diag_length - 1      // -1 for starchar, /2 to split in half (both sides)
                ,space2_width = (a_horizontal_length)/2 + a_diag_length - 1         // Dropped one "-1" to account for rounding when horiz_length is even or odd.
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

fn draw_hex_bottoms(row: usize, num_cols: usize, density_dm: isize, a_diag_width: usize, a_horiz_width: usize,  rng: &mut ThreadRng, starmap: &mut StarMap) {
    print!( "    ");         // row header
    for i in 0..num_cols/2 {
        if rng.gen_range(1..=6) + density_dm < 4 {
            // No star system.
            print!("{:>diag_width$}{:_<a_horiz_width$}{:<diag_width$}{:<a_horiz_width$}", "\\", "", "/", ""
                ,diag_width = a_diag_width
            );
        }
        else {
            // Star system! Draw symbol.
            starmap.insert((row, 2*(i+1)), StarSystem::generate( rng));
            let starchar = if a_diag_width <= 2 {"o"} else {"O"};      // big hexes ==> bigger symbol, for looks
            print!( "{:>diag_width$}{:_<a_horiz_width$}{:<diag_width$}{:center1_width$}{}{:center2_width$}"
                , "\\", "", "/", "", starchar, ""
                ,diag_width = a_diag_width
                ,center1_width = (a_horiz_width - 1)/2     // -1 for starchar, /2 to split in half (both sides)
                ,center2_width = (a_horiz_width)/2         // Dropped one "-1" to account for rounding when horiz_length is even or odd.
            );
        }
    }
    print!("{:>diag_width$}", "\\"       // Trailing "\"
        ,diag_width = a_diag_width
    );
}

/// Generate star systems.
fn generate_systems( starmap: &mut StarMap, rng: &mut ThreadRng, rows: usize, cols: usize, axis_style: AxisStyle) {
    match axis_style {
        AxisStyle::RowCol => {
            for row in 1..=rows {
                for col in 1..=cols {
                    print_star_system( row, col, starmap, axis_style);
                }
            }
        }
        AxisStyle::XY => {
            for col in 1..=cols {
                for row in 1..=rows {
                    print_star_system(row, col, starmap, axis_style)
                }
            }
        }
    }
}

fn print_star_system(row: usize, col: usize, starmap: &mut HashMap<(usize, usize), StarSystem>, axis_style: AxisStyle) {
    let hex = ( row, col);
    let entry = starmap.get( &hex);
    match entry {
        Some( starsys) => {
            match axis_style {
                AxisStyle::RowCol => {println!( "{:02}{:02}    {}", row, col, starsys.uwp())}
                AxisStyle::XY => {println!( "{:02}{:02}    {}", col, row, starsys.uwp())}
            }
        }
        None => {}
    }
}
