use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map_corresponding_index: HashMap<i32, i32> = HashMap::new();

        for (index, num) in nums.iter().enumerate() {
            map_corresponding_index.insert(target - num, index as i32);
        }

        for (index, num) in nums.iter().enumerate() {
            let index_another = map_corresponding_index.get(&num);
            if !index_another.is_none() {
                let index_another= *index_another.unwrap();
                if index as i32 == index_another { continue; }
                return vec![index as i32, index_another];
            }
        }

        panic!("answer not found");
    }
}
