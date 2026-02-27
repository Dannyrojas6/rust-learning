use std::io;

fn main() {
    // 1 1 2 3 5 8 13 21 34 ...
    println!("==== Fibonacci Array ====");
    let mut n = String::new();
    println!("Please enter n: ");
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Type Error!");
    let mut fibonacci: Vec<u64> = vec![1, 1];
    for t in 0..n {
        if t + 2 < n {
            fibonacci.push(fibonacci[t] + fibonacci[t + 1])
        }
    }
    // let mut check = 0;
    // for i in fibonacci {
    //     print!("{}", i);
    //     if check < n - 1 {
    //         print!(",");
    //         check += 1;
    //     }
    // }
    for i in 0..n {
        print!("{}", fibonacci[i]);
        if i < n - 1 {
            print!(",");
        }
    }
}
