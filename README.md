https://doc.rust-lang.org/book/second-edition/ch02-00-guessing-game-tutorial.html

Install
---
``` sh
curl https://sh.rustup.rs -sSf | sh
```

``` sh
# build
$ cargo build

# build (as release)
$ cargo build --release

# run
$ cargo run

# check
$ cargo check
```

CheatSeet
---
``` rust
// This is a comment

//
// This is multi comment
//

// immutable valirable
let x = 4;

// mutable variable
let mut y = 0;

// constant
const MAX_SIZE: u32 = 100_000;

// shadowing
let age = "14"
let age: i32 = age.parse().unwrap();

// if Expressions
let number = 3;
if number % 3 == 0 {
  println!("Fizz");
} else if number % 5 == 0 {
  println!("Buzz");
} else {
  println!("{}", number);
}

// Using if in a let Statement
let condition = true;
let number = if condition {
  5
} else {
  6
}

// Repeating Code with loop
loop {
  println!("again!");
  break;
}

// Conditional Loops with while
let mut number = 3;
while number != 0 {
  number = number - 1;
}

// Looping Through a Collection with for
let a = [10, 20, 30];
for element in a.iter() {
  println!("the value is: {}", element);
}

for number in (1..4).rev() {
  println!("{}", number);
}

```

Data type
---
i8/u8
i16/u16
i32(default)/u32
i64/u64
f32/f64(default)
bool
char
_tuple_
_array_

