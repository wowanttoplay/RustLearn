fn main() {
    // if , else , else if
    // "If number {}" is wrong, you must directly translate the value to bool
    let number = 3;
    if number < 4 {
        println!("condition was true");
    } else if number == 4 {
        println!("numer equals 4");
    } else {
        println!("condition was false");
    }

    // If is an expression , it can return value to other variable
    let a = if number == 4 { 5 } else { 10 };
    println!("a is {}", a);

    // use loop, while, for to loop execution
    // loop {
    //     println!("again!");
    // }
    // Loop label can e used to break special loop
    break_loop_label();
    // break is an expression, it can return value
    break_return_value();
    // use of while
    use_of_while();
    // use while to print array
    while_print_array();
    // for to print array
    for_print_array();
    // for to control range to loop
    for_print_range();
}

fn break_loop_label() {
    let mut counter = 0;
    'counting_up: loop {
        println!("count is {}", counter);
        let mut remaining = 10;

        loop {
            println!("remaining {}", remaining);
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        counter += 1;
    }
    println!("End count = {}", counter);
}

fn break_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result is {}", result);
}

fn use_of_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn while_print_array() {
    let a = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < a.len() {
        println!("the value is {}", a[index]);
        index += 1;
    }

    println!("LIFTOFF!!!");
}

fn for_print_array() {
    let a = [1, 2, 3, 4, 5];

    for element in a {
        println!("the value is {}", element);
    }

    println!("LIFTOFF!!!");
}

fn for_print_range() {
    for number in (1..6).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!!!");
}
