use std::error::Error;
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Row = Vec<Node>;

pub struct Node {
    digit: u8,
    visited: bool,
}

impl Node {
    fn from_char(char: &str) -> Node {
        Node {
            digit: char.parse().unwrap_or(0),
            visited: false,
        }
    }

    fn is_path(&self) -> bool {
        self.digit == 1
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct Labyrinth {
    rows: Vec<Row>,
    size: Vector2d,
}

#[derive(Eq, Debug)]
pub struct Vector2d {
    x: usize,
    y: usize,
}

impl Vector2d {
    fn get_direction(&self, to: &Vector2d) -> Direction {
        let x = to.x as i64 - self.x as i64;
        let y = to.y as i64 - self.y as i64;
        if (x != 0 && y != 0) || (x == 0 && y == 0) {
            panic!("at the disco");
        }
        if x > 0 {
            Direction::RIGHT
        } else if x < 0 {
            Direction::LEFT
        } else if y > 0 {
            Direction::UP
        } else {
            Direction::DOWN
        }
    }
}

impl PartialEq for Vector2d {
    fn eq(&self, other: &Vector2d) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl From<(usize, usize)> for Vector2d {
    fn from(f: (usize, usize)) -> Self {
        Vector2d {
            x: f.0,
            y: f.1,
        }
    }
}

pub static START_POINT: Vector2d = Vector2d {
    x: 0,
    y: 1,
};

pub struct State {
    turns: u32,
    current_result: Option<u32>,
    direction: Direction,
}

impl State {
    pub fn empty() -> Self {
        State {
            turns: 0,
            current_result: None,
            // zakładam, że pierwszy ruch jest zawsze w prawo,
            // czyli, zakładam, że oprócz wejścia i wyjścia z labiryntu
            // wszystkie znaki na granicach planszy to zera
            // todo kierunek musi być Option jeżeli to nie prawda
            direction: Direction::RIGHT,
        }
    }
}

impl Labyrinth {
    pub fn from_file(path: &str) -> Result<Labyrinth> {
        let contents = fs::read_to_string(path)
            .expect("Cannot read the file");
        let mut lines = contents.lines();
        let first_line = lines.next()
            .ok_or("An error occurred while parsing the input file")?;
        let size_chunks: Vec<&str> = first_line.split(",").collect();
        let size = Vector2d {
            x: size_chunks[0].parse()?,
            y: size_chunks[1].parse()?,
        };
        let rows = lines
            .map(|line| {
                line.split("")
                    .filter(|char| !char.trim().is_empty())
                    .map(|char| Node::from_char(char))
                    .collect()
            })
            .collect();

        Ok(Labyrinth {
            rows,
            size,
        })
    }

    pub fn find_way_out_from(&mut self, current_coords: &Vector2d, state: State) -> Option<u32> {
        if state.current_result.is_some() && state.turns >= state.current_result.unwrap() {
            return state.current_result;
        }
        if self.is_end_point(current_coords) {
            return Some(state.turns);
        }
        self.rows[current_coords.y][current_coords.x].visited = true;

        let mut current_result = state.current_result;
        let neighbours = self.get_unvisited_path_neighbour_coordinates(current_coords);
        for neighbour_coords in neighbours {
            let direction = current_coords.get_direction(&neighbour_coords);
            let new_turns = if direction != state.direction {
                state.turns + 1
            } else {
                state.turns
            };
            let new_state = State {
                direction,
                current_result,
                turns: new_turns,
            };
            let neighbour_result = self.find_way_out_from(&neighbour_coords, new_state);
            match (neighbour_result, current_result) {
                (Some(new_turns), None) => current_result = Some(new_turns),
                (Some(new_turns), Some(current_turns)) => {
                    if new_turns < current_turns {
                        current_result = Some(new_turns);
                    }
                }
                _ => ()
            };
        }

        self.rows[current_coords.y][current_coords.x].visited = false;
        current_result
    }

    pub fn _print(&self) {
        self.rows.iter()
            .for_each(|row| {
                row.iter()
                    .for_each(|node| {
                        print!("{}", node.digit);
                    });
                print!("\n");
            })
    }

    fn is_end_point(&self, coords: &Vector2d) -> bool {
        coords.x == self.size.x - 1 && coords.y == self.size.y - 2
    }

    fn get_unvisited_path_neighbour_coordinates(&self, coords: &Vector2d) -> Vec<Vector2d> {
        vec![
            (coords.x.checked_sub(1), Some(coords.y)),
            (Some(coords.x), coords.y.checked_sub(1)),
            (Some(coords.x + 1), Some(coords.y)),
            (Some(coords.x), Some(coords.y + 1)),
        ].iter()
            // todo filter jeśli ściany mogą mieć jedynki
            .filter(|c| c.0.is_some() && c.1.is_some())
            .map(|c| Vector2d::from((c.0.unwrap(), c.1.unwrap())))
            .filter(|c| !self.rows[c.y][c.x].visited && self.rows[c.y][c.x].is_path())
            .collect()
    }
}
