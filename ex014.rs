// http://c.learncodethehardway.org/book/ex16.html
#![feature(box_syntax)]

struct Person {
    name: &'static str,
    age: u8,
    height: u8,
    weight: u8
}

fn create_person(name: &'static str, age: u8, height: u8, weight: u8) -> Box<Person> {
    box Person {name: name, age: age, height: height, weight: weight}
}


impl Person {
    fn print(&self) {
        println!("Name: {}", self.name);
        println!("\tAge: {}", self.age);
        println!("\tHeight: {}", self.height);
        println!("\tWeight: {}", self.weight);
    }
}

fn main() {
    let mut joe = create_person("Joe Alex", 32, 64, 140);
    let mut frank = create_person("Frank Blank", 20, 72, 180);

    println!("Joe is at memory location {:p}", &*joe as *const _);
    joe.print();

    println!("Frank is at memory location {:p}", &*frank as *const _);
    frank.print();

    joe.age += 20;
    joe.height -= 2;
    joe.weight += 40;
    joe.print();

    frank.age += 20;
    frank.weight += 20;
    frank.print();
}