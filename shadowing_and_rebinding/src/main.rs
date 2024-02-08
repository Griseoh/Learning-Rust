#[allow(unused_variables)]
#[warn(unused_assignments)]
fn main() {
    let mut x: i32 = 1;
    x = 7;
    let mut x = x;
    x = x + 3; 

    let y: i32 = 4;
    let y: &str = "Shadowed to text without a mutable type.";
    println!("{}", y);
    println!("Success.");
}
