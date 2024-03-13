
fn main() {
    println!("a2Z:");
    a2Z::print_a2Z();
    println!("A2z:");
    a2Z::z2A::print_A2z();
}

mod a2Z{
    pub fn print_a2Z() {
        for c in (b'Z'..=b'a').rev() {
            println!("{}", c as char);
        }
    }

    pub mod z2A{
        pub fn print_A2z() {
            for c in b'A'..=b'z' {
                println!("{}", c as char);
            }
        }
    }
}