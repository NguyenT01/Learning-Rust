fn main(){
    println!("Hello world!");
    another_function();
    function_with_param(25);
    print_labeled_measurement(5,'h');
    expression();
    fn_with_returned_value();
}

fn another_function(){
    println!("Another function!");
}

fn function_with_param(x: i32){
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

fn expression(){
    let y = {
        let x= 3;
        // the last line of expression does not have a semicolon
        x+1
    };
    println!("y is: {}",y);
}

fn fn_with_returned_value(){
    let x = five();

    println!("x is: {x}");

    let x2 = plus_one(x);
    println!("x plus one is: {x2}");
}

fn five() -> i32{5}

fn plus_one(x: i32) -> i32{
    // the return value does not have a semicolon
    x+1
}