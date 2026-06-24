use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

fn read_file_to_lines<P>(filename: P) -> Result<Vec<String>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}


fn main() -> Result<()> {
    let lines = read_file_to_lines("src/input.txt")?;
    let mut lll: Vec<i128> = Vec::new();

        for line in &lines {
            let clean_parts = line.split(",").filter(|s| !s.is_empty());
            for part in clean_parts{
                let mut a = part.split("-");
                let l = a.next().unwrap();
                let r = a.next().unwrap();
                let left: i128 = l.parse().unwrap();
                let right: i128 = r.parse().unwrap();
                for i in left..=right{
                    let s = i.to_string();
                    let len = s.len();
                    if len % 2 == 0{
                        let b = len /2;
                        let ln = &s[0..b];
                        let rn = &s[b..];
                        if ln == rn{
                            lll.push(i);
                        }
                    }
                }
            }
        }

        let mut total: i128 = 0;
        for i in lll{
            total += i;
        }
        println!("{total}");




    Ok(())
}
