import fs from 'fs';

class Tree{
    constructor(public height:number, public isVisible = true){
    }
}


const file = fs.readFileSync('input.txt', 'utf8');
const res = file.split("\n").map(x=>[...x.trim()].map(x=>new Tree(Number(x))))
function printMap(trees: Tree[][]){
    trees.forEach(x=>{
        console.log(x.map(y=> y.isVisible ? y.height : 'x').join(" "))
    })
}

function part_one(){
    function getTree(x:number, y:number){
        return res[x][y]
    }
    function getVisibleSides(x:number, y:number){
        //check all trees above, below, to the left and to the right.
        let tree = getTree(x,y)
        let visibleSides = 4;
        for(let xcursor = x-1; xcursor >= 0; xcursor--){
            if(tree.height <= getTree(xcursor, y).height){
                visibleSides -= 1;
                break;
            }
        }
        for(let xcursor = x+1; xcursor < res.length; xcursor++){
            if(tree.height <= getTree(xcursor, y).height){
                visibleSides -= 1;
                break;
            }
        }
    
        for(let ycursor = y-1; ycursor >= 0; ycursor--){
            if(tree.height <= getTree(x, ycursor).height){
                visibleSides -= 1;
                break;
            }
        }
        for(let ycursor = y+1; ycursor < res.length; ycursor++){
            if(tree.height <= getTree(x, ycursor).height){
                visibleSides -= 1;
                break;
            }
        }
    
        return visibleSides;
    }
    
    let final = res.map((x, i)=>{
        return x.map((y, j)=>{
            // console.log()
            const visibile = getVisibleSides(i,j);
            if(visibile === 0){
                return 0 as number;
            }
            getTree(i,j).isVisible = true;
            return 1 as number;
        })
    });
    console.log(final)
    
    console.log(final.flatMap(x=>x).reduce((a,b)=>{return a+b}, 0))    
}

function part_two(){
    function getTree(x:number, y:number){
        return res[x][y]
    }
    function getVisibility(x:number, y:number){
        //check all trees above, below, to the left and to the right.
        let tree = getTree(x,y)
        // tree.isVisible= false;
        // console.log(tree)
        let visibility:number[] = []
        let count = 0;
        for(let xcursor = x-1; xcursor >= 0; xcursor--){
            count++;

            if(tree.height <= getTree(xcursor, y).height){
                break;
            }
        }
        visibility.push(count);
        count = 0;
        for(let xcursor = x+1; xcursor < res.length; xcursor++){
            count++;
            if(tree.height <= getTree(xcursor, y).height){
                break;
            }
        }
        visibility.push(count);
        count = 0;
        for(let ycursor = y-1; ycursor >= 0; ycursor--){
            count++;
            if(tree.height <= getTree(x, ycursor).height){
                break;
            }
        }
        visibility.push(count);

        count = 0;

        for(let ycursor = y+1; ycursor < res.length; ycursor++){
            count++;

            if(tree.height <= getTree(x, ycursor).height){
                break;
            }
        }
        visibility.push(count);
        count = 0;
        // visibility = visibility.map(x=>x+1)

        // console.log(visibility)
    
        return visibility.reduce((a,b)=>{return a*b}, 1);
    }

    let final = res.flatMap((x, i)=>{
        return x.map((y, j)=>{
            return getVisibility(i,j)
        })
    })
    let max = Math.max(...final)
    // printMap(final)
    // console.log(final)
    console.log(max)

}
// part_two();
part_one();