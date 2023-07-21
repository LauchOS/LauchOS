use crate::shell::shell::BUFFER_LENGTH;

pub fn to_char_array(str: &str) -> [char; BUFFER_LENGTH] {
    let mut arr = ['\0'; BUFFER_LENGTH];  // Initialize array with null characters
    for (i, c) in str.chars().enumerate() {
        arr[i] = c;
    }
    arr
}

#[allow(dead_code)]
pub fn is_equal_to(s: &str, chars: &[char]) -> bool {
    let mut chars_iter = chars.iter();

    for sc in s.chars() {
        match chars_iter.next() {
            Some(&c) if sc == c => continue,
            _ => return false,
        }
    }
    true
}

// compares both char arrays until the first one is empty
pub fn are_chars_equal(chars1: &[char], chars2: &[char]) -> bool{
    for i in 0.. chars1.len(){
        if chars1[i] == '\0'{
            break;
        }
        if chars1[i] != chars2[i]{
            return false;
        }
    }
    true
}