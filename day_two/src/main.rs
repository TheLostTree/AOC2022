use std::{path::{Path}, fs::File, io::Read};


struct RpsResult{
    win_points : i32,
    win : bool,
}
// static RPC: [&str;3] = ["rock", "paper" ,"scissors"];
fn main() {
    
    let path = Path::new("./input.txt");
    let mut file = match File::open(path){
        Ok(f)=>f,
        Err(_)=>panic!("!!!!!")
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut total_points = 0;
    // let mut total_wins = 0;
    for str in contents.split("\n"){
        //
        let iter : Vec<&str>= str.split(" ").into_iter().collect();
        // println!("{:?}", iter);
        if iter.len() < 2 {continue;};


        // its actually closer to "my expected outcome" but we're ignoring that.
        let (opp_move, my_move_str) = (iter[0] , iter[1]);
        
        // let result = get_result(my_move_str, opp_move);

        let outcome = parse_move(my_move_str);
        let their_move = parse_move(opp_move);

        let points = (outcome + their_move +3)%3 + 3*(outcome + 1);

        total_points+=points;

        // total_points += result.win_points;
        // if result.win {total_wins+=1};
    }

    println!("total points... {}, you also won games", total_points)
}

fn get_result(my_move_str: &str, their_move_str: &str)->RpsResult{

    let my_move = parse_move(my_move_str);
    let their_move = parse_move(their_move_str);

    let mut points = 0;

    let is_win :bool;
    let val = (their_move-my_move + 3) % 3;


    match val{
        0=>{
            points += my_move + 3;
            is_win = false;
        },
        1=>{
            points += my_move;
            is_win = false;
        },
        2=>{
            is_win = true;
            points += my_move + 6;
        },
        _ => panic!()
    }


    // println!("{} vs {} = {}", RPC.get((their_move-1) as usize).unwrap(), RPC[(my_move-1) as usize], points);


    return RpsResult{ win: is_win, win_points: points};
}


//i gave up on enums :rolling eyes:
fn parse_move(p_move:&str)->i32{
    return match p_move{
        "A"=>1,
        "B"=>2,
        "C"=>3,
        "X"=>-1,
        "Y"=>0,
        "Z"=>1,
        _=>panic!("illegal character '{}'", p_move) 
    }

}