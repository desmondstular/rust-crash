/*Notes:
- rust can always detect data types so not necessary to put data types if you want.
 */

#![deny(clippy::all)]

const MY_AGE: u8 = 28;

fn main() {
    // Called variable shadowing and is unique to rust.
    // Can declare a variable two times with different types
    let _name = "Doug";
    let _name = 10;

    // Shadowing blocks
    let data = "Foo";
    {
        let data = data.to_string();
    }

    // Constants: upper-cased and typed
}

fn _c1() {
    println!("Hello, world!");

    let name: &str = "Desmond";
    // name = John; Once you create a variable with let, it is immutable
    // let mut name = "Desmond"; I would need to use mut keyword to make it mutable
    // let name = 10; This would throw an error since you cannot change the type of a variable once declared
    let last_name: &str = "Stular";
    // let mut age: i32 = 28;

    // Can use unsigned int types
    let mut age: u8 = 22;
    println!("My age is not {}", age);

    age = 28u8; // Can include u8 after number to signal unsigned 8 bit integer and it works

    println!(
        "Hello, my name is {} {}! I am {} years old",
        name, last_name, age
    );

    // Can use underscores in numbers for readability
    let _population = 62_000_000;

    // Can also use hex codes
    let _red = 0xFA;

    // Can also use floating values
    let _distance_in_km: f32 = 5.32;
}
