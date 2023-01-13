import fs from "fs";
const a_num = "a".charCodeAt(0);
const z_num = "z".charCodeAt(0);
const A_num = "A".charCodeAt(0);
const Z_num = "Z".charCodeAt(0);

function getPriority(c: string){
    //a-z -> 1-26
    if(c?.length != 1) return 0;
    // if()

    let num = c.charCodeAt(0);
    if(a_num <= num && num <= z_num){
        return num - a_num + 1;
    }

    if(A_num <= num && num <= Z_num){
        return num - A_num + 27;
    }
}

function partone(){
    const file = fs.readFileSync("./input.txt", "utf-8");
    let lines = file.split("\n").map(x=>[x.substring(0,x.length/2), x.substring(x.length/2)]);
    let priorities = lines.map((a)=>{
        let f = new Set([...a[0]])
        let sharedLetter = [...a[1]].filter(x=>f.has(x))[0];
        return getPriority(sharedLetter)!;
    })
    console.log(priorities.reduce((x,y)=>{return x+y}))
}

function parttwo(){
    const file = fs.readFileSync("./input.txt", "utf-8");
    let lines = file.split("\n").map(x=>[...x]);
    //every three lines is a "group"
    let sum = 0;
    const chunkSize = 3;
    for (let i = 0; i < lines.length; i += chunkSize) {
        const chunk = lines.slice(i, i + chunkSize);
        let remain = chunk.reduce((p,v)=>{
            return p.filter(x=>v.includes(x))
        })[0]
        sum += getPriority(remain)!;
    }
    console.log(sum)
}


partone();
parttwo()