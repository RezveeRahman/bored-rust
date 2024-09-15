/**
 * @author: Rezvee Rahman
 * @date: 09/14/2024
 *
 * @description: Temperature converter application.
 */

use std::io;

/**
 * Main entry point of the program.
 */
fn main() {
    let temp : f32;
    let unit : String;

    println!("Temperature convertor application: Converts Celsius, Farenheit, and Kelvin");

    unit = temp_unit();
    temp = get_value();
    println!("{}\u{00B0}{}", temp, unit);

    if unit == "Kelvin" {
        println!("{}\u{00B0}{}", convert_to_celsius(&unit, temp), "Celsius");
        println!("{}\u{00B0}{}", convert_to_farenheit(&unit, temp), "Farenheit");
    } else if unit == "Farenheit" {
        println!("{}\u{00B0}{}", convert_to_celsius(&unit, temp), "Celsius");
        println!("{}\u{00B0}{}", convert_to_kelvin(&unit, temp), "Kelvin");
    } else if unit == "Celsius" {
        println!("{}\u{00B0}{}", convert_to_farenheit(&unit, temp), "Farenheit");
        println!("{}\u{00B0}{}", convert_to_kelvin(&unit, temp), "Kelvin");
    }
}

fn temp_unit() -> String {
    const UPPER_LIMIT : u8 = 3;
    let mut opt : String;
    let mut opn : u8;

    loop {
        opt = String::new();

        io::stdin()
                .read_line(&mut opt)
                .expect("Random erroR?");

        opn = match opt.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if  opn > UPPER_LIMIT {
                println!("Enter a number between [1 - 3]");
                continue;
        }

        break;
    }

    match opn {
        1 => "Farenheit".to_owned(),
        2 => "Celsius".to_owned(),
        3 => "Kelvin".to_owned(),
        _ => "Null".to_owned()
    }
}

fn get_value() -> f32 {
    let mut user_input : String;

    loop {
        user_input = String::new();
        println!("Enter temperature");
        io::stdin()
                .read_line(&mut user_input)
                .expect("Could not handle");
        if user_input == "" {
            println!("Please enter a value!");
            continue;
        } else {
            break;
        }
    }

    user_input.trim().parse::<f32>().unwrap()
}

fn convert_to_farenheit(nt : &String, value : f32) -> f32 {
    let conversion : f32;
    if nt == "Celsius" {
        conversion = (value * 9.0/5.0) + 32.0;
    } else if nt == "Kelvin" {
        conversion = (value - 273.15) * (9.0/5.0);
    } else {
        conversion = 0.0;
    }

    conversion
}

fn convert_to_celsius(nt : &String, value : f32) -> f32 {
    let conversion : f32;
    if nt == "Farenheit" {
        conversion = (value - 32.0) * (5.0/9.0);
    } else if nt == "Kelvin" {
        conversion = value + 273.15;
    } else {
        conversion = 0.0;
    }

    conversion
}

fn convert_to_kelvin(nt : &String, value : f32) -> f32 {
    let conversion : f32;
    if nt == "Celsius" {
        conversion = (value - 32.0) * (5.0/9.0);
    } else if nt == "Farenheit" {
        conversion = (value - 32.0) * (5.0/9.0) + 273.15;
    } else {
        conversion = 0.0;
    }

    conversion
}