
use super::*;

#[allow(dead_code)]
pub struct Day9 {
    input: Vec<u8>
}

impl Day9 {
    pub fn new(input: &str) -> Self {
        let input = 
        input.as_bytes().iter().map(|byte| *byte - b'0').collect();
        Self { input }
    }

    fn two_pointer(mut disk: Vec<u8>) -> u64 {
        let len = disk.len();

        let (mut i, mut j) = (0, len - 1);
        let mut fir_ptr = unsafe { disk.get_unchecked_mut(i) as *mut _ };
        let mut sec_ptr = unsafe { disk.get_unchecked_mut(j) as *mut _ };

        let mut checksum = 0;
        let mut counter = 0;

        let mut start_idx_val = 0 as u64;
        let mut end_idx_val = (len / 2) as u64;
        while i < j {
            if unsafe { *fir_ptr == 0_u8 } {
                i += 1;
                if i % 2 == 0 { start_idx_val+=1; }

                if i >= j { 
                    break;                
                }
                fir_ptr = unsafe { disk.get_unchecked_mut(i) as *mut _ };
                if unsafe { *fir_ptr == 0u8 } { 
                    continue; 
                }
            } 
            
            if unsafe { *sec_ptr == 0u8 } {
                if j < 2 { break; }
                j -= 2;
                if i >= j { 
                    break;                
                }
                end_idx_val -= 1;
                
                sec_ptr = unsafe { disk.get_unchecked_mut(j) as *mut _ };
                if unsafe { *sec_ptr == 0u8 } { 
                    continue; 
                }
                
            } 
            if i % 2 == 0 {
                checksum += counter * start_idx_val;
            } else {
                checksum += counter * end_idx_val;
                unsafe { *sec_ptr -= 1_u8; } 
            }
            unsafe { *fir_ptr -= 1_u8; } 
            counter += 1;
        }
        for i in i..disk.len() {
            if i % 2 == 1 { continue; }
            let val = disk.get_mut(i).unwrap();
            while *val > 0 {
                checksum += counter * start_idx_val;

                *val -= 1;
                counter+=1;
            }   
            start_idx_val+=1;
        }
        checksum
    }

}

impl Solution for Day9 {
    fn part1(&self) -> String { Self::two_pointer(self.input.clone()).to_string() }
    fn part2(&self) -> String { 0.to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "2333133121414131402";
    
    #[test] fn test1() {
        assert_eq!(Day9::new(TEST).part1(), 1928.to_string());
    }
    #[test] fn test1_2() {
        assert_eq!(Day9::new("13345").part1(), 57.to_string());
    }

    #[test] fn test2() {
        assert_eq!(Day9::new(TEST).part2(), 0.to_string());
    }
}

