class node{
    constructor(public children: number[], public index: number, public size:number, public name: string, public isDir: boolean = true){
    }
}
class FS{
    nodes = [new node([], 0, 0, "/", true)];
    root_id = 0;
    node_count= 1;
    curdir:string[] = [];

    get_curdir(){
        return this.curdir.length == 0 ? "/" : "/" + this.curdir.join("/");
    }

    print_filesys(){
        let root = this.nodes[0];
        let stack = [{root, level: 0}];
        while(stack.length > 0){
    
            let {root, level} = stack.pop()!;
            let indent = " ".repeat(level * 4);
            console.log(`${indent}-${root.name.split("/").pop() || "/"} ${ root.isDir ?  "(folder) ":root.size}`);
            root.children.forEach((child: number) => {
                stack.push({root: this.nodes[child], level: level + 1});
            });
        }
    }
    get_node_size(nodeid: number): number{
        return this.nodes[nodeid].children.length == 0 ?
        this.nodes[nodeid].size :
            this.nodes[nodeid].children.reduce((a, b) => a + this.get_node_size(b), 0) + this.nodes[nodeid].size;
    
    }
    
    get_dir_id(name:string){
        return this.nodes.filter((node) => node.name == name)[0].index;
    }
    
    add_dir(parent: number, name: string){
        let index= filesys.node_count++
        this.nodes.push(new node([], index, 0, name, true));
        this.nodes[parent].children.push(index);
    }
    
    add_file(parent: number, size: number, name: string){
    
        if(this.nodes[parent].isDir == false){
            console.log("Cannot add file to a file");
            return;
        }
    
        let index= this.node_count++
        this.nodes.push(new node([], index, size, name, false));
        this.nodes[parent].children.push(index);
    
    }
}

const filesys = new FS();


import fs from "fs"
const file = fs.readFileSync("input.txt", "utf-8");
const lines = file.split("\n");


for(let line of lines){
    let [size, command, ...args] = line.split(" ");
    if(command == "cd"){
        if(args[0] == ".."){
            filesys.curdir.pop();
            continue;
        }
        if(args[0] == "/"){
            filesys.curdir = [];
            continue;
        }
        let current = filesys.get_curdir();
        filesys.curdir.push(args[0]);
        // console.log();
        filesys.add_dir(filesys.get_dir_id(current), filesys.get_curdir());
    }
    if(size != "$" && size != "dir"){
        filesys.add_file(filesys.get_dir_id(filesys.get_curdir()), parseInt(size), command);
    }
    if(size == "dir"){
        filesys.add_dir(filesys.get_dir_id(filesys.get_curdir()), command);
    }
}

// print_filesys();

function part_one(){
    const smalldirssize = filesys.nodes.filter((node) => node.isDir).map(x=>filesys.get_node_size(x.index)).filter(x=> x<=100000).reduceRight((a, b) => a + b, 0)
    console.log(smalldirssize);
}


function part_two(){
    let total_diskspace = 70000000;
    let unused_space_needed = 30000000;

    let unused_space = total_diskspace - filesys.get_node_size(filesys.root_id);
    
    let min_dir = filesys.nodes.filter(node=> node.isDir).map(x=>filesys.get_node_size(x.index)).filter(x=> x > unused_space_needed-unused_space);
    min_dir.sort((a, b) => a - b);
    console.log(min_dir[0])
}

part_two();
