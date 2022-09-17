use std::{thread, time};

#[derive(Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Point {
    fn from(item: (usize, usize)) -> Self {
        return Self {
            x: item.0,
            y: item.1,
        };
    }
}

#[derive(Debug, Clone)]
struct Cell {
    alive: bool,
}

impl Cell {
    fn new(alive: bool) -> Self {
        Cell { alive: alive }
    }
    fn is_alive(&self) -> bool {
        return self.alive;
    }
    fn set_state(&mut self, state: bool) {
        self.alive = state;
    }
}

struct Map {
    cells: Vec<Cell>,
    height: usize,
    width: usize,
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        Map {
            width: width,
            height: height,
            cells: vec![Cell::new(false); width * height],
        }
    }
    fn set_state(&mut self, cells_coords: &[Point]) {
        self.cells = vec![Cell::new(false); self.width * self.height];

        for &pos in cells_coords.iter() {
            let idx = self.coords_to_index(&pos);
            self.cells[idx].set_state(true);
        }
    }
    fn coords_to_index(&self, pos: &Point) -> usize {
        pos.y * self.width + pos.x
    }
    fn get_neighbours(&self, pos: &Point) -> usize {
        let mut count: usize = 0;

        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_col == 0 && delta_row == 0 {
                    continue;
                }
                let neighbour_row: usize = (pos.x + delta_row) % self.height;
                let neighbour_col: usize = (pos.y + delta_col) % self.width;
                let idx: usize = self.coords_to_index(&Point::from((neighbour_row, neighbour_col)));

                if self.cells[idx].is_alive() {
                    count += 1;
                }
            }
        }
        count
    }
    fn update_map(&mut self) {
        let mut next = self.cells.clone();
        for i in 0..self.height {
            for j in 0..self.width {
                let posi: Point = Point::from((i, j));
                let idx: usize = self.coords_to_index(&posi);
                let neigh: usize = self.get_neighbours(&posi);
                if self.cells[idx].is_alive() == true && (neigh < 2 || neigh > 3) {
                    next[idx].set_state(false);
                } else if self.cells[idx].is_alive() == false && neigh == 3 {
                    next[idx].set_state(true);
                }
            }
        }
        self.cells = next;
    }
    fn render_map(&self) {
        print!("{}[2J", 27 as char);
        for i in 0..self.height {
            // if i == 0 {
            //     for _j in 0..self.width * 4 {
            //         print!("-");
            //     }
            //     println!();
            // }
            for j in 0..self.width {
                // if j == 0 {
                //     print!("|");
                // }
                let posi = Point::from((i, j));
                if self.cells[self.coords_to_index(&posi)].alive == true {
                    print!(" * ");
                } else {
                    print!("   ");
                }
                // print!("|");
            }
            // println!();
            // for _j in 0..self.width * 4 {
            //     print!("-");
            // }
            println!();
        }
        println!();
    }
}

fn main() {
    let mut map: Map = Map::new(10, 10);
    let p: [Point; 5] = [
        Point::from((1, 3)),
        Point::from((2, 3)),
        Point::from((3, 3)),
        Point::from((3, 2)),
        Point::from((2, 1)),
    ];
    map.set_state(&p);
    map.get_neighbours(&p[0]);
    println!("{}", map.get_neighbours(&p[0]));

    loop {
        let time_sleep = time::Duration::from_millis(500);
        thread::sleep(time_sleep);
        map.render_map();
        map.update_map();
    }
}
