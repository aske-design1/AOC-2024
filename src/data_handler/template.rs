
pub fn get_template(day_num: u8) -> String 
{
format!(r#"
use super::*;

#[allow(dead_code)]
pub struct Day{day_num} {{
    input: String
}}

impl Day{day_num} {{
    pub fn new(input: &str) -> Self {{
        let splitter = if input.contains("\r\n") {{ "\r\n" }} else {{ "\n" }};
        let input = input.to_string();
        Self {{ input }}
    }}
}}

impl Solution for Day{day_num} {{
    fn part1(&self) -> String {{ 0.to_string() }}
    fn part2(&self) -> String {{ 0.to_string() }}
}}

#[cfg(test)]
mod tests {{
    use super::*;
    const TEST: &str = "INPUT HERE";
    
    #[test] fn test1() {{
        assert_eq!(Day{day_num}::new(TEST).part1(), 0.to_string());
    }}
    #[test] fn test2() {{
        assert_eq!(Day{day_num}::new(TEST).part2(), 0.to_string());
    }}
}}

"#)
}


mod template {
    use crate::solutions::Solution;

    #[allow(dead_code)]
    pub struct Template {
        input: String
    }

    impl Template {
        #[allow(dead_code)]
        pub fn new(input: &str) -> Self {
            let input = input.to_string();
            Self { input }
        }
    }

    impl Solution for Template {
        fn part1(&self) -> String { 0.to_string() }
        fn part2(&self) -> String { 0.to_string() } 
    }

    #[cfg(test)]
    mod tests {
        use crate::solutions::Solution;
        use super::*;
        const TEST: &str = "INPUT HERE";
        
        #[test] fn test1() {
            assert_eq!(Template::new(TEST).part1(), 0.to_string());
        }
        #[test] fn test2() {
            assert_eq!(Template::new(TEST).part2(), 0.to_string());
        }
    }
}