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
// immutable valirable
let x = 4;

// mutable variable
let mut y = 0;

// constant
const MAX_SIZE: u32 = 100_000;

// shadowing
let age = "14"
let age: i32 = age.parse().unwrap();
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
