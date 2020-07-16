use std::error::Error;
use std::{fs, env};
use std::ops::Index;
use std::slice::SliceIndex;

// todo global Result alias
type Result<T> = std::result::Result<T, Box<dyn Error>>;

struct Node {
    is_path: bool,
    digit: u8,
    // x: u32,
    // y: u32,
}

type Row = Vec<Node>;

struct Board {
    rows: Vec<Row>,
    size: Size,
}

impl<I: SliceIndex<[Row]>> Index<I> for Board {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.rows[index]
    }
}

struct Size {
    height: u32,
    width: u32,
}

impl Board {
    pub fn from_file(path: &str) -> Result<Board> {
        // let p = format!("{}/{}", env::current_dir()?.display(), path);
        // println!("{}", p);
        let contents = fs::read_to_string(format!("{}/{}", env::current_dir()?.display(), path))
            .expect("Cannot read the file");
        let mut lines = contents.lines();
        let first_line = lines.next().ok_or("An error occurred while parsing the input file")?;
        let size_chunks: Vec<&str> = first_line.split(",").collect();
        let size = Size {
            height: size_chunks[0].parse()?,
            width: size_chunks[1].parse()?,
        };
        let rows = lines
            .map(|line| {
                line.split("")
                    .filter(|char| !char.trim().is_empty())
                    .map(|char| {
                        // println!("too: {}", char);
                        let digit = char.parse().unwrap_or(0);
                        Node {
                            is_path: digit == 1,
                            digit,
                        }
                    })
                    .collect()
            })
            .collect();

        Ok(Board {
            rows,
            size,
        })
    }

    pub fn print(&self) {
        self.rows.iter()
            .for_each(|row| {
                row.iter().for_each(|node| {
                        print!("{}", node.digit);
                    });
                print!("\n");
            })
    }
}

pub fn solve(input_path: &str) -> Result<u32> {
    let board = Board::from_file(input_path)?;
    board.print();
    Ok(0)
}