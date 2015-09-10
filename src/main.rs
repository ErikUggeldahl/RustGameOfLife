use std::fmt;

// Idea: Implement again using bitwise math and check the speed difference
const SIZE: usize = 5;

type Coord = (usize, usize);

fn main() {

	let mut new_grid = Grid::new(&[
		[false,false,false,false,false],
		[false,false,true, false,false],
		[false,false,true, false,false],
		[false,false,true, false,false],
		[false,false,true, false,false]]);

	println!("{}", new_grid);
	println!("{}", new_grid.next().unwrap());
	println!("{}", new_grid.next().unwrap().next().unwrap());
}

#[derive(Clone, Debug)]
struct Grid {
	grid: [[bool; SIZE]; SIZE]
}

impl Grid {
	fn new(initial_state: &[[bool; SIZE]; SIZE]) -> Grid {
		Grid {
			grid: initial_state.clone()
		}
	}

	fn occupied(self: &Grid, coord: Coord) -> bool {
		let (x, y) = coord;

		if self.grid[y][x] { true } else { false }
	}

	fn count_neighbours(self: &Grid, coord: Coord) -> u8 {
		let (x, y) = coord;

		let (x_min, y_min, x_max, y_max) = (x == 0, y == 0, x >= SIZE - 1, y >= SIZE - 1);

		let mut valid_coords: Vec<Coord> = Vec::new();

		if !x_min && !y_min { valid_coords.push((x-1, y-1)); }
		if !x_min           { valid_coords.push((x-1, y  )); }
		if !x_min && !y_max { valid_coords.push((x-1, y+1)); }
		if !y_min           { valid_coords.push((x,   y-1)); }
		if !y_max           { valid_coords.push((x,   y+1)); }
		if !x_max && !y_min { valid_coords.push((x+1, y-1)); }
		if !x_max           { valid_coords.push((x+1, y  )); }
		if !x_max && !y_max { valid_coords.push((x+1, y+1)); }

		valid_coords.into_iter().fold(0, |acc, item| acc + if self.occupied(item) { 1 } else { 0 })
	}

	fn alive(self: &Grid, coord: Coord) -> bool {
		let neighbours = self.count_neighbours(coord);

		match self.occupied(coord) {
			true => match neighbours {
				2 | 3 => true,
				_ => false
			},
			false => if neighbours == 3 { true } else { false }
		}
	}
}

impl Iterator for Grid {
	type Item = Grid;

	fn next(&mut self) -> Option<Grid> {
		let mut new_grid = self.clone();

		for (j, row) in new_grid.grid.iter_mut().enumerate() {
			for (i, item) in row.iter_mut().enumerate() {
				*item = self.alive((i, j));
			}
		}
		
		Some(new_grid)
	}
}

impl fmt::Display for Grid {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

		let mut print = String::new();

		for row in self.grid.iter() {
			for item in row.iter() {
				print.push_str(if *item { "0 " } else { ". " });
			}
			print.push('\n');
		}

		write!(f, "{}", print)
    }
}
