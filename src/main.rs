use std::io;

fn main() {
    println!("Welcome!");
    selector();
}

fn selector() {
    println!(
        "Pick Program. 1 for fahrenheit_celcius, 2 for nth_fibonacci, 3 for the_twelve_days_of_xmus."
    );

    loop {
        println!("Choose 1, 2 or 3 and then press enter!");
        let mut select = String::new();

        io::stdin().read_line(&mut select).expect("Failed to read input.");

        let select: u32 = match select.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        match select {
            1 => fahrenheit_celcius(),
            2 => nth_fibonacci(),
            3 => the_twelve_days_of_xmus(),
            _ => {
                continue;
            }
        }
        println!("Program at its end.");
    }
}

fn fahrenheit_celcius() {
    // Convert temperatures between Fahrenheit and Celsius.
    println!("Fahrenheit -> Celsius write 'fc', Celsius -> Fahrenheit write 'cf'.");

    //definiera
    let mut select = String::new();
    //input
    io::stdin().read_line(&mut select).expect("failed");
    //trim,parse etc
    let select = select.trim();
    //
    if select == "fc" || select == "FC" {
        println!("Fahrenheit -> Celsius");
        println!("Write a tempreture in Fahrenheit");
        //definera
        let mut select = String::new();
        //input
        io::stdin().read_line(&mut select).expect("Failed to read input.");
        //parse
        let select: i32 = match select.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                return;
            }
        };
        let result: i32 = ((select - 32) * 5) / 9;

        println!("Result: {}", result);
    } else if select == "cf" || select == "CF" {
        println!("Celsius -> Fahrenheit");
        println!("Write a tempreture in Celsius");
        //definera
        let mut select = String::new();
        //input
        io::stdin().read_line(&mut select).expect("Failed to read input.");
        //parse
        let select: i32 = match select.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                return;
            }
        };
        let result: i32 = (select * 9) / 5 + 32;
        println!("Result: {}", result);
    } else {
        println!("error in input");
    }
}

fn nth_fibonacci() {
    // Generate the nth Fibonacci number.
    println!("Write a number of which Fibonacci number you like to get. 45 is the limit!");
    loop {
        let mut nth = String::new();
        io::stdin().read_line(&mut nth).expect("failed");
        let nth: u32 = match nth.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        let mut num1 = 0;
        let mut num2 = 1;
        let mut num3 = 0;
        let mut count = 0;
        loop {
            if nth == count {
                println!("{nth}th number of Fibonacci is {num1}");
                break;
            }
            num3 = num1 + num2;
            num1 = num2;
            num2 = num3;
            count += 1;
        }
        break;
    }
}

fn the_twelve_days_of_xmus() {
    // Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

    let day_of_send = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];
    let true_love_send = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five gold rings (five golden rings)",
        "Four calling birds",
        "Three French hens",
        "Two turtledoves",
        "And a partridge in a pear tree",
    ];

    let mut day = 0;

    loop {
        println!(" ");
        println!("On the {} day of Christmas, my true love sent to me", day_of_send[day]);
        let mut count = 0;
        loop {
            println!("{}", true_love_send[11 - day + count]);

            if count == day {
                break;
            }
            count += 1;
        }

        day += 1;
        if day == 12 {
            break;
        }
    }

    //svans
    println!(" ");
    println!("{}", true_love_send[11]);
}