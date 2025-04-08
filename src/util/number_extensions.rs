

pub trait CalcDigits {
    type Output;
    fn number_digits(&self) -> Self::Output; 
}

impl CalcDigits for u64 {
    type Output = usize;

    fn number_digits(&self) -> Self::Output {
        let mut n = *self;
        if n == 0 { return 1; }
        let mut digits = 0;
        while n > 0 {
            n /= 10;
            digits += 1;
        }
        digits
    }
}