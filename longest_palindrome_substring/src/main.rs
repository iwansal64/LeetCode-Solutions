pub fn longest_palindrome(s: String) -> String {
    let mut palindromes: Vec<Vec<char>> = Vec::new();
    let mut current_string: Vec<char> = Vec::new();
    let mut longest_length: usize = 0;
    let s = s.chars().collect::<Vec<char>>();
    
    let mut start_index: usize = 0;

    let mut reversed_s = s.clone();
    reversed_s.reverse();

    if s == reversed_s {
        return s.into_iter().collect();
    }
    
    while start_index < s.len() {
        
        current_string.push(s[start_index]);

        for character in &s[start_index+1..] {
            current_string.push(character.clone());
            
            let mut reversed_string = current_string.clone();
            reversed_string.reverse();
            
            if current_string == reversed_string {
                // println!("{:?} is the same!", current_string);
                if current_string.len() > longest_length {
                    longest_length = current_string.len();
                }
                palindromes.push(current_string.clone());
            }

        }

        current_string.clear();

        start_index += 1;
        
    }

    for palindrome in palindromes {
        if palindrome.len() >= longest_length {
            return palindrome.into_iter().collect();
        }
    }
    
    s[0].to_string()
}

fn main() {
    let res: String = longest_palindrome("uhrfjotnewtodhmbplsaolnpcdaohiytmfllukijouxipvqohtsgxbtfoxyfkfczkfwhzimbefiohmtimrcxbpgcxogystdkcqujvbxsgirpccdnvejtljftwkdpsqpflzwruwwdzovsbmwbcvlftkjnxqaguvtsycylqzquqkbnybnbaeahbxejhphwrpmymcemuhljwtuvxefqfzjhskuqhifydkxpnfwfxkpeexnjltfqwfvchphmtsrsyayxukvmlqodshqwbeaxhcxdbssnrdzvxtusngwqdxvluauphmmbwmgtazjwvolenegwbmjfwprfuswamyvgrgshqocnhirgyakbkkggviorawadzhjipjjgiwpelwxvtaegauerbwpalofrbghfhnublttqtcmqskcocwwwxpnckrnbepusjyohsrretrqyvgnbezuvwmzizcefxyumtdwnqjkgsktyuacfpnqocqjxcurmipjfqmjqrkdeqsfseyigqlwmzgqhivbqalcxhlzgtsfjbdbfqiedogrqasgmimifdexbjjpfusxsypxobxjtcwxnkpgkdpgskgkvezkriixpxkkattyplnpdbdifforxozfngmlgcunbnubzamgkkfbswuqfqrvzjqmlfqxeqpjaqayodtetsecmfbplscmslpqiyhhykftzkkhshxqvdwmwowokpluwyvavwvofwqtdilwqjgrprukzyhckuspyzaoe".to_string());

    print!("{res}");
}
