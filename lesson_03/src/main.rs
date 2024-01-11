
// changed once a value is defined.
//
//
//
// Mutable
// Variables are immutable by default. Prefix the variable name with mut keyword to
// make it mutable. The value of a mutable variable can be changed.
//
//
//
// fn main() {
// let mut fees:i32 = 25_000;
// println!("fees is {} ",fees);
// fees = 35_000;
// println!("fees changed is {}",fees);
// }
//
//
//
// Immutable -- cannot change the value.
// Mutable – can change the value .
//
//
//
// ----------------
// Constants
// Constants represent values that cannot be changed. If you declare a constant then
// there is no way its value changes. The keyword for using constants is const.
// The naming convention for Constants are similar to that of variables. All characters in
// a constant name are usually in uppercase.
//
//
//
// Constant vs variable
// • Constants are declared using the const keyword while variables are declared
// using the let keyword.
// • A variable declaration can optionally have a data type whereas a constant
// declaration must specify the data type. This means const USER_LIMIT=100
// will result in an error.
// • A variable declared using the let keyword is by default immutable. However,
// you have an option to mutate it using the mut keyword. Constants are
// immutable.
// • Constants can be set only to a constant expression and not to the result of a
// function call or any other value that will be computed at runtime.
// • Constants can be declared in any scope, including the global scope, which
// makes them useful for values that many parts of the code need to know about
//
//
//
// const A = 5; -> error.
// const A:i32 = 5; -> correct.
// // Data type specification is must.
//
//
//
// // Even mut can't work. Because makes no sense.
//
//
//
// --------------
// Literals
// A literal is a notation for representing a fixed value in a variable.
// Numeric literals can be type annotated by adding the type as a suffix. As an example,
// to specify that the literal 42 should have the type i32, write 42i32.
// The type of unsuffixed numeric literals will depend on how they are used. If no
// constraint exists, the compiler will use i32 for integers, and f64 for floating-point
// numbers.
//
//
//
// fn main() {
// // Suffixed literals, their types are known at initialization
// let x = 1u8;
// let y = 2u32;
// let z = 3f32;
//
//
//
// // Unsuffixed literals, their types depend on how they are used
// let i = 1;
// let f = 1.0;
//
//
//
// Casting
// The conversion of one data type into another.
// Rules for converting between different types. In Rust we make use of as keyword when
// we want to convert from one type to another.
//
//
//
// In rust, it doesn't support implicit type casting.
// We can do explicit type casting.
//
//
//
// fn main() {
// let decimal = 65.43_f32;
// let integer = decimal as u8;
// }
// ====
// fn main(){
// // let one = 1;
// // let two:i8 = 10;
// // let three = 20_i8;
// // println!("{}",three);
// // let four = 12.2_f32;
// // println!("{}",four);
// let five = 1i32;
// println!("{} - byte",std::mem::size_of_val(&five));
// // let six = 2i16;
// // println!("{}",std::mem::size_of_val(&six));
// let seven = five as f32;
// println!("{} - byte",std::mem::size_of_val(&seven));
// println!("{}",five);
// println!("{}",seven);
// }
// -------------
//
fn main() {


    let n1 = 20_i8;


    let d1: f64 = 10.20;

    let n4: i32 = d1 as i32;

    println!("{}", n4);

    let n2 = 20;


}
