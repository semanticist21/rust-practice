pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();

    for (index, i) in nums.iter().enumerate(){
        map.insert(*i, index);
    }

    for i in 0..nums.len(){
        let find_num = target - nums[i];

        let check = map.get(&find_num);
        if let Some(value) = check{

            if *value == i{
                continue;
            }
            
            let value = *value as i32;
            let i = i as i32;

            if value > i {
                return vec![i, value];
            } else{
                return vec![value, i];
            }
        }
    }

    vec![9,9]
}