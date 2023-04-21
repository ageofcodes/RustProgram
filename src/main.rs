use std::io;

fn main() {
    println!("Welcome!");
    //
    selector();
    
}

fn selector() {
    println!("Pick Program. Press 1 for fahrenheit_celcius, Press 2 for nth_fibonacci, Press 3 for the_twelve_days_of_xmus.");
    
    loop {
        println!("Choose 1, 2 or 3 and press enter!");
        let mut select = String::new();

        io::stdin()
            .read_line(&mut select)
            .expect("Failed to read input.");

        let select: u32 = match select.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; 

        match select {
            1 => fahrenheit_celcius(),
            2 => nth_fibonacci(),
            3 => the_twelve_days_of_xmus(),
            _ => continue,
        }

    }

}

fn fahrenheit_celcius() {
// Convert temperatures between Fahrenheit and Celsius.
println!("Hello, Fahrenheit!");
} 

fn nth_fibonacci() {
// Generate the nth Fibonacci number.
println!("Hello, Fibonacci!");
} 

fn the_twelve_days_of_xmus() {
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song. 
println!("Hello, Carol!");
} 