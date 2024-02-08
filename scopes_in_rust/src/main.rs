fn main() {
    let x: i32 = 10;
    {
        let y:i32 = 5;
        println!("The value of x is {} and the value of y is {}", x,y);
    }
    //println!("The value of x is {} and the value of y is {}", x,y); 
    //'---> This causes error due to non existence of a variable y in this scope.
}
