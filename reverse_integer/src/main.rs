pub fn reverse(x: i32) -> i32 {
    let mut x: Vec<char> = x.to_string().chars().into_iter().collect::<Vec<char>>();
    if x[0] == '-' {
        x[0] = ' ';
        x.push('-');
    }
    x.reverse();
    x.iter().collect::<String>().trim().parse().unwrap_or(0)
}

fn main() {
    let res = reverse(-123);
    println!("{}", res);
}
