
use std::{collections::{HashMap, VecDeque}, fs::File, io::Read, path::Path};

fn main() {
    // println!("Hello, world!");
    part_two()
}
#[derive(Debug)]
struct Move{
    quantity: i32,
    from_id: i32,
    to_id: i32,
}

impl Move {
    fn parse_from_str(input: &String)->Move{
        //should be "move a from b to c"
        let parts:Vec<&str> = input.split(" ").collect();
        Move {
            quantity: parts[1].parse().unwrap(),
            from_id: parts[3].parse().unwrap(),
            to_id: parts[5].parse().unwrap(),
        }

    }
}


#[allow(dead_code)]
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
    //the first 9 lines are for the inital state
    let lines: Vec<String>  = contents.split("\n").map(|x| x.to_string()).collect();

    let first_ten: Vec<String> = lines[0..9].to_vec();

    let mut stacks: HashMap<i32, VecDeque<String>> = HashMap::new();

    for f in (0..8).rev() {

        //input is missing a " " at the end to allow me to do a brainless pattern
        let line = &first_ten[f];
        //3 char skip 1
        for i in (0..line.len() - 1).step_by(4) {
            let slice = &line[i..i+3];
            let stack_id = (i / 4 + 1) as i32;
            // println!("'{}' '{}'", slice, stack_id);
            if slice == "   "{
                continue;
            }

            //default adding a new vec, but if it exists already jsut push it to the end!
            stacks
                .entry(stack_id)
                .and_modify(|vec| vec.push_back(slice.to_string()))
                .or_insert(VecDeque::from([slice.to_string()]));
        }
    }
    // for stack in stacks{
    //     println!("{} {:?}", stack.0, stack.1);
    // }

    for f in 10..lines.len(){
        let line = &lines[f];
        if line.len() < 4{
            continue;
        }
        let mv = Move::parse_from_str(line);

        //process move

        let mut taken_nums:VecDeque<String> = VecDeque::new();
        {
            //only one mutable borrow, so this is in a block
            let from_vec = (&mut stacks).get_mut(&mv.from_id).unwrap();
            for _ in 0..mv.quantity{
                taken_nums.push_back(from_vec.pop_back().unwrap());
            }
        }
        // println!("{:?}: {:?}", mv, &taken_nums);
        let to_vec = (&mut stacks).get_mut(&mv.to_id).unwrap();

        to_vec.append(&mut taken_nums);

        /*
        to_vec.push_back(from_vec.pop_back().unwrap());
         */

    }

    let mut result = String::new();
    for i in 1..10{
        result += stacks.get(&i).unwrap().back().unwrap();
    }
    println!("{}", result.replace("]", "").replace("[", ""));


    
}


fn part_two() {
    let path = Path::new("./input.txt");
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            panic!("lol panic")
        }
    };

    let lines_of_info = 9;

    let mut contents = String::new();
    _ = file.read_to_string(&mut contents);
    //the first 9 lines are for the inital state
    let lines: Vec<String>  = contents.split("\n").map(|x| x.to_string()).collect();

    let first_ten: Vec<String> = lines[0..lines_of_info].to_vec();

    let mut stacks: HashMap<i32, VecDeque<String>> = HashMap::new();

    for f in (0..(lines_of_info-1)).rev() {

        //input is missing a " " at the end to allow me to do a brainless pattern
        let line = &first_ten[f];
        //3 char skip 1
        for i in (0..line.len() - 1).step_by(4) {
            let slice = &line[i..i+3];
            let stack_id = (i / 4 + 1) as i32;
            // println!("'{}' '{}'", slice, stack_id);
            if slice == "   "{
                continue;
            }

            //default adding a new vec, but if it exists already jsut push it to the end!
            stacks
                .entry(stack_id)
                .and_modify(|vec| vec.push_back(slice.to_string()))
                .or_insert(VecDeque::from([slice.to_string()]));
        }
    }
    

    for f in (lines_of_info+1)..lines.len(){
        let line = &lines[f];
        if line.len() < 4{
            continue;
        }
        let mv = Move::parse_from_str(line);

        //process move

        let mut taken_nums:VecDeque<String> = VecDeque::new();
        {
            //only one mutable borrow, so this is in a block
            let from_vec = (&mut stacks).get_mut(&mv.from_id).unwrap();
            for _ in 0..mv.quantity{
                taken_nums.push_front(from_vec.pop_back().unwrap());
            }
        }
        // println!("{:?}: {:?}", mv, &taken_nums);
        let to_vec = (&mut stacks).get_mut(&mv.to_id).unwrap();

        to_vec.append(&mut taken_nums);

        // for stack in &stacks{
        //     println!("{} {:?}", stack.0, stack.1);
        // }
        /*
        to_vec.push_back(from_vec.pop_back().unwrap());
         */

    }

    let mut result = String::new();
    for i in 1..10{
        result += stacks.get(&i).unwrap().back().unwrap();
    }
    
    println!("{}", result.replace("]", "").replace("[", ""));


    
}
