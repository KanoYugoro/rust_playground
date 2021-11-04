use std::io;

fn main() {
    println!("This program will perform Euler's Sieve up to the given number.");
    println!("Please input the number to check up to (Max 10,000).");

    let mut max_str = String::new();
    io::stdin()
        .read_line(&mut max_str)
        .expect("Failed to read line");

    println!("Now attempting the sieve up to: {}", max_str);

    let max: u32 = max_str.trim().parse().expect("Please type a number!");
    let mut nums: [u32; 10000] = [0; 10000];

    for i in 2..max as usize {
        nums[i] = i as u32;
    }
    for i in 2..max as usize {
        for j in 2..max as usize {
            if j*i < max as usize {
                // println!("Setting {} to 0", i*j);
                nums[j*i] = 0;
            }
        }

        if nums[i] != 0 {
            println!("{}", nums[i])
        }
    }
}
