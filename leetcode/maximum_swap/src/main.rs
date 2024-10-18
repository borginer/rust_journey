// https://leetcode.com/problems/maximum-swap

fn main() {
    let num = maximum_swap(99448); 
    println!("max swap number: {num}");
}

fn maximum_swap(num: i32) -> i32 {
    let mut digit_vec: Vec<char> = num.to_string().chars().collect();

    for filter in (2..=10).rev() {    
        let idx = get_last_largest_digit(&digit_vec, filter);
        match find_swap_idx(&digit_vec, idx) {
            Some(i) => {
                digit_vec.swap(idx, i);
                break;
            },
            None => {},
        };
    }  


   return digit_vec
        .iter()
        .collect::<String>()
        .parse()
        .unwrap(); 
}

fn find_swap_idx(digits: &Vec<char>, idx: usize) -> Option<usize> {
    let idx_num = digits[idx].to_digit(10).unwrap();
    
    for (i, digit) in digits.iter().enumerate() {
        let num = digit.to_digit(10).unwrap();
        if idx > i && idx_num > num {
            return Some(i);
        }
    }

    None
}

// ignores digits above filter
fn get_last_largest_digit(digits: &Vec<char>, filter: u32) -> usize {
    let mut largest = 0;
    let mut idx = 0;

    for (i, digit) in digits.iter().enumerate() {
        let num = digit.to_digit(10).unwrap();
        if num >= largest && num < filter {
            idx = i;
            largest = num;
        }
    } 

    idx
}
