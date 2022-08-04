Je vais écrire les exemples du livre dans le dossier exemple.

http://xion.io/post/code/rust-examples.html

1. create an `examples/` directory at the same level as the `src/` directory.
2. run examples with `cargo run --example scriptname`

...

By writing listing 1.2, TIL:
1. On peut écrire des chaines multilignes en rust 
2. On peut negate un saut de ligne avec un `\` qui le précède.

...

By writing listing 1.3, TIL:
1. `#![allow(dead_code)]` permet de ne pas afficher les warning de deads codes
2. J'ai créer un alias `cre="cargo run --example"` pour plus facilement run les examples

...

I'm encountering issues with nonbreaking space... those happens by typing `alt + <Space>` for example before opening a curly bracket pair.

I'm looking for a way to remove them automatically.

I've added `:autocmd BufWritePre *.rs :%s/ / /g` to my `.vimrc`

(and I've removed it currently because it screams an error when there's no `nbsp` in the file :/ )

...

There's something called a `prelude` that I need to investigate : https://doc.rust-lang.org/std/prelude/index.html

...

TODO: Page 46 includes a cheat-sheet about iterating over a collection, such as performance tips.

...

The `break` keyword can return a value.
```rust
let n = loop {
	break 123;
};

assert_eq!(n, 123);
```
__ Curly brackets & semi-colons __
- Question: when am I supposed to put semicolons after a curly bracket (if, loop, ..)?

...

In that exemple the comparison won't work because a `&integer` et `integer` ne sont pas comparable.
```rust
fn main() {
    let needle = 0o204;
    let haystack = [1, 2, 132, 12345];

    for item in &haystack {
        if item == needle {
            println!("{}", item);
        }
    }
}
```
Error Code : `rustc --explain E0277`

... 

Je prends ne note la page 45+ du livre que je trouve bien faite

- `for item in collection { ... } ` will end the lifetime  of collection after the block. (item is immutable), it's equivalent `for item in in IntoIterator::into_iter(collection)`
- `for item in &collection { ... } ` will borrow immutable references to each item in the collection without ending its lifetime, it's equivalent to `for item in collection.iter()`
- `for item in &mut collection { ... } ` will borrow mutable references to each item in the collection without ending the collection lifetime, it's equivalent to `for item in collection.iter_mut()`

also, anonymous loops 
`for _ in 0..10 { ... }`

we can break from a specific loop by labelling it 
```rust
'outer: for x in 1..10 {
    for y in 1..11 {
        for z in 5..=10 {
            println!("Hey");
            if x * y * z == 125 {
                break 'outer;
            }
        }
    }
}
```

(we can also use `continue` in that fashion).

...

Lifetime + type annotation syntax can get a bit busy

```rust
fn multiply_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i * *j
}
```

- parameter `i` is an `i32` with lifetime `a`
- Lifetimes are usually inferred in a process called _lifetime elision_

...

Generics

```rust
fn add<T>(i: T, j: T) -> T { 
    i + j 
}
```

This won't compile as is because type `T` doesn't implement `std::ops::Add`.

From `rustc --explain E0369`
> It is also possible to overload most operators for your own type by implementing traits from `std::ops`.

```rust
fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    i + j 
}
```
...

Notes on arrays :

- type `[T; n]` is an array, type `[T]` is a slice
- `[u8; 3]` is a different type than `[u8; 4]`
- in Rust, Array indexing is bounds checked and will panic when such check fail

... 

In `10_b_vectors.rs`, why did we need to add .iter() to the vector, doesn't it come with iterator trait support from the start? Do we need to use .iter() to transform it into something compatible with the iterator trait?

..

Use `cre 11_a_grep_lite -q -- --help` to access the help written by `clap@2` (like in `.js`).
The `--` signals to `cargo` to pass the arguments to the built binary.

...
