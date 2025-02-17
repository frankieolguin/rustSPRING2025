// this code runs on leetcode
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> 
    {
        let mut ans: Vec<String> = vec![];
        for num in 1..=n{
            let by_3 = num%3 ==0;
            let by_5 = num%5 ==0;
            match(by_3,by_5) {
                (true,false) => ans.push("Fizz".to_string()),
                (false,true) => ans.push("Buzz".to_string()),
                (true,true)  => ans.push("FizzBuzz".to_string()),
                _ => ans.push(num.to_string()),
            }
        }
        return ans;
    }
}