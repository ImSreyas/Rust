use std::collections::HashMap;

fn roman_to_int(s: String) -> i32 {
    let v = vec![("I", 1), ("V", 5), ("X", 10), ("L", 50), ("C", 100), ("D", 500), ("M", 1000)];
    let map: HashMap<_, _> = v.into_iter().collect();
    let mut sum = 0;
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut i = 0;
    loop {
        if chars.len() == 1 {
            sum += map.get(chars[len-1].to_string().as_str()).unwrap();
        }
        let current = map.get(chars[i].to_string().as_str()).unwrap();
        let next = map.get(chars[i+1].to_string().as_str()).unwrap();
        if current < next {
            sum = sum + next - current;
            i+=1;
        } else {
            sum += current;
        }
        i+=1;
        if i == len-1 {
            sum += map.get(chars[len-1].to_string().as_str()).unwrap();
        }
        if i >= len-1 { break } 
    }
    sum
}
fn main() {
    let test_string = String::from("MCMXCIV");
    let out: i32 = roman_to_int(test_string);
    println!("The value is : {}", out);
}