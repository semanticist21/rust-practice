use std::thread::current;

pub fn longest_common_prefix(input: Vec<String>) -> String {
    input
        .into_iter()
        .reduce(|acc, cur| {
            acc.chars()
                .zip(cur.chars())
                .take_while(|(a, c)| a == c)
                .map(|(c, _)| c)
                .collect()
        })
        .unwrap()
}

pub fn longest_common_prefix_old(strs: Vec<String>) -> String {
    let mut vec = vec![];

    for i in 0..usize::MAX {
        let mut marker = '?';
        let mut is_same = true;

        for str in strs.iter() {
            let chars = str.chars().collect::<Vec<char>>();
            let chars_at_index = chars.get(i);
            if let None = chars_at_index {
                is_same = false;
                break;
            }

            let chars_at_index = *chars_at_index.unwrap();

            if marker == '?' {
                marker = chars_at_index;
            } else if marker != chars_at_index {
                is_same = false;
            }
        }

        if is_same {
            vec.push(marker);
        } else {
            break;
        }
    }

    vec.iter().collect::<String>()
}

#[test]
fn test_longest_prefix() {
    let vec = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];

    let result = longest_common_prefix(vec);
    println!("{}", result);

    let vec = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
    let result = longest_common_prefix(vec);
    println!("{}", result);

    let vec = vec!["".to_string()];
    let result = longest_common_prefix(vec);
    println!("{}", result);
}
