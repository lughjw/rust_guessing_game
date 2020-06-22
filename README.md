# rust_guessing_game
Basic guessing game created to learn rust. This was my first time building anything in Rust and I folowed the instructions from the Rust docs itself located [here](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).
What follows is my observations and understandings that I had from building it.
## Concepts learned
### Variables
* Variable types are inferred unless specified ```let foo = 0;``` versus ```let foo: u32 = 0;```
* Variables are immutable by default. In order to make it mutible you must declare it as ```let mut foo = 0;``` versus ```let foo = 0;```
* You can get the address for a variable using ```&```.<br>
```Rust
let foo = 0;
// Both of these print the value of foo
println!("{}", foo);
println!("{}", &foo);

// Print the address of foo (p stands for pointer)
println!("{:p}", &foo);
```