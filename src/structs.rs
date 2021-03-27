// Structs - Used to create custom data types

// Traditiona Struct
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

// Tuple Struct
struct Colore(u8, u8, u8);

// Person Struct
struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  // Construct
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: String::from(first),
      last_name: String::from(last),
    }
  }

  // Get full name
  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  // Set last name
  fn set_last_name(&mut self, last: &str) {
    self.last_name = String::from(last);
  }

  // Name to tuple
  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}

pub fn run() {
  let mut c = Color {
    red: 255,
    green: 0,
    blue: 0,
  };

  println!("Color: {} {} {}", c.red, c.green, c.blue);

  c.red = 200;

  println!("Color: {} {} {}", c.red, c.green, c.blue);

  let mut c = Colore(255, 0, 0);

  println!("Colore: {} {} {}", c.0, c.1, c.2);

  c.0 = 200;
  println!("Colore: {} {} {}", c.0, c.1, c.2);

  let p = Person::new("John", "Doe");
  println!("Person {}", p.full_name());
  println!("Person {} {}", p.first_name, p.last_name);

  let mut p2 = Person::new("Mary", "Doe");
  println!("Person {}", p2.full_name());
  p2.set_last_name("Williams");
  println!("Person {}", p2.full_name());
  println!("Person {:?}", p2.to_tuple());
}
