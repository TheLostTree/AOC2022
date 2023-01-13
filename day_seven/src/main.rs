use std::{path::Path, fs::File, io::Read, fmt::Debug};

const INPUT: &str = "input.txt";

fn main() {
    part_one();
    part_two();
}

#[derive(Clone, Debug)]
struct Node{
    children: Vec<usize>,
    index: usize,
    size: usize,
    is_dir: bool,
    name: String,
}
impl Node{
    fn new(index: usize, size: usize, name: String, is_dir: bool)->Node{
        Node{
            children: Vec::<usize>::new(),
            index,
            size,
            is_dir,
            name: name.to_string(),
        }
    }
}

// indexed storage pattern? 
struct FileSystem{
    nodes: Vec<Node>,
    root_id: usize,
    cur_dir: Vec<String>,
    node_count: usize,
}

impl FileSystem{
    fn new()->FileSystem{
        FileSystem{
            nodes: vec![Node::new(0, 0, "/".to_string(), true)],
            root_id: 0,
            cur_dir: Vec::<String>::new(),
            node_count: 0,
        }
    }

    fn get_node_size(&mut self, node_id:usize)->usize{
        if self.nodes[node_id].children.len() == 0{
            return self.nodes[node_id].size;
        }else{
            let mut size = 0;
            for child in self.nodes[node_id].children.clone(){
                size = size + self.get_node_size(child);
            }
            // return 0;
            return size;
        }
    }

    fn get_curdir(&mut self)->String{
        if self.cur_dir.len() == 0{
            return "/".to_string();
        }
        let mut s = "".to_string();

        for dir in &self.cur_dir{
            s.push_str("/");
            s.push_str(dir);
        }
        s
    }

    fn add_dir(&mut self, parent: usize, name:&str){
        self.node_count = self.node_count + 1;
        let index = self.node_count;
        let node = Node::new(index, 0, name.to_string(), true);
        self.nodes.push(node);
        self.nodes[parent].children.push(index);
    }

    fn add_file(&mut self, parent: usize, size: usize, name:&str){
        self.node_count = self.node_count + 1;
        let index = self.node_count;
        let node = Node::new(index, size, name.to_string(), false);
        self.nodes.push(node);
        self.nodes[parent].children.push(index);
    }

    fn get_dir_id(&mut self, name:&str)->Option<usize>{
        for node in &self.nodes{
            if node.name == name{
                return Some(node.index);
            }
        }
        None
    }
}

struct NodeDebug{
    node: usize,
    level: i32,
}

impl Debug for FileSystem{
    // fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //     f.debug_struct("FileSystem").field("nodes", &self.nodes.clone()).field("root_id", &self.root_id).field("cur_dir", &self.cur_dir).field("node_count", &self.node_count).field("veclen", &self.nodes.len()).finish()
    // }
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)->std::fmt::Result{
        let mut s = String::new();

        let mut stack = vec![NodeDebug{
            node: self.root_id,
            level: 0,
        }];


        while stack.len() > 0{
            let nodedbg = stack.pop().unwrap();

            if nodedbg.node >= self.nodes.len() {
                println!("node {} out of bounds", nodedbg.node);
                continue;
            }
            let node = &self.nodes[nodedbg.node];
            let indent = " ".repeat(nodedbg.level as usize * 4);
            
            s.push_str(&format!("{}-{} {}\n", indent, 
                match node.name.split("/").last().unwrap(){
                    ""=>"/",
                    s=>s,
                }, 
                if node.is_dir{
                    "(folder)".to_owned()
                }else{
                    format!("({} bytes)", node.size)
                }
            ));

            // nodedbg.level += 1;

            for child in node.children.clone(){

                stack.push(NodeDebug{
                    node: child,
                    level: nodedbg.level + 1,
                });


            }
            
        }


        write!(f, "{}", s)

    }
    
}


fn part_one(){
    let mut fs = common();
    let dirs = fs.nodes.iter().filter(|x| x.is_dir).map(|x|x.index)
    .collect::<Vec<usize>>();

    let mut sum = 0;
    for dir in dirs{
        let size = fs.get_node_size(dir);
        if size < 100000{
            // println!("{}: {}", fs.nodes[dir].name, size);
            sum = sum + size;

        }
    }
    println!("part 1: sum: {}", sum);
}

fn part_two(){
    let mut fs = common();
    let total_diskspace = 70000000;
    let unused_space_needed = 30000000;

    let unused_space = total_diskspace - fs.get_node_size(fs.root_id);

    let dirs :Vec<usize>= fs.nodes.iter().filter(|x| x.is_dir).map(|x|x.index).collect();

    let min_dir = dirs.iter().filter_map(|x| {
        let size = fs.get_node_size(*x);
        if size > unused_space_needed-unused_space{
            Some(size)
        }else{
            None
        }
    }).min().unwrap();

    println!("part 2: min dir size: {}", min_dir);
    
}

fn common()->FileSystem{
    let contents = read_file(INPUT);

    let lines = contents.split("\n");
    let mut fs = FileSystem::new();

    for line in lines{
        //process line
        let mut args = line.split(" ");
        let token = args.next().unwrap();
        if token == "$"{
            // commandline
            let command = args.next().unwrap();
            if command == "cd" {
                let arg = args.next().unwrap();

                match arg{
                    ".."=>{
                        fs.cur_dir.pop();
                    },
                    "/"=>{
                        fs.cur_dir.clear();
                    },
                    _=>{
                        let current = fs.get_curdir();
                        fs.cur_dir.push(arg.to_string());
                        let parent = fs.get_dir_id(&current).unwrap();
                        let new_dir = fs.get_curdir();
                        fs.add_dir(parent, &new_dir);
                    }
                }
            
            }
        } else{
            // token = size or maybe dir.
            if token == "dir" {
                // let name = args.next().unwrap();
                // let curdir = fs.get_curdir();
                // let parent = fs.get_dir_id(&curdir).unwrap();
                // fs.add_dir(parent, name);
                continue;
            }

            let size = token.parse::<usize>().unwrap();
            let name = args.next().unwrap();
            let curdir = fs.get_curdir();
            let parent = fs.get_dir_id(&curdir).unwrap();
            fs.add_file(parent, size, name);

        }
    }
    // println!("{:?}", fs);


    fs
    
}


fn read_file(in_path: &str)->String{
    let path = Path::new(in_path);
    let mut file = match File::open(path){
        Ok(k)=>k,
        Err(_)=>panic!("L")
    };
    let mut contents = String::new();
    _ = file.read_to_string(&mut contents);
    contents
}