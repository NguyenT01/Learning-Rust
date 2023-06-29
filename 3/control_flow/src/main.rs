fn main(){
    condition1();
    condition2();
    multiple_ifs();
    if_in_a_let();
    loop1();
    loop2();
    while_loop();
    for_loop();
    for_range_loop();
}

fn condition1(){
    let number = 3;
    if number <5{
        println!("Condition was true");
    } else{
        println!("condition was false");
    }
}

fn condition2(){
    let number = 3;
    if number!=0{
        println!("number was something other than zero");
    }
}

fn multiple_ifs(){
    let number =6;

    if number%4 ==0{
        println!("number is divisible by 4");
    } else if number %3 ==0{
        println!("number is divisible by 3");
    } else if number%2 ==0{
        println!("number is divisible by 2");
    } else{
        println!("number is not divisible by 4,3 or 2");
    }
}

fn if_in_a_let(){
    let condition = true;
    // if condition is true, number is 5 else is 6.
    let number = if condition{5} else{6};

    println!("the value of number is: {number}");
}

fn loop1(){
    let mut counter =0;
    let result = loop{
        counter +=1;
        if counter ==10{
            break counter*2
        }
    };
    println!("result in loop1 is: {result}");
}

fn loop2(){
    let mut count =0;

    'counting_up: loop{
        println!("count= {count}");
        let mut remaining = 10;

        loop{
            println!("remaining = {remaining}");
            if remaining ==9{break;}
            if count==2 {break 'counting_up;}
            remaining -=1;
        }
        count +=1;
    }
    println!("End count = {count}\n");
}

fn while_loop(){
    let mut number =3;

    while number!=0{
        println!("{number}!");
        number -=1;
    }
    println!("LIFTOFF!!!\n");
}

fn for_loop(){
    let a = [10,20,30,40,50];

    for element in a{
        println!("the value is {element}");
    }
}

fn for_range_loop(){
    for number in (1..4).rev(){
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}