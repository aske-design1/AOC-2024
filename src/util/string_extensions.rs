

pub trait TrimTralingEmptyLine {
    fn remove_trailing_newline(self) -> Self;
    fn trim_trailing_newline(&mut self);
}

impl TrimTralingEmptyLine for String {
    fn remove_trailing_newline(mut self) -> Self {
        if self.chars().last() == Some('\n') {
            self.pop();
        }
        self
    }

    fn trim_trailing_newline(&mut self) {
        if self.chars().last() == Some('\n') {
            self.pop();
        }
    }

}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn check_string_with_newline() {
        assert_eq!("Hello World!", "Hello World!\n".to_string().remove_trailing_newline());
    }

    #[test]
    fn check_string_with_no_newline() {
        assert_eq!("Hello World!", "Hello World!".to_string().remove_trailing_newline());
    }

    #[test]
    fn inline_trimming_with_newline() {
        let mut test_string = "Hello World!\n".to_string();
        test_string.trim_trailing_newline();
        assert_eq!("Hello World!", test_string);
    }

    #[test]
    fn inline_trimming_with_no_newline() {
        let mut test_string = "Hello World!".to_string();
        test_string.trim_trailing_newline();
        assert_eq!("Hello World!", test_string);
    }

}