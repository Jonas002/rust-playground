

pub fn run(){
    let age: u8 = 18;
    let parents_check: bool = true;
    let knows_age: bool = true;

    //if else
    if age > 21 || knows_age {
        println!("Bartender: What would you like to drink?")
    } else if age < 21 && parents_check {
        println!("Bartender: Oh, you are the parent. What would you like to drink?")
    } else {
        println!("Bartender: Sorry, you have to leave!")
    }

    // shorthand if
    let is_mature: bool = if age >= 21 { true } else { false };
    println!("Maturity passed: {}", is_mature);


}