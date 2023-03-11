use std::arch::x86_64::_rdrand16_step;

const ROW_MAX: i32 = 8;
const COL_MAX: i32 = 10;

fn main() {
    println!( "1<<15 = {}", 1<<15);
    for row in 1..ROW_MAX {
        println!( "row {}", row);
        for col in 1..COL_MAX {
            let mut t: u16 = 0;
            unsafe {if _rdrand16_step(&mut t) == 0 { 
                panic!("unable to generate random number from hardware")}
            }
            if t < 1<<15 {
                println!( "  col {}: {}", col, t);
            }
        }
    }
}