use std::fs::File;
use std::io::{BufRead, BufReader};


//part 2 


fn dampener(levels: &Vec<i32>) -> bool {

    for level in 0..levels.len(){

        //creating new vector 
        let mut modified = levels.clone();
        modified.remove(level);


        //Check with is_safe

        if is_safe(&modified){
            return true;
        }
    }

    false
}





//part 1
fn is_safe(levels: &Vec<i32>) -> bool {

    let mut direction = 0; 
    for level in 1..levels.len(){
        let diff = levels[level] - levels[level-1]; 


        //check difference
        
        if diff.abs() < 1 || diff.abs() > 3 {
            return false; 
        }

        if direction == 0{

            if diff > 0 {

                direction = 1; 
            } else if diff < 0 {
                direction = -1; 
            }
        } else { 

            if (direction == 1 && diff < 0) || (direction == -1  && diff > 0) {
                return false; 
        }
    }
 }

 true 
}


fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?; 
    let reader = BufReader::new(file);
    let mut safe = 0; 


    for line in reader.lines() {

        let line=line?;
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        //println!("{:?}" , levels);

        if is_safe(&levels) { 
            safe +=1;
        } else if dampener(&levels){
            safe+=1;
        }
    }



    println!("safe reports {}", safe);
    Ok(())

}
