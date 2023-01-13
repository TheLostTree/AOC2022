use std::{fs::File, io::Read, collections::HashSet};

const INPUT: &str = "input.txt";

fn main() {
    part_two();
}

struct Move {
    direction: String,
    distance: i32,
}
#[derive(Clone, Copy)]
struct Vec2(i32, i32);

struct Grid {
    start: Vec2,
    head: Vec2,
    tail: Vec2,
    size: Vec2,
    stopped: bool,
}
impl Grid {
    fn print_grid(&self) {
        for y in (0..self.size.1).rev() {
            for x in 0..self.size.0 {
                let symbol: &str;
                if x == self.head.0 && y == self.head.1 {
                    symbol = "H";
                } else if x == self.tail.0 && y == self.tail.1 {
                    symbol = "T";
                } else if x == self.start.0 && y == self.start.1 {
                    symbol = "s";
                } else {
                    symbol = ".";
                }
                print!("{}", symbol);
            }
            println!("");
        }
        println!("head {},{} tail {} {}, start {} {}", self.head.0, self.head.1, self.tail.0, self.tail.1, self.start.0, self.start.1);

    }
    fn new(x: i32, y: i32) -> Self {
        Grid {
            start: Vec2(0, 0),
            head: Vec2(0, 0),
            tail: Vec2(0, 0),
            size: Vec2(x, y),
            stopped: true,
        }
    }

    fn move_head(&mut self, direction: &str) {
        //figure out where the tail is in comparison to the head
        match direction {
            "R" => {
                self.head.0 += 1;
            }
            "L" => {
                self.head.0 -= 1;
            }
            "U" => {
                self.head.1 += 1;
            }
            "D" => {
                self.head.1 -= 1;
            }
            _ => {}
        }

    
        let tail_delta = Vec2(self.head.0 - self.tail.0, self.head.1 - self.tail.1);
        /*
                 y0 y1 y2
              x0 F  F  T
              x1 F  F  T
              x2 T  T  T
        */
        if tail_delta.0.abs() == 2 || tail_delta.1.abs() == 2 {
            self.move_tail();
        }
        
        // find the place around the current tail pos where the head is closest
        
        // println!("new tail: {},{}", self.tail.0, self.tail.1);

    }

    fn move_tail(&mut self){
        let mut candidates: Vec<(Vec2, Vec2)> = Vec::new();
        for i in -1..=1{
            for j in -1..=1{
                let new_tail = Vec2(self.tail.0 + i, self.tail.1 + j);
                let new_tail_delta = Vec2(self.head.0 - new_tail.0, self.head.1 - new_tail.1);
                candidates.push((new_tail, new_tail_delta));
            }
        }
        let min = candidates.iter().min_by(|a, b| {
            let a_dist = a.1.0.abs() + a.1.1.abs();
            let b_dist = b.1.0.abs() + b.1.1.abs();
            a_dist.cmp(&b_dist)
        }).unwrap();
        // println!("tail delta: {},{}", tail_delta.0, tail_delta.1);
        // println!("min: {},{}", min.0.0, min.0.1);
        self.tail = min.0;
    }
    fn stop_moving(&mut self){
        self.stopped = true;
    }
}

fn part_one() {
    let mut file = File::open(INPUT).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    let moves = contents.split("\n").map(|x| {
        let mut chars = x.split(" ");
        let direction = chars.next().unwrap();
        let distance = chars.next().unwrap().parse::<i32>().unwrap();
        Move {
            direction: direction.to_string(),
            distance: distance,
        }
    });

    //init grid
    let mut grid = Grid::new( 6,6);
    // grid.tail = Vec2(1, 1);
    // grid.head = Vec2(1, 2);
    // grid.print_grid();
    let mut tail_locations : HashSet<(i32, i32)> = HashSet::new();
    for m in moves {
        // println!("== {} {} ==", m.direction, m.distance);
        for _ in 0..m.distance {
            grid.move_head(&m.direction);
            // grid.print_grid();
            tail_locations.insert((grid.tail.0, grid.tail.1));
        }
    }
    println!("tail locations count: {}", tail_locations.len());
    
}


struct Grid2 {
    start: (i32, i32),
    body: Vec<(i32, i32)>,
    size: (i32, i32),
    stopped: bool,
}
impl Grid2 {
    fn print_grid(&self) {
        for y in (0..self.size.1).rev() {
            for x in 0..self.size.0 {
                let mut symbol: &str = ".";
                let mut index = -1;
                for (i, b) in self.body.iter().enumerate() {
                    if b.0 == x && b.1 == y {
                        index = i as i32;
                        break;
                    }
                }
                if index >= 0 {
                    let resu = index;
                    if resu == 0 {
                        print!("H");
                    }else{
                        print!("{}", resu);
                    }
                    continue;

                }
                else if x == self.start.0 && y == self.start.1 {
                    symbol = "s";
                }
                print!("{}", symbol);
            }
            println!("");
        }
        println!("body {:?}, start {} {}", self.body, self.start.0, self.start.1);

    }
    fn new(x: i32, y: i32) -> Self {
        // let start = (10, 5);
        let start = (0, 0);
        Grid2 {
            start: start.clone(),
            body: vec![start.clone();10],
            size: (x, y),
            stopped: true,
        }
    }

    fn move_head(&mut self, direction: &str) {
        //figure out where the tail is in comparison to the head


        match direction {
            "R" => {
                self.body.get_mut(0).unwrap().0 += 1;
            }
            "L" => {
                self.body.get_mut(0).unwrap().0 -= 1;
            }
            "U" => {
                self.body.get_mut(0).unwrap().1 += 1;
            }
            "D" => {
                self.body.get_mut(0).unwrap().1 -= 1;
            }
            _ => {}
        }
        self.move_tail()
        
        // find the place around the current tail pos where the head is closest
        
        // println!("new tail: {},{}", self.tail.0, self.tail.1);

    }

    fn move_tail(&mut self) {
        for i in 1..self.body.len() {
            let next = *self.body.get(i).unwrap();
            let latest = *self.body.get(i-1).unwrap();

            let delta = (next.0 - latest.0, next.1 - latest.1);

            if delta.0.abs() < 2 && delta.1.abs() < 2{
                // println!("skipping, delta is {:?}", delta);
                continue;
            }

            let mut candidates: Vec<(Vec2, Vec2)> = Vec::new();

            for i in -1..=1{
                for j in -1..=1{
                    let new_tail = Vec2(next.0 + i, next.1 + j);
                    let new_tail_delta = Vec2(latest.0 - new_tail.0, latest.1 - new_tail.1);
                    candidates.push((new_tail, new_tail_delta));
                }
            }
            let min = candidates.iter().min_by(|a, b| {
                let a_dist = a.1.0.abs() + a.1.1.abs();
                let b_dist = b.1.0.abs() + b.1.1.abs();
                a_dist.cmp(&b_dist)
            }).unwrap();
            // println!("tail delta: {},{}", tail_delta.0, tail_delta.1);
            // println!("min: {},{}", min.0.0, min.0.1);
            self.body[i] = (min.0.0, min.0.1);
        }
    }

}


fn part_two(){
    let mut file = File::open(INPUT).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    let moves = contents.split("\n").map(|x| {
        let mut chars = x.split(" ");
        let direction = chars.next().unwrap();
        let distance = chars.next().unwrap().parse::<i32>().unwrap();
        Move {
            direction: direction.to_string(),
            distance: distance,
        }
    });
    let mut grid = Grid2::new( 100,100);
    let mut tail_locations : HashSet<(i32, i32)> = HashSet::new();

    for m in moves{
        // println!("== {} {} ==", m.direction, m.distance);
        for _ in 0..m.distance{
            grid.move_head(&m.direction);
            let tail = grid.body.iter().last().unwrap();
            tail_locations.insert(*tail);
        }
        // grid.print_grid();
    }

    println!("tail locations count: {}", tail_locations.len());


}
