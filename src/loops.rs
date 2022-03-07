
pub fn run(){
    let mut count = 0;
    

    // infinite loop
    loop {
        count += 1;
        println!("Loop Number: {}", count);

        if count == 20 {
            break;
        }
    }

    // While loop (fizzbuzz) /3 => fizz; /5 => buzz; /15 => fizzbuzz
    let mut fizzbuzz_count = 0;
    while fizzbuzz_count <= 30 {
        
        if fizzbuzz_count % 15 == 0 {
            println!("fizzbuzz: {}", fizzbuzz_count);
        } else if fizzbuzz_count % 5 == 0 {
            println!("buzz: {}", fizzbuzz_count);
        } else if fizzbuzz_count % 3 == 0 {
            println!("fizz: {}", fizzbuzz_count);
        } else {
            println!("{}", fizzbuzz_count);
        }
        fizzbuzz_count +=1;
    }

    // For loop (fizzbuzz) /3 => fizz; /5 => buzz; /15 => fizzbuzz
    for fizzbuzz_count in 30..50 { 
        if fizzbuzz_count % 15 == 0 {
            println!("fizzbuzz: {}", fizzbuzz_count);
        } else if fizzbuzz_count % 5 == 0 {
            println!("buzz: {}", fizzbuzz_count);
        } else if fizzbuzz_count % 3 == 0 {
            println!("fizz: {}", fizzbuzz_count);
        } else {
            println!("{}", fizzbuzz_count);
        }
    }


}