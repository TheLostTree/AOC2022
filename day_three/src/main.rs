use std::{path::Path, fs::File, io::Read, collections::btree_set::Intersection, str::Chars};

fn main() {
    // part_one();
    part_two()
}

#[allow(dead_code)]
fn part_one(){
    let path = Path::new("./input.txt");
    let mut file = match File::open(path){
        Ok(f)=>f,
        Err(_)=>panic!("lol"),
    };
    let mut contents = String::new();
    _ = file.read_to_string(&mut contents);

    let a = contents.split("\n");
    let rucksacks =  a.map(|x| [&x[..x.len()/2], &x[(x.len()/2)..]]);

    let mut sum =0;
    for rucksack in rucksacks{

        if rucksack[0].len() < 1 {continue;}
        //get common letter between two strs
        if let [first, second] = &rucksack[..]{
            let common: Vec<char> = first.chars().filter(|c| second.contains(*c)).collect();
            
            let priority = get_priority(match common.first(){
                Some(f)=>*f,
                None=>'.'
            });
            sum += priority;
        }

    }

    println!("{} ", sum);
    
    
}


fn part_two(){
    let path = Path::new("./input.txt");
    let mut file = match File::open(path){
        Ok(f)=>f,
        Err(_)=>panic!("lol"),
    };
    let mut contents = String::new();
    _ = file.read_to_string(&mut contents);

    let rucksacks: Vec<&str> =  contents.split("\n").collect();

    let mut sum = 0;
    for i in (0..rucksacks.len()-3).step_by(3){
        let slice = &rucksacks[i..i+3];

        //common between 1 and 2
        let common:Vec<char> = slice[0].chars().filter(|x| slice[1].contains(*x)).collect();
        let common_all = slice[2].chars().filter(|x| common.contains(x));


        let com_char = common_all.collect::<Vec<char>>().first().unwrap().to_owned();
        let prio = get_priority(com_char);

        sum += prio;

        // println!("{:?}", prio);
    }

    println!("{} ", sum);
}

fn get_priority(c : char)->i32{

    let f = c as i32;
    if 'a' as i32 <= f && f <= 'z' as i32{
        return (c as i32 - 'a' as i32) + 1;
    }else if 'A' as i32 <= f && f <= 'Z' as i32{
        return (c  as i32 - 'A' as i32) + 27;
    }

    0
}