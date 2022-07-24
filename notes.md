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
