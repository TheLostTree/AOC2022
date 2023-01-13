use std::{fs::File, io::Read, path::Path};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let path = Path::new("./input.txt");
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            panic!("lol panic")
        }
    };
    let mut contents = String::new();
    _ = file.read_to_string(&mut contents);

    let mut count = 0;
    for line in contents.split("\n") {
        //two ranges in te format a-b,c-d
        let mut splt = line.split(",");
        // println!("{:?} {:?}", splt.next(), splt.next());
        let mut s1 = splt.next().unwrap().split("-");
        let mut s2 = splt.next().unwrap().split("-");

        let one_start: i32 = s1.next().unwrap().parse().unwrap();
        let one_end: i32 = s1.next().unwrap().parse().unwrap();
        let two_start: i32 = s2.next().unwrap().parse().unwrap();
        let two_end: i32 = s2.next().unwrap().parse().unwrap();

        if one_start <= two_start && one_end >= two_end {
            count += 1;
            continue;
        }

        if two_start <= one_start && two_end >= one_end {
            count += 1;
        }
    }
    println!("{}", count)
}

fn part_two() {
    let path = Path::new("./input.txt");
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            panic!("lol panic")
        }
    };
    let mut contents = String::new();
    _ = file.read_to_string(&mut contents);

    let mut count = 0;
    for line in contents.split("\n") {
        //two ranges in te format a-b,c-d
        let mut splt = line.split(",");
        // println!("{:?} {:?}", splt.next(), splt.next());
        let mut s1 = splt.next().unwrap().split("-");
        let mut s2 = splt.next().unwrap().split("-");

        let one_start: i32 = s1.next().unwrap().parse().unwrap();
        let one_end: i32 = s1.next().unwrap().parse().unwrap();
        let two_start: i32 = s2.next().unwrap().parse().unwrap();
        let two_end: i32 = s2.next().unwrap().parse().unwrap();

        if one_start >= two_start && one_start <= two_end {
            count += 1;
            continue;
        }

        if two_start >= one_start && two_start <= one_end {
            count += 1;
        }
    }
    println!("{}", count)
}
