fn main() {
    let nat_nums: Vec<i32> = (1..=20).collect();

    let even = |num: &&i32| *num % 2 == 0;

    let even_nums: Vec<i32> = nat_nums.iter().filter(even).cloned().collect();

    println!("The even numbers filtered from the natural numbers vector [1 to 20]: {:?}", even_nums);
}
