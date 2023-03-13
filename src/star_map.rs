use std::collections::HashMap;

use rand::rngs::ThreadRng;

use crate::util::*;

/// Index into starmap.
pub type RowCol = (usize,usize);

// #[derive(Debug)]
pub struct StarSystem {
    size: isize,
    atmosphere: isize,
    hydrographics: isize,
    population: isize,
    government: isize,
    law_level: isize,
    technology: isize,
    starport: char
}

// ORDERED list of starport die throws, 2-11.  Will be binary searched.
const STARPORTS: &[(usize, char)] = &[
    (2, 'X')
    , (3, 'E')
    , (4, 'E')
    , (5, 'D')
    , (6, 'D')
    , (7, 'C')
    , (8, 'C')
    , (9, 'B')
    , (10, 'B')
    , (11, 'A')
    ];

// See:
//   https://www.traveller-srd.com/core-rules/world-creation/
//   https://www.reddit.com/r/traveller/comments/o72ca1/world_creation_rules_on_one_page/

impl StarSystem {
    pub fn generate(rng: &mut ThreadRng) -> StarSystem {
        static starports: Vec<(usize, char)> = Vec::new();

        // With luck, this will only execute once.
        if starports.len() == 0 {
            for t in STARPORTS {
                starports.push( *t);
            }
        }

        let size = dice(2, -2, rng);
        let atmo = clamp( dice(2, -7 + size, rng), 0, 15);
        let hydro = clamp(
            match (size, atmo) {
                (0, _) | (1, 0) => 0,
                (_, 0) | (_, 1) | (_, 10) | (_, 11) | (_, 12) => dice( 2, -7 + size - 4, rng),
                _ => dice( 2, -7 + size, rng)
            },
            0, 10);
        let pop = dice( 2, -2, rng);
        let govt = clamp( dice( 2, -7 + pop, rng), 0, 13);
        let law = clamp( dice( 2, -7 + govt, rng), 0, 15);
        let starport = 
            starports[
            starports.binary_search_by_key(
                 &(clamp( dice(2, 0, rng), 2, 11) as usize),
                 | &(n,c)| n).unwrap()].1;
        // And finally...
        let tech = clamp( dice(1, 0, rng)
            + match starport {
                'A' => 6,
                'B' => 4,
                'C' => 2,
                _ => 0
                }
            + match size {
                0 | 1 => 2,
                2..=4 => 1
                }
            + match atmo {
                0..=3 => 1,
                10..=15 => 1,
                _ => 0
                }
            + match hydro {
                0 => 1,
                9 => 1,
                10 => 2,
                _ => 0
                }
            + match pop {
                1..=5 => 1,
                9 => 1,
                10 => 2,
                11 => 3,
                12 => 4,
                _ => 0
                }
            + match govt {
                0 | 5 => 1,
                7 => 2,
                13 | 14 => -2,
                _ => 0
                }
            , 0, 99);       // clamp min, max
            
        return StarSystem {
            size
            , atmosphere: atmo
            , hydrographics: hydro
            , population: pop
            , government: govt
            , law_level: law
            , technology: tech
            , starport
        }
    }
    
    /// Universal World Profile
    pub fn uwp(&self) -> String {
        return format!( "{}-{}{}{}{}{}{}-{}"
            , self.starport
            , self.size
            , self.atmosphere
            , self.hydrographics
            , self.population
            , self.government
            , self.law_level
            , self.technology
        );
    }
}

/// Index into starmap.
// #[derive(PartialEq, Eq, Hash)]
// struct RowCol {
//     row: usize,
//     col: usize
// }

/// Map from RowCol to characteristic string for star system at that location.
pub type StarMap = HashMap<RowCol, StarSystem>;

