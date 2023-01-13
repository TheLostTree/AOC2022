use std::{path::Path, fs::File, io::Read};

fn main() {
    // println!("Hello, world!");
    // part_one();
    part_two()
}

fn part_two(){
    let path = Path::new("./input.txt");
    let mut file = match File::open(path) {
        Ok(l)=>l,
        Err(_) => panic!("fucky!"),
    };
    let mut str = String::new();
    _ = file.read_to_string(&mut str);

    let mut int = -1;
    //iterate over the str in 4 ranges
    for i in 0..str.len()-14{
        let slice = &str[i..i+14];
        // if slice does not have characters that are repeated
        // set int to i + 4


        let mut is_slay = true;

        let chars = slice.chars();
        for c in chars{
            let mut count = 0;
            for b in slice.chars(){
                if b == c {
                    count +=1;
                }
            }

            if count > 1{
                is_slay = false
            }
        }

        if is_slay{
            int = (i as i32) + 14;
            break;
        }
    }
    

    println!("first start of messsage marker at {}", int)


}

#[allow(dead_code)]
fn part_one(){
    let path = Path::new("./input.txt");
    let mut file = match File::open(path) {
        Ok(l)=>l,
        Err(_) => panic!("fucky!"),
    };
    let mut str = String::new();
    _ = file.read_to_string(&mut str);

    let mut int = -1;
    //iterate over the str in 4 ranges
    for i in 0..str.len()-4{
        let slice = &str[i..i+4];
        // if slice does not have characters that are repeated
        // set int to i + 4


        let mut is_slay = true;

        let chars = slice.chars();
        for c in chars{
            let mut count = 0;
            for b in slice.chars(){
                if b == c {
                    count +=1;
                }
            }

            if count > 1{
                is_slay = false
            }
        }

        if is_slay{
            int = (i as i32) + 4;
            break;
        }
    }
    

    println!("first marker at {}", int)


}