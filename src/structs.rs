// Traditional Struct
struct  Color {
    red: u8,
    green: u8,
    blue: u8
}

// tuple struct
struct Color2 (u8, u8, u8);

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

impl Person {
    fn say_hello() {
        println!("Hello")
    }
}

pub fn run() {
    let mut c = Color {
        red: 0,
        blue: 10,
        green: 255
    };

    c.red = 200;

    println!("Color: {} {} {}", c.green, c.red, c.blue);

    let mut c2 = Color2(10, 10, 10);

    c2.0 = 255;

    println!("Color2 {} {} {}", c2.0, c2.1, c2.2);

    let mut p = Person::new("John", "Doe");
    Person::say_hello();
    println!("{}", p.full_name());
    p.set_last_name("Nowak");
    println!("Person {} {}", p.first_name, p.last_name);
    println!("Person {:#?}", p);
    println!("{:?}", p.to_tuple());
    // println!("Person {:#?}", p);
}