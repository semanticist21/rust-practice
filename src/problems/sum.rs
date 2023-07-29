use std::io;

fn add_sum() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("failed to read");

    let answer 
    = buffer
    .trim()
    .split(' ')
    .map(|x| x.parse::<i32>().unwrap())
    .reduce(|a,c| a+c)
    .unwrap();

    println!("{}", answer);
}