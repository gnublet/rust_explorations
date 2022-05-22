fn main() {
    // VARIABLES AND MUTABILITY
    let x = 5;
    println!("The value of x is: {}", x);

    // doesn't work since x is not mutable
    // x = 6;
    // println!("The value of x is: {}", x);

    // works since the let tells us that we're shadowing
    let x = 6;
    println!("The value of x is: {}", x);

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {}", x);
    }

    // inner shadowing ends and x returns to being 6
    println!("The value of x after inner scope is: {}", x);

    // Shadowing is different that marking a variable as mut since re-assiging without the let keyword gives an error
    // x = 5;

    // Another difference between shadowing and mut is 


    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // doesn't work. Cannot redefine a const
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3 + 1;
    

    // Works since shadowing allows us to create a new variable with the same name
    let spaces = "   ";
    let spaces = spaces.len();

    // However, this won't work since we can't change the type of a mutable variable
    // let mut spaces = "  ";
    // spaces = spaces.len();

    // LOOPs
    let mut count = 0;
    // you can assign a return value to a loop as well as naming a loop for a break to reference
    let outer_result = 'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        let inner_result = loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break remaining;
            }

            if count == 2 {
                break 'counting_up count;
            }

            remaining -= 1;
        };

        println!("Inner result: {}", inner_result);

        count += 1;
    };
    println!("Outer result = {}", outer_result);
    println!("End count = {}", count);

    let a = [1,2,3,4];

    for element in a {
        println!("the value element is {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}
