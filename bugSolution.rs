fn main() {
    let mut x = 5;
    {  
        let y = &mut x; 
        *y = 10; 
    }
    let z = &mut x; 
    *z = 15;
}

//Alternative Solution using a different approach:
fn main() {
    let mut x = 5;
    x = 10; //Directly modifying the value
    x = 15; //Directly modifying the value
}