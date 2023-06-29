fn main() {
    fp();
    numeric_operation();
    boolean_type();
    char_type();
    tuple_type();
    array_type();
}

fn fp(){
    let x = 2.0; // f64
    let y: f32 = 3.14; // f32
    println!("{x} {y}");
}

fn numeric_operation(){
    let sum = 5+10;
    let diff = 95.5-4.3;
    let product = 4*30;

    let quotinent = 56.7/32.2;
    let truncated = -5/3;
    // let c2 = 5.25/2;

    let reminder = 43%5;

    println!("division results: {quotinent} ,{truncated}");
}

fn boolean_type(){
    let t = true;
    let f: bool = false;
    println!("{t} ,{f}");
}

fn char_type(){
    let c= 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("{c} ,{z}, {heart_eyed_cat}");
}

fn tuple_type(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("The value of y is: {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    println!("{five_hundred} ,{six_point_four}");
}

fn array_type(){
    let a= [1,2,3,4,5];
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
    
    let b: [i32;5] = [2,3,4,5,6];
    let c = [7;5]; // [7,7,7,7,7]

    let first = a[0];
    let second = a[1];
    println!("{first}, {second}");

}