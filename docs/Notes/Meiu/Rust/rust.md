# Rust Pogramming Language

## Variables

Declaration

```rust
let mut x= 5;
```

Constants

```rust
const STRING:&str="Hello World";
```

Shadowing

```rust
let x = 5;
let x = x + 1;
```

Destructuring

```rust
struct Ponit2D(u32,u32);
let origin=Point2D(100,200);
let Point2D(x,y)=origin;
println("{:?} {:?}", x, y);

```

## Data Types

### Scalar Types

* Intergers
* characters
* boolean
* float

#### Integers

Signed integers have both `+` and `-`

Unsigned integers are always positive;

#### Characters

Represents letters, specified using the char keyword

Use single quotes;

### Compound Types

* Arrays

#### Arrays

* Fixed length
* Length known at compile time
* Heterogeneous - Can only contain items of the same data type

```rust
let array:[u32;4]=[1,2,3,4]

let onde=array[0];
```

Accessing

* Access items by index

#### Tuples

* Fixed length
* length known at compile time
* Homogeneous - Items can be of different types
  Empty tuple is known as `unit`

```rust
let tuple:(bool,i34,u8)=(true,2,4);
let onde=tuple.0;
```

## Functions

* Arguments types are always required
* Return type alwasy required if value returned
* if not return type is unit (an empty tuple)

```rust
fn exclaim(input:String)-> String{
  let mut output=input.to_uppercase();
  ouput.push('!')
  output
}
```

```rust
fn print_excited(input:String){
  let output= exclain(input);
  println!("{}",output);
}
```

## Structs

* A type composed of other types
* Can contain different types

Examples of structs

Access using the dot notation

#### Classic

* Ecah field has a name and type

```rust
struct Car {
  make: String,
  model: String,
  year: u32,
}
```

```rust
let car1= Car{
  make: String::from("Ford"),
  model: String::from("Mustange"),
  year: 1967,
}

println!(car1.year);
```

#### Tuple

* Similar to classic structs
* Fields have no names

```rust
struct Ponit2D(u32,u32);

let origin=Point2D(100,200);

println!("The value is {:?} and {:?}", origin.0, origin.1);

let Point2D(x,y)=origin;
println("{:?} {:?}", x, y);

```

#### Unit

* Have no fields
* Similar to the () unit type

## Enum

* List all variations of some data
* Referred to as algebraic data types

```rust
enum CardialPoints {
  North,
  South,
  East,
  West
}

enum CardialPoints {
  North(String),
  South(String),
  East(String),
  West(String),
}

fn main(){
  let north=CardialPoints::North;
  let west=CardialPoints::West(String::from("West"));
}

```

* An Enum variant can include any kind of data
* Can have a variety of types
* Similar to structs with more flexibility

#### Advantages

* Describe what kind of data to be strored
* Each variant can be of different kind
* All variants stored under the same enum type

## Expressions

### Match

* A pattern matching
* A `scrutinee` expression is provided to the patterns
* Arms are evaluated and compared with the scrutinee expression

```rust
fn main(){
  let x = 1;
  match x {
     1 => println!(),
 2 => println!("two"),
 3 => println!("three")
 _ => println!("something else"),
  }

}
```

* The scrutinee expression is `x`
* Each arm has a pattern and some code. the `=>` operator separates the pattern and the code to run
* The first arm with a matching pattern is chosen as the branch target of the match

```rust
fn main() {
    let x = 1;

    let y = match x {
        1 => 4,
        2 => x,
        _ => 5,
    };

    println!("The value is {}", y);
}
```

### if else

```rust
fn main(){
  if 1== 2 {
    println!("math is broken"); 
  } else {
    println!("ALl is good");
  }

}
```

## Loops

### loop

* Used to execute a code of block forever or until it's stopped or the program quits
* `break` can be used to stop a loop

```rust
fn main(){
  loop {
    println!("I loop forever")
  }
}
```

### while

* Conditional loops
* Run until a condition is met or become false

### for

* Iterate over elements in a collection
* Each pass of the loop exreacts values

```rust
fn main(){
  let a = [10,23,34];

  for element in a.iter() {
     println!("The value is {}",element);
  }
}
```

example 2

```rust

fn main() {
    let mut i = 1;
    let something = loop {
        i *= 2;
        if i > 100 {
            break i;
        }
    };
    println!("The value of something is {}", something);

    for item in 0..5 {
        println!("Value us {}", item);
    }
}
```

## Error Handling

### Panic

* The Simplest way to handle error
* Quits the program execution
* Call the `panic!` macro along with the message to print out
* Shoulb be used only if the program is in a unrecoverable state

Steps after a panic

1. Failure message is printed
2. Program unwinds and cleans up the stack
3. Program quits

### The Option enum

```rust
enum Option<T>{
  None,
  SOme(T)
}
```

* Manages the existence of nonexistent values
* Type `T` is generic and associated with Some variant
* `None` indicates no elemnt was found
* `Some` means that an element of type `T` was found

### The Result enum

```rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

* Used for recoverable errors that are more common
* The `Ok(T)` variant represents success and contains a value
* The `Err(E)` variant represnts an error
* Used when a failure is expred

Mostly used for the following

1. Parsing Strings
2. File access
3. Data Validation

Has the `unwrap` and `Expect` helper methods

`Unwrap` returns the value `T` of `Ok(T)` variant and a `panic`! for the `Err` variant

`Expect` returns a value just like `unwrap` or called the `panic!` macro with a detailed message

```rust
fn main(){
 File::open("hello.txt").unwrap();
 File::open("heloo1.txt").expect("Failed to open file");

}
```

### The `?` operator

* Similar to match statement

#### For result Type

* Unwraps the value if OK variant
* Returns an error for Err variant

#### For Option Type

* Returns a value for Some variant
* Returns nothing for the None variant

## Ownership

#### Rules of Ownership

* Each value in rust has a variable that is called it's `owner`.
* There can only be one owner at a time, to prevents

1. Dangling Pointers - pointers that do not point to a valid object of the appropriate type, for example if the memory is deallocated
2. Double Free - Trying to free memory that has already been freed
3. Memory Leaks - Not freeing memory that should have been freed

```rust
fn main(){


}
```

#### Rules of Ownership in functions

1. Passing a variable to a function transfers the ownershop to that function

## Borrowing

Options include cloning

```rust
fn main(){
  let say = String::from("Hello");
  print_out(say.clone());
  println!(say)
}

fn print_out(out:String){
  println!(out);
}

```

We borrow a variable by adding an `&` to it

```rust
fn main(){
  let say = String::from("Hello");
  print_out(&say);
  println!(say)
}

fn print_out(out: &String){
  println!(out);
}
```

#### Mutable Borrowing

```rust
fn main(){
  let mut myvec= vec![1,2,3];
  println!("{:?}", myvec) //1,2,3
  add_to_vec(&myvec);
  println!("{:?}",myvec) //1,2,3,4
}

fn add_to_vec(a_vec: &mut Vec<i32>){
  a_vec.push(4);
}
```

## Read More on

* Enums
* &String vs &str
*
