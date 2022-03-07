
pub fn run(){

    greet("hi", "Chuchu");

    // Bind function values to variables
    let sum = get_sum(1, 200);
    println!("The sum is: {}", sum);

    //Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3, 1))

}

fn greet (greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn get_sum (n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}