fn main() {
    let a: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let even: Vec<&i32> = a.iter().filter(|&num| num % 2 == 0).collect();
    let odd: Vec<&i32> = a.iter().filter(|&num| num % 2 == 1).collect();
    println!("First array is {:?}", a);
    println!("Even array is {:?}", even);
    println!("Odd array is {:?}", odd);
}