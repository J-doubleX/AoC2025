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
    let mut abc: Vec<i128> = Vec::new();
    let mut tt: i128 = 0;

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
                    let mut p = 1;
                    while p < len{
                        if len % p == 0{
                            let mut aaa: Vec<String> = Vec::new();
                            for i in (0..len).step_by(p) {
                                let ln = &s[i..i+p];
                                aaa.push(ln.to_string());
                            }
                            if aaa.iter().all(|x| *x == aaa[0]){
                                if p == 2 {
                                    abc.push(s.parse().unwrap());
                                }
                                lll.push(s.parse().unwrap());
                                break;
                            }
                        }
                        p+=1;
                    }
                }
            }
        }
        let mut b: i128 = 0;
        for i in abc{
            b += i;
        }
        for i in lll{
            tt += i;
        }
        println!("{b}");
        println!("{tt}");




    Ok(())
}
