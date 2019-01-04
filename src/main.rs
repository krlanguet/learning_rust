// First learning Project - Integer Vector Operations

fn get_sorted_ints() -> Vec<u32> {
    use std::io::stdin;

    'outer: loop {
        let mut input = String::new();
        let mut input_nums: Vec<u32> = vec![];

        println!("Please Enter a sequence of non-negative integers");
        println!("\te.g. \"0 24 3 7\":");
    
        stdin().read_line(&mut input)
            .expect("Sorry, Failed to read line");

        let input: Vec<&str> = input.trim()
            .split_whitespace().collect();

        for x in &input {
            match x.parse::<u32>() {
                Ok(i) => input_nums.push(i),
                _ => {
                        println!("Sorry, try again.");
                        continue 'outer;
                }
            }
        }

        input_nums.sort();
        return input_nums;
    }
}

fn main() {
    let nums = get_sorted_ints();


    println!("You entered {:?}", nums);
}
