use std::cmp::max;
use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32
    {
        let vec_s = s.chars().collect::<Vec<char>>();
        let mut map_last_seen: HashMap<char, usize> = HashMap::new();

        let mut answer: i32 = 0;
        let mut ptr_back: i32 = -1;

        for index in 0..vec_s.len()
        {
            let current_char = vec_s[index];
            let last_seen = map_last_seen.get(&current_char);
            // seen before
            if last_seen.is_some()
            {
                let last_seen = *last_seen.unwrap();
                // advance ptr_back & delete old record
                while ptr_back < last_seen as i32
                {
                    ptr_back += 1;
                    let _ = &map_last_seen.remove(&vec_s[ptr_back as usize]);
                }
            }

            // update index info and answer
            map_last_seen.insert(current_char, index);
            answer = max(answer, index as i32 - ptr_back);
        }

        return answer;
    }
}
