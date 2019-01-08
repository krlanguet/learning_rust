// First learning Project - Integer Vector Operations

use std::collections::HashMap;

struct IntData {
    ints: Vec<u32>,
    sum: u32,
    counts: HashMap<u32, u32>,
}

impl IntData {
    fn get() -> IntData {
        use std::io::stdin;

        'input: loop {
            println!("Please Enter a sequence of non-negative integers");
            println!("\te.g. \"0 24 3 7\":");
        
            let mut _str = String::new();

            stdin().read_line(&mut _str)
                .expect("Sorry, Failed to read line");

            let _strs: Vec<&str> = _str.trim()
                .split_whitespace().collect();

            if _strs.len() == 0 {
                println!("Sorry, You didn't enter anything");
                continue 'input;
            }

            let mut _ints: Vec<u32> = vec![];
            let mut _sum: u32 = 0;
            let mut _counts: HashMap<u32, u32> = HashMap::new();

            for x in &_strs {
                match x.parse::<u32>() {
                    Ok(i) => {
                        _ints.push(i);
                        _sum += i;
                        *_counts.entry(i).or_insert(0) += 1;
                    },
                    _ => {
                        println!("Sorry, You entered {} which is not an integer.", x);
                        continue 'input;
                    },
                }
            }

            _ints.sort();

            return IntData {
                ints : _ints, 
                sum : _sum,
                counts : _counts,
            }
        }
    }

    fn mean(&self) -> f64 {
        self.sum as f64 / (self.ints.len() as f64)
    }

    fn median(&self) -> f64 {
        let len = self.ints.len();
        match len % 2 {
            0 => (self.ints[ len/2 ] as f64 + self.ints[ (len/2) - 1] as f64) / 2.0,
            _ => self.ints[ len/2 ] as f64,
        }
    }

    fn mode(&self) -> u32 {
        self.counts
            .iter()
            .max_by_key(|&(_, value)| value)
            .map(|(key, _)| *key)
            .unwrap()
    }
}


fn main() {
    let data = IntData::get();
    
    //println!("You entered {:?}", data.ints);
    
    println!("Mean: {}", data.mean());
    println!("Median: {}", data.median());
    println!("Mode: {}", data.mode());
}
