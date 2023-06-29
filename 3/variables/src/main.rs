const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;

fn main(){
    let mut x=5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}\n");

    shadowing();
    shadowing2();
}

fn shadowing(){
    let y =5;
    let y = y+1;
    {
        let y= y*2;
        println!("The value of y in inner scope is: {y}");
    }
    println!("The value of y is: {y}\n");
}

fn shadowing2(){
    let spaces = " a";
    let spaces = spaces.len();
    println!("space is: {spaces}");
}