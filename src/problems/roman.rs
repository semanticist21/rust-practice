pub fn int_to_roman_solution(num: i32) -> String {
    let mut retv = String::with_capacity(10); // Pre-allocate according to the max length in the problem statement

    let mut num = num;
    loop {
        let (string, sub): (&str, i32) = match num {
            i if i >= 1000 => ("M", 1000),
            900..=999 => ("CM", 900),
            500..=899 => ("D", 500),
            400..=499 => ("CD", 400),
            100..=399 => ("C", 100),
            90..=99 => ("XC", 90),
            50..=89 => ("L", 50),
            40..=49 => ("XL", 40),
            10..=39 => ("X", 10),
            9 => ("IX", 9),
            5..=8 => ("V", 5),
            4 => ("IV", 4),
            1..=3 => ("I", 1),
            0 => {
                return retv;
            }
            _ => ("", 0),
        };

        println!("{}", string);
        println!("{}", sub);
        num -= sub;
        retv.push_str(string);
    }
}

pub fn int_to_roman(num: i32) -> String {
    let four_digit = (num / 1000) % 10;
    let three_digit = (num / 100) % 10;
    let two_digit = (num / 10) % 10;
    let one_digit = (num) % 10;

    let mut four_str = "";
    let mut three_str = "";
    let mut two_str = "";
    let mut one_str = "";

    four_str = match four_digit {
        1 => "M",
        2 => "MM",
        3 => "MMM",
        4 => "MMMM",
        _ => "",
    };

    three_str = match three_digit {
        1 => "C",
        2 => "CC",
        3 => "CCC",
        4 => "CD",
        5 => "D",
        6 => "DC",
        7 => "DCC",
        8 => "DCCC",
        9 => "CM",
        _ => "",
    };

    two_str = match two_digit {
        1 => "X",
        2 => "XX",
        3 => "XXX",
        4 => "XL",
        5 => "L",
        6 => "LX",
        7 => "LXX",
        8 => "LXXX",
        9 => "XC",
        _ => "",
    };

    one_str = match one_digit {
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        6 => "VI",
        7 => "VII",
        8 => "VIII",
        9 => "XI",
        _ => "",
    };

    format!("{}{}{}{}", four_str, three_str, two_str, one_str)
}

pub fn int_to_roman_slow(num: i32) -> String {
    let mut vec = vec![];

    let mut four_num = num / 1000;
    while four_num != 0 {
        let num_str = match four_num {
            num if num > 0 => {
                four_num = four_num - 1;
                "M"
            }
            _ => "",
        };

        vec.push(num_str);
    }

    let mut third_num = (num / 100) % 10;
    while third_num != 0 {
        let num_str = match third_num {
            num if num == 4 => {
                third_num = third_num - 4;
                "CD"
            }
            num if num < 5 => {
                third_num = third_num - 1;
                "C"
            }
            num if num == 9 => {
                third_num = third_num - 9;
                "CM"
            }
            num if num > 5 => {
                third_num = third_num - 5;
                "D"
            }
            _ => "",
        };

        vec.push(num_str);
    }

    let mut second_num = (num / 10) % 10;
    while second_num != 0 {
        let num_str = match second_num {
            num if num == 4 => {
                second_num = second_num - 4;
                "XL"
            }
            num if num < 5 => {
                second_num = second_num - 1;
                "X"
            }
            num if num == 9 => {
                second_num = second_num - 9;
                "XC"
            }
            num if num > 5 => {
                second_num = second_num - 5;
                "L"
            }
            _ => "",
        };

        vec.push(num_str);
    }

    let mut first_num = num % 10;
    while first_num != 0 {
        let num_str = match first_num {
            num if num == 4 => {
                first_num = first_num - 4;
                "IV"
            }
            num if num < 5 => {
                first_num = first_num - 1;
                "I"
            }
            num if num == 9 => {
                first_num = first_num - 9;
                "IX"
            }
            num if num > 5 => {
                first_num = first_num - 5;
                "V"
            }
            _ => "",
        };

        vec.push(num_str);
    }

    let result = vec.concat();

    result
}

#[test]
fn test_roman() {
    let result = int_to_roman_solution(1994);
    println!("{}", result);
}
