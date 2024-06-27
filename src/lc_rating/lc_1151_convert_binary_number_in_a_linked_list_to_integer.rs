// https://leetcode.cn/problems/convert-binary-number-in-a-linked-list-to-integer/description/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}
impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        match head {
            Some(mut node) => {
                let mut rst = node.val;
                while let Some(next) = node.next {
                    rst *= 2;
                    rst += next.val;
                    node = next;
                }
                rst
            }
            None => {
                0
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}