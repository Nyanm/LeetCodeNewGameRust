pub struct Solution;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool
    {
        let mut matrix_match_validity = vec![vec![false; s.len() + 1]; p.len() + 1];
        let s = s.chars().collect::<Vec<char>>();
        let p = p.chars().collect::<Vec<char>>();

        for index_char_p in 0..(p.len() + 1)
        {
            // assign ' ' as empty sign
            let char_current_p = if index_char_p == 0 { ' ' } else { p[index_char_p - 1] };

            for index_char_s in 0..(s.len() + 1)
            {
                let char_current_s = if index_char_s == 0 { ' ' } else { s[index_char_s - 1] };

                let flag_left_up_validity =
                    if index_char_p == 0 && index_char_s == 0 { true }
                    else if index_char_p == 0 || index_char_s == 0 { false }
                    else { matrix_match_validity[index_char_p - 1][index_char_s - 1] };

                // '*' inherit status from up, left and up-up
                if char_current_p == '*'
                {
                    if index_char_p <= 1 { return false; }
                    let char_last_p = p[index_char_p - 2];

                    // corresponds to match a consecutive identical character train
                    let flag_left_validity =
                        if index_char_s == 0 { false }
                        else { matrix_match_validity[index_char_p][index_char_s - 1] };
                    // corresponds to match the first character to a character train (maybe)
                    let flag_up_validity =
                        if index_char_p == 0 { false }
                        else { matrix_match_validity[index_char_p - 1][index_char_s] };
                    // corresponds to match 0 character above
                    let flag_up_up_validity =
                        if index_char_p <= 1 { false }
                        else { matrix_match_validity[index_char_p - 2][index_char_s] };

                    // println!("p: {index_char_p}, s: {index_char_s}, last p: {char_last_p},\
                    //     {flag_left_validity} {flag_up_validity} {flag_up_up_validity}");

                    if char_last_p == '.'
                    {
                        matrix_match_validity[index_char_p][index_char_s] =
                            flag_left_validity || flag_up_validity || flag_up_up_validity;
                    }
                    else
                    {
                        matrix_match_validity[index_char_p][index_char_s] =
                            ((flag_left_validity || flag_up_validity) && (char_current_s == char_last_p))
                            || flag_up_up_validity;  // since match 0 does not need to match actually, the flag is independent of others
                    }
                }
                // common character inherit status from left-up
                else if char_current_p == '.'
                {
                    matrix_match_validity[index_char_p][index_char_s] = flag_left_up_validity;
                }
                // based on left-up validity directly
                else
                {
                    matrix_match_validity[index_char_p][index_char_s] =
                        flag_left_up_validity && (char_current_s == char_current_p);
                }
            }
        }

        // for index_char_p in 0..(p.len() + 1)
        // {
        //     println!("{:?}", matrix_match_validity[index_char_p]);
        // }

        return matrix_match_validity[p.len()][s.len()];
    }
}
