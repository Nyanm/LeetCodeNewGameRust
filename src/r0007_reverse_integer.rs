pub struct Solution;
impl Solution {
    pub fn reverse(x: i32) -> i32
    {
        let mut x = x;
        if x == i32::MIN || x == 0 { return 0; }
        let base: u64 = 10;

        // transform number to digits
        let mut vec_digits: Vec<i64> = vec![];
        while x != 0
        {
            vec_digits.push((x % 10) as i64);
            x = x / 10;
        }

        // transform digits to number
        let mut answer: i64 = 0;
        for digit_index in 0..vec_digits.len()
        {
            answer += (base.pow(digit_index as u32) as i64 * vec_digits[vec_digits.len() - digit_index - 1]) as i64;
        }

        // boundary check
        if answer > i32::MAX as i64 || answer < i32::MIN as i64 { return 0; }
        return answer as i32;
    }
}