use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind, Read};
use std::convert::TryInto;

use rayon::prelude::*;

/**
    A struct holding a 9x9 filed sudoku board
 */
struct Sudoku{
    fields: [u8;81]
}

/**
    Provide implementations for different views into the data structure
    Since the values (u8) are smaller than pointers to them,
    the functions allocate a new slice and copy over the values.
*/
impl Sudoku{

    /**
        Returns the field values as an array of arrays, each one representing the rows
    */
    fn as_rows(&self)-> [[u8;9];9] {
        let mut res = [[0u8;9];9];
        self.fields.iter().enumerate().for_each(
            |(e,v)| res[e/9][e%9]=*v
        );
        res
    }

    /**
        Returns the field values as an array of arrays, each one representing the columns
    */
    fn as_columns(&self)-> [[u8;9];9]{
        let mut res = [[0u8;9];9];
        self.fields.iter().enumerate().for_each(
            |(e,v)| res[e%9][e/9]=*v
        );
        res
    }
    /**
       Returns the field values as an array of arrays, each one representing one of the 3 3x3 boxes
       x = (e/3)%3 box for each row
         + (e/27)*3 (integer division cut-off!) offset for each 3 rows
       y = (e%3) column in box
         + (e/9)%3*3 row in box
    */
    fn as_boxes(&self)-> [[u8;9];9]{
        let mut res = [[0u8;9];9];
        self.fields.iter().enumerate().for_each(
            |(e,v)|
                res [(e/3)%3 + (e/27)*3] [(e%3) + (e/9)%3*3] = *v
        );
        res
    }
}

/**
    Determines if all the slice are unique
*/
fn is_valid_view(sl: [[u8;9];9]) -> bool {
    sl.iter().all(|e| is_slice_unique(e) )
}

/**
    Determines if all 9 numbers in the slice are unique
*/
fn is_slice_unique(sl: &[u8;9]) -> bool{
    // 9 unique numbers shifting '1' their value to the left will yield 9 ones
     sl.iter().fold(0u16,|a,e|
         a | 1u16<<(*e-1) as u16
     ) == (1u16<<9) -1
}

/**
    Evaluate a given sudoku
    For a sudoku, filled with valid values, to be valid, all of the following needs to contain no duplicates:
    - rows
    - columns
    - boxes
*/
fn eval_sudoku(sudoku: Sudoku) -> bool{
    is_valid_view(sudoku.as_columns())
        && is_valid_view(sudoku.as_rows())
        && is_valid_view(sudoku.as_boxes())
}

/**
    Create a sudoku from a string representation, if it describes a sudoku filled with valid values (1-9)
 */
fn parse_raw_line_to_sudoku(input: &str) -> Option<Sudoku>{
    let mut fields =  [0;81];
    let count = input.split(',')
        .filter_map(|split| split.trim().parse::<u8>().ok())
        .take(81)
        .filter(|e| *e<10 && *e>0)
        .enumerate()
        .map(|(idx,num)| fields[idx] = num)
        .count();
    match count {
        81 => {
            Some(Sudoku{ fields })
        },
        _ => None
    }
}

/**
    Run all the things
    We read the file line by
*/
fn main() -> std::io::Result<()>  {

    let input_file = env::args()
        .nth(1)
        .ok_or(Error::from(ErrorKind::InvalidInput))?;

    let file = File::open(input_file)?;


    let mut valid_count = 0;
    for i in 0 .. 100 {
        let mut reader = BufReader::new(&file);


        // The sequential version

        valid_count += reader.lines()
            .filter_map(|line|line.ok())
            .filter_map(|line| parse_raw_line_to_sudoku(&line) )
            .map(|sudoku| eval_sudoku(sudoku) )
            .filter(|b|*b)
            .count();

        // The parallel version
        /*
        let mut lines = String::new();
        reader.read_to_string(&mut lines).unwrap();
        valid_count += lines.par_lines()
            .filter_map(|line| parse_raw_line_to_sudoku(line) )
            .map(|sudoku| eval_sudoku(sudoku) )
            .filter(|b|*b)
            .count();

         */

    }


    println!("Valid sudokus: {}",valid_count);
    Ok(())
}
