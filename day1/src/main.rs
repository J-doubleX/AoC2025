use std::fs::File;
use std::io::{BufRead,BufReader,Result};
use std::path::Path;


fn read_file_to_lines<P>(filename: P) -> Result<Vec<String>> 
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn main() -> Result<()>{

    let start = 50;
    let mut cur = start;
    let lines = read_file_to_lines("src/input.txt")?;
    let mut c = 0;
    let mut b = 0;


    for line in &lines{
        let t: i32 = line[1..].parse().unwrap();
        if line.starts_with('L') {
            let stz = if cur!=0 {cur} else {100};
            if t >= stz {
                c += 1 + (t - stz) / 100;
            }
            
            cur = (cur - t).rem_euclid(100);


            
        }else {
            let stz = if cur != 0 {100 - cur} else {100};
            if t >= stz {
                c += 1 + (t - stz) / 100;
            }

            cur = (t + cur).rem_euclid(100);


        }
        if cur == 0 {
            b += 1;
        }
    }
    println!("{}", b);
    println!("{}", c);
    Ok(())
}
