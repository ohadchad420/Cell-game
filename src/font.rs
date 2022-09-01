/* 
pub enum Character {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, 
    One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero,
    Colon, Floating, Arrow, Space
} 
*/

//height = 550px
//RULESET = 1
//all 4 connections + radius control = 20
//speed = 1
//MISC = 1
//rand => rand + shuffle location (R KEY)
//save => save configuration
//load => load configuration
//total => 30 rows, 18px each.
//button layout top -> bot: 01100XXXXXXXX00110 => 8 px per letter

pub fn get_letter(c: char) -> Vec<Vec<bool>> {
    match c.to_ascii_uppercase() {
        'A' => {
            return vec![vec![false,false,true,true,true,true,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false]];
        }
        'B' => {
            return vec![vec![false,true,true,true,true,true,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,false,false]];
        }
        'C' => {
            return vec![vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false]];
        }
        'D' => {
            return vec![vec![false,true,true,true,true,true,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,false,false]];
        }
        'E' => {
            return vec![vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false]];
        }
        'F' => {
            return vec![vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,false,false,false,false,false]];
        }
        'G' => {
            return vec![vec![false,false,true,true,true,true,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,false,true,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,false,true,true,true,true,false,false]];
        }
        'H' => {
            return vec![vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false]];
        }
        'I' => {
            return vec![vec![false,false,true,true,true,true,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,true,true,true,true,false,false]];
        }
        'J' => {
            return vec![vec![false,false,false,true,true,true,true,false],
                        vec![false,false,false,false,true,true,false,false],
                        vec![false,false,false,false,true,true,false,false],
                        vec![false,false,false,false,true,true,false,false],
                        vec![false,true,true,false,true,true,false,false],
                        vec![false,true,true,false,true,true,false,false],
                        vec![false,true,true,false,true,true,false,false],
                        vec![false,false,true,true,true,false,false,false]];
        }
        'K' => {
            return vec![vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,true,true,true,false],
                        vec![false,true,true,true,true,true,false,false],
                        vec![false,true,true,true,true,true,false,false],
                        vec![false,true,true,true,true,true,false,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false]];
        }
        'L' => {
            return vec![vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false]];
        }
        'M' => {
            return vec![vec![true,true,false,false,false,false,true,true],
                        vec![true,true,true,false,false,true,true,true],
                        vec![true,true,true,true,true,true,true,true],
                        vec![true,true,true,true,true,true,true,true],
                        vec![true,true,false,true,true,false,true,true],
                        vec![true,true,false,false,false,false,true,true],
                        vec![true,true,false,false,false,false,true,true],
                        vec![true,true,false,false,false,false,true,true]];
        }
        'N' => {
            return vec![vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,true,false,true,true,false],
                        vec![false,true,true,true,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,true,true,true,false],
                        vec![false,true,true,false,true,true,true,false],
                        vec![false,true,true,false,false,true,true,false]];
        }
        'O' => {
            return vec![vec![false,false,true,true,true,true,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,false,true,true,true,true,false,false]];
        }
        'P' => {
            return vec![vec![false,true,true,true,true,true,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,false,false],
                        vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,false,false,false,false,false]];
        }
        'Q' => {
            return vec![vec![false,false,true,true,true,true,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,true,false,true,false],
                        vec![false,true,true,false,true,true,false,false],
                        vec![false,false,true,true,false,true,true,true]];
        }
        'R' => {
            return vec![vec![false,true,true,true,true,true,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,false,false],
                        vec![false,true,true,false,true,true,false,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false]];
        }
        'S' => {
            return vec![vec![false,false,true,true,true,true,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,true,true,true,false,false],
                        vec![false,false,true,true,true,true,true,false],
                        vec![false,false,false,false,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,false,true,true,true,true,false,false]];
        }
        'T' => {
            return vec![vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false]];
        }
        'U' => {
            return vec![vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,false,true,true,true,true,false,false]];
        }
        'V' => {
            return vec![vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,false,true,true,true,true,false,false],
                        vec![false,false,false,true,true,false,false,false]];
        }
        'W' => {
            return vec![vec![true,true,false,false,false,false,true,true],
                        vec![true,true,false,false,false,false,true,true],
                        vec![true,true,false,false,false,false,true,true],
                        vec![true,true,false,true,true,false,true,true],
                        vec![true,true,true,true,true,true,true,true],
                        vec![true,true,true,true,true,true,true,true],
                        vec![true,true,true,false,false,true,true,true],
                        vec![true,true,false,false,false,false,true,true]];
        }
        'X' => {
            return vec![vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,false,true,true,true,true,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,true,true,true,true,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,true,true,false]];
        }
        'Y' => {
            return vec![vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,false,true,true,true,true,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false]];
        }
        'Z' => {
            return vec![vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,false,false,false,true,true,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,true,true,false,false,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false]];
        }
        '1' => {
            return vec![vec![false,false,false,true,true,false,false,false],
                        vec![false,false,true,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,true,true,true,true,false,false]];
        }
        '2' => {
            return vec![vec![false,false,true,true,true,true,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,true,true,true,false],
                        vec![false,false,false,false,true,true,true,false],
                        vec![false,false,false,true,true,true,false,false],
                        vec![false,false,true,true,true,false,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false]];
        }
        '3' => {
            return vec![vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,false,false,false,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,false,false,false,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false]];
        }
        '4' => {
            return vec![vec![false,true,true,false,true,true,false,false],
                        vec![false,true,true,false,true,true,false,false],
                        vec![false,true,true,false,true,true,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,false,false,false,true,true,false,false],
                        vec![false,false,false,false,true,true,false,false],
                        vec![false,false,false,false,true,true,false,false]];
        }
        '5' => {
            return vec![vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,true,true,true,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,false,false,false,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,false,false]];
        }
        '6' => {
            return vec![vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,false,true,false],
                        vec![false,true,true,false,false,false,false,false],
                        vec![false,true,true,true,true,true,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,false,true,true,true,true,false,false]];
        }
        '7' => {
            return vec![vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,false,false,false,true,true,false],
                        vec![false,false,false,false,true,true,true,false],
                        vec![false,false,false,false,true,true,false,false],
                        vec![false,false,false,true,true,true,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false]];
        }
        '8' => {
            return vec![vec![false,false,true,true,true,true,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,false,true,true,true,true,false,false],
                        vec![false,false,true,true,true,true,false,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,false,true,true,true,true,false,false]];
        }
        '9' => {
            return vec![vec![false,false,true,true,true,true,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,false,true,true,true,true,true,false],
                        vec![false,false,false,false,false,true,true,false],
                        vec![false,false,false,false,false,true,true,false],
                        vec![false,false,false,false,false,true,true,false]];
        }
        '0' => {
            return vec![vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,false,true,true,true,false],
                        vec![false,true,true,true,false,true,true,false],
                        vec![false,true,true,false,false,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false]];
        }
        ':' => {
            return vec![vec![false,false,true,true,false,false,false,false],
                        vec![false,false,true,true,false,false,false,false],
                        vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,false,false,false,false,false],
                        vec![false,false,true,true,false,false,false,false],
                        vec![false,false,true,true,false,false,false,false]];
        }
        '.' => {
            return vec![vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false]];
        }
        '>' => {
            return vec![vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,false,true,false,false,false],
                        vec![false,false,false,false,true,true,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,false,false,false,true,true,false,false],
                        vec![false,false,false,false,true,false,false,false],
                        vec![false,false,false,false,false,false,false,false]];
        }
        '<' => {
            return vec![vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,true,false,false,false,false],
                        vec![false,false,true,true,false,false,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,false,true,true,false,false,false,false],
                        vec![false,false,false,true,false,false,false,false],
                        vec![false,false,false,false,false,false,false,false]];
        }                                                                       //********************//
        '~' => {                                                  // <======    //ACTS AS DOWN ARROW!!//
            return vec![vec![false,false,false,false,false,false,false,false],  //********************//
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,false,true,true,true,true,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,false,false,false,false,false]];
        }
        '^' => {
            return vec![vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,true,true,true,true,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,false,false,false,false,false]];
        }  
        '-' => {
            return vec![vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,false,false,false,false,false],
                        vec![false,false,true,true,true,true,false,false],
                        vec![false,false,true,true,true,true,false,false],
                        vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,false,false,false,false,false]];
        }
        '#' => {
            return vec![vec![false,false,true,true,false,true,true,false],
                        vec![false,true,true,true,true,true,true,true],
                        vec![false,false,true,true,false,true,true,false],
                        vec![false,true,true,true,false,true,true,false],
                        vec![false,true,true,false,true,true,true,false],
                        vec![false,true,true,false,true,true,false,false],
                        vec![true,true,true,true,true,true,true,false],
                        vec![false,true,true,false,true,true,false,false]];
        }
        '/' => {
            return vec![vec![false,false,false,false,true,true,false,false],
                        vec![false,false,false,false,true,true,false,false],
                        vec![false,false,false,true,true,true,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,true,true,true,false,false,false],
                        vec![false,false,true,true,false,false,false,false],
                        vec![false,false,true,true,false,false,false,false]];
        }
        '+' => {
            return vec![vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,true,true,true,true,true,true,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,true,true,false,false,false],
                        vec![false,false,false,false,false,false,false,false]];
        }
        ' ' => {
            return vec![vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,false,false,false,false,false],
                        vec![false,false,false,false,false,false,false,false]];
        }
        _ => {
            return vec![vec![true,true,true,true,true,true,true,true],
                        vec![true,true,true,true,true,true,true,true],
                        vec![true,true,true,true,true,true,true,true],
                        vec![true,true,true,true,true,true,true,true],
                        vec![true,true,true,true,true,true,true,true],
                        vec![true,true,true,true,true,true,true,true],
                        vec![true,true,true,true,true,true,true,true],
                        vec![true,true,true,true,true,true,true,true]];
        }
    }
}