
struct Color {
    red: u8,
    green: u8,
    blue: u8,        
}

struct Person {
    firstname: String,
    lastname: String,
}

impl Person {
    // Construct Person

    fn new(first: &str, last: &str) -> Person {
        Person {
            firstname: first.to_string(),
            lastname: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.firstname, self.lastname)
    }

    fn set_last_name(&mut self, last: &str) {
        self.lastname = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.firstname, self.lastname)
    }
}

pub fn run(){

    // Color
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0, 
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);


    // Person
    let mut p = Person::new("John", "Doe");
    println!("Person is {} {}", p.firstname, p.lastname);
    println!("Person's full name is {}", p.full_name());

    p.set_last_name("Dobby");
    println!("Person's full name is {}", p.full_name());

    println!("Person's tuple name is {:?}", p.to_tuple());

}