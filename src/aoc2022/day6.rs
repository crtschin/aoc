use std::error::Error;

use crate::util::read_file;

fn process(input: Vec<u8>, marker_size: usize) -> usize {
    let mut jump_table: [usize; 26] = [0; 26];
    let mut buffer: u32 = 0;
    let mut pointer_1 = 0;
    let mut pointer_2 = 0;
    while pointer_2 < input.len() {
        while pointer_1 <= pointer_2 + marker_size {
            let position = input[pointer_1] - b'a';
            if buffer & (1 << position) != 0 {
                pointer_2 = jump_table[position as usize];
                pointer_1 = jump_table[position as usize] + 1;
                buffer = 0;
            }

            buffer |= 1 << (input[pointer_1] - b'a');
            jump_table[position as usize] = pointer_1;
            pointer_1 += 1;
        }
        return pointer_1;
    }
    0
}

pub fn first() -> Result<usize, Box<dyn Error>> {
    Ok(process(read_file("days/2022/day6.txt")?, 4))
}

pub fn second() -> Result<usize, Box<dyn Error>> {
    Ok(process(read_file("days/2022/day6.txt")?, 14))
}

#[cfg(test)]
mod tests {
    mod first {
        #[test]
        fn test1() {
            let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
            let result = super::super::process(input.as_bytes().to_vec(), 4);
            assert_eq!(result, 5);
        }

        #[test]
        fn test2() {
            let input = "nppdvjthqldpwncqszvftbrmjlhg";
            let result = super::super::process(input.as_bytes().to_vec(), 4);
            assert_eq!(result, 6);
        }

        #[test]
        fn test3() {
            let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
            let result = super::super::process(input.as_bytes().to_vec(), 4);
            assert_eq!(result, 10);
        }

        #[test]
        fn test4() {
            let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
            let result = super::super::process(input.as_bytes().to_vec(), 4);
            assert_eq!(result, 11);
        }
        }
    mod second {
        #[test]
        fn test1() {
            let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
            let result = super::super::process(input.as_bytes().to_vec(), 14);
            assert_eq!(result, 23);
        }

        #[test]
        fn test2() {
            let input = "nppdvjthqldpwncqszvftbrmjlhg";
            let result = super::super::process(input.as_bytes().to_vec(), 14);
            assert_eq!(result, 23);
        }

        #[test]
        fn test3() {
            let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
            let result = super::super::process(input.as_bytes().to_vec(), 14);
            assert_eq!(result, 29);
        }

        #[test]
        fn test4() {
            let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
            let result = super::super::process(input.as_bytes().to_vec(), 14);
            assert_eq!(result, 26);
        }
        }
}
