use std::{path::Path, fs::File, io::Read, collections::HashMap};

fn main() {
    println!("Hello, world!");
    // println!("{:?}", std::env::current_dir());
    let path = Path::new("./input.txt");
    let display = path.display();

    let mut file = match File::open(&path){
        Ok(f)=>{f},
        Err(_)=>panic!("file not found!")
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("{} has a size of {}", display, s.len()),
    }
    let parts = s.split("\n");

    let mut elf_id = 0;
    let mut cur_total = 0;

    let mut map: HashMap<i32, i32> = HashMap::new();

    for s in parts{
        if s == "" {
            // chuck into map
            map.insert(elf_id, cur_total);
        
            // next one
            elf_id +=1; 
            cur_total = 0;
            continue;
        }
        let cals:Result<i32, _>= s.parse();
        match cals{
            Ok(v)=>{
                cur_total += v;
            },
            Err(_)=>{
                println!("this.. '{}' somehow doesnt work...", s);
            }
        }
    }

    let mut sorted_values :Vec<i32>= map.values().into_iter().map(|x|{x.to_owned()}).collect();

    sorted_values.sort_by(|x,y|{x.cmp(y).reverse()});

    let sum :i32= sorted_values.iter().take(3).sum();
    println!("three most together are {}", sum);


}

