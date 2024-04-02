pub fn length_of_longest_substring(s: String) -> i32 {
    let mut string_vec: Vec<char> = Vec::<char>::new();
    let mut longest_length: i32 = 0;
    let mut current_length: i32 = 0;

    'loop1:
    for iteration in 0..s.len() {
        for character in s[iteration..].chars() {
            if string_vec.contains(&character) {
                current_length = 0;
                string_vec.clear();
                continue 'loop1;
            }
            
            current_length += 1;
            
            if current_length > longest_length {
                longest_length = current_length;
            }
            
            string_vec.push(character);
        }
        break;
    }

    longest_length
}

fn main() {
    let result = length_of_longest_substring("pwwkew".to_string());
    println!("{result}");
}