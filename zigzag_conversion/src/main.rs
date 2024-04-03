pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }
    
    let num_rows: usize = num_rows as usize;
    let mut result_string = String::new();
    
    let mut row_index: usize = 0;


    'main_loop:
    loop {
        let mut current_index: usize = row_index;
        let mut dir: bool = true;
        
        if row_index >= num_rows-1 {
            dir = false;
        }
        
        loop {
            let number_of_skips: usize = if dir { ((num_rows - 1) + (num_rows - 1)) - 2 * row_index } else { ((num_rows - 1) + (num_rows - 1)) - 2 * (num_rows - row_index - 1) };
            // println!("{}", number_of_skips);

            match s.get(current_index..current_index+1) {
                
                Some(character) => {
                    result_string += character.to_string().as_str();
                    current_index += number_of_skips;
                    if row_index > 0 && row_index < num_rows-1 { 
                        dir = !dir;
                    }

                }
                None => {
                    row_index += 1;

                    if row_index >= num_rows {
                        break;
                    }
                    
                    continue 'main_loop;
                }
            }
        }

        break;
    }

    result_string
}

fn main() {
    let input: String = "A".to_string();
    println!("{input}");
    let res: String = convert(input, 1);    
    println!("{res}");
}


/*
num_rows = 3
P   A   H   N
A P L S I I G
Y   I   R

num_rows = 4
P     I    N
A   L S  I G
Y A   H R
P     I

num_rows = 5
P       H
A     S I
Y   I   R
P L     I G
A       N

*/