use std::io;

fn diff() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("read error");

    let answer = buffer
        .split('\n')
        .next()
        .unwrap()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .reduce(|a, c| a - c)
        .unwrap();

    println!("{}", answer);
}