// https://leetcode.cn/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence/description/
struct Solution;

impl Solution {
    fn self_wheel(sentence: String, search_word: String) -> i32 {
        let mut current_word_index = 0;
        let mut scanner: i32 = 0;
        let sentence = sentence.as_bytes();
        let search_word = search_word.as_bytes();
        for target_chr in sentence {
            if *target_chr == b' ' {
                scanner = 0;
                current_word_index += 1;
            } else if scanner >= 0 && *target_chr == search_word[scanner as usize] {
                scanner += 1;
                if scanner == search_word.len() as i32 {
                    return current_word_index + 1;
                }
            } else {
                scanner = -1;
            }
        }
        -1
    }

    fn std(sentence: String, search_word: String) -> i32 {
        let sp = sentence.split(' ');
        for (i, s) in sp.enumerate() {
            if s.starts_with(&search_word) {
                return i as i32 + 1;
            }
        }
        -1
    }
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        Self::std(sentence, search_word)
    }
}

#[cfg(test)]
mod tests {
    use crate::lc_rating::lc_1126_check_if_a_word_occurs_as_a_prefix_of_any_word_in_a_sentence::Solution;

    #[test]
    fn test() {
        assert_eq!(4, Solution::is_prefix_of_word("i love eating burger".to_string(), "burg".to_string()));
    }
}