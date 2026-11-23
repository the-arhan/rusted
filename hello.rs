#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str, 
    age: u8
}

fn main() {
    naming()
}

#[allow(dead_code)]
fn hello() {
    println!("Hello world!");
    println!("I'm a Rustacean!");
}

#[allow(dead_code)]
fn names() {
    let name = "Arhan";
    let surname = "Ali";
    println!("My surname is {1}, and my name is {0}, and my full name is {0} {1}", name, surname)
}

#[allow(dead_code)]
fn grammar() {
    println!("{subject} {verb} {object}",
            object = "the lazy dog.",
            subject = "The quick brown fox",
            verb = "jumps over"
            );
}

#[allow(dead_code)]
fn actors() {
    println!("{0:?} {1:?} is the {actor:?} name.",
            "Ian",
            "Somerhalder",
            actor="actor(s)");
}

#[allow(dead_code)]
fn stctr() {
    println!("Now {:?} will print!", Structure(3));
}

#[allow(dead_code)]
fn naming() {
    let name = "Peter";
    let age = 27;
    let peter = Person ( name, age );

    println!("{:#}", peter);
}