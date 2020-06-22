# rust_guessing_game
Basic guessing game created to learn rust
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