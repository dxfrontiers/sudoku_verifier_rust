use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};

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

fn eval_sudoku_string(line: String) -> bool{

}
fn parse_raw_line_to_sudoku(input: Result<String,_>) -> Option<Sudoku>{
    match input{
        Ok(input) => {
            input
            Sudoku
        }
        _ => None
    }
}

fn main() -> std::io::Result<()>  {

    let input_file = env::args()
        .skip(1)
        .next()
        .unwrap();
        //.ok_or(|| ErrorKind::InvalidInput)?;


    let file = File::open(input_file)?;
    let reader = BufReader::new(file);


    let valids = reader.lines()
        .enumerate()
        .filter_map(|(i,line)|parse_raw_line_to_sudoku(line))
        .map(|(i,line)| (i,eval_sudoku_string(line)) )
        //.map(|(i,validity)| {println!("Line {} \t valid: {}",i,validity)});
        .fold(0,|acc,t| acc+t.1 as i32);

    println!("Valids: {}",valids);

    let sudoku = Sudoku{
        fields:[
            8, 3, 5,  4, 1, 6,  9, 2, 7,
            2, 9, 6,  8, 5, 7,  4, 3, 1,
            4, 1, 7,  2, 9, 3,  6, 5, 8,

            5, 6, 9,  1, 3, 4,  7, 8, 2,
            1, 2, 3,  6, 7, 8,  5, 4, 9,
            7, 4, 8,  5, 2, 9,  1, 6, 3,

            6, 5, 2,  7, 8, 1,  3, 9, 4,
            9, 8, 1,  3, 4, 5,  2, 7, 6,
            3, 7, 4,  9, 6, 2,  8, 1, 5
        ]
    };

    let correct_range = sudoku.fields.iter().all(|e| *e<10 && *e>0);
    if !correct_range {
        eprintln!("Invalid range");
        //return Err();
    }

    let valid_columns = is_valid_view(sudoku.as_columns());
    let valid_rows = is_valid_view(sudoku.as_rows());
    let valid_boxes = is_valid_view(sudoku.as_boxes());

    println!("Valid placement: {}", valid_rows && valid_columns && valid_rows);

    println!("Hello, world!");
    Ok(())
}
