use std::{fs::File, io::Read};


const FILE_NAME: &str = "input.txt";

fn main() {
    // println!("Hello, world!");
    part_one();
    part_two();
}

fn get_visible_sides(trees: Vec<Vec<i32>>, x: usize, y:usize)->i32{
    let tree = trees.get(x).expect(":(").get(y).expect(">:(");
    let mut visible_sides = 4;


    if x !=0{
        for xcursor in (0..x).rev(){
            let test_tree = trees.get(xcursor).expect(":(").get(y).expect(":(");
            if tree <= test_tree{
                visible_sides = visible_sides - 1;
                break;
            }
        }
    }
    
    for xcursor in x+1..trees.len() {
        let test_tree = trees.get(xcursor).expect(":(").get(y).expect(":(");
        if tree <= test_tree{
            visible_sides = visible_sides - 1;
            break;
        }
    }
    if y != 0{
        for ycursor in (0..y).rev(){
            let test_tree = trees.get(x).expect(":(").get(ycursor).expect(":(");
            if tree <= test_tree{
                visible_sides = visible_sides - 1;
                break;
            }
        }
    }
    for ycursor in y+1..trees.get(x).expect(":(").len() {
        let test_tree = trees.get(x).expect(":(").get(ycursor).expect(":(");
        if tree <= test_tree{
            visible_sides = visible_sides - 1;
            break;
        }
    }
    // return if visible_sides == 0 {0} else {1}
    visible_sides
}
fn part_one(){
    let mut file = File::open(FILE_NAME).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    
    let lines = contents.split("\n");
    
    let mut vec : Vec<Vec<i32>>= Vec::new();
    for line in lines{
        let mut row : Vec<i32> = Vec::new();
        for c in line.chars(){
            let num :i32 = c.to_digit(10).unwrap() as i32; 
            row.push(num);
        }
        vec.push(row);
    }
    // println!("{:?}",vec);
    let mut visible_trees = 0;
    for tree in vec.iter().enumerate(){
        for tree2 in tree.1.iter().enumerate(){
            let is_visible = get_visible_sides(vec.clone(), tree.0, tree2.0);
            // println!("x {} y {} isvis {} height {}", tree.0, tree2.0, is_visible, tree2.1);
            visible_trees = visible_trees + if is_visible == 0 {0} else {1};
        }
    }

    println!("Visible trees: {}", visible_trees)

    
}

fn get_visible_range(trees: Vec<Vec<i32>>, x: usize, y:usize)->i32{
    let tree = trees.get(x).expect(":(").get(y).expect(">:(");
    let mut visibility : [i32;4]= [0,0,0,0];


    let mut count = 0;

    for xcursor in (0..x).rev(){
        count+=1;
        let test_tree = trees.get(xcursor).expect(":(").get(y).expect(":(");
        if tree <= test_tree{
            break;
        }
    }
    visibility[0] = count;
    count = 0;

    
    for xcursor in x+1..trees.len() {
        count+=1;

        let test_tree = trees.get(xcursor).expect(":(").get(y).expect(":(");
        if tree <= test_tree{
            break;
        }
    }

    visibility[1] = count;
    count = 0;

    for ycursor in (0..y).rev(){
        count+=1;

        let test_tree = trees.get(x).expect(":(").get(ycursor).expect(":(");
        if tree <= test_tree{
            break;
        }
    }

    visibility[2] = count;
    count = 0;
    for ycursor in y+1..trees.get(x).expect(":(").len() {
        count+=1;

        let test_tree = trees.get(x).expect(":(").get(ycursor).expect(":(");
        if tree <= test_tree{
            break;
        }
    }
    visibility[3] = count;
    // return if visible_sides == 0 {0} else {1}

    let ret = visibility.iter().fold(1, |a,b|a*b);
    // println!("score: {}, vec {:?}",ret, visibility);
    ret
}


fn part_two(){
    let mut file = File::open(FILE_NAME).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    
    let lines = contents.split("\n");
    
    let mut vec : Vec<Vec<i32>>= Vec::new();
    for line in lines{
        let mut row : Vec<i32> = Vec::new();
        for c in line.chars(){
            let num :i32 = c.to_digit(10).unwrap() as i32; 
            row.push(num);
        }
        vec.push(row);
    }
    // println!("{:?}",vec);
    let mut max_score = 0;
    for tree in vec.iter().enumerate(){
        for tree2 in tree.1.iter().enumerate(){
            let scenic_score = get_visible_range(vec.clone(), tree.0, tree2.0);
            // println!("x {} y {} isvis {} height {}", tree.0, tree2.0, is_visible, tree2.1);
            max_score = max_score.max(scenic_score);
        }
    }

    println!("max score: {}", max_score)

    
}