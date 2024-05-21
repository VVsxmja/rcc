# `rcc`

A toy C compiler written in Rust.

## Usage

```
$ cargo run -- help

Usage: rcc <COMMAND>

Commands:
  preprocess        View the preprocessing result of a source file
  lex               View the lexical analysis result of a source file
  syntax            View the syntax analysis result of a source file
  semantic          View the semantic analysis result of a source file
  compile-binary    Generate binary from a source file
  compile-assembly  Generate assembly from a source file
  help              Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Try an example

Let's generate some fibonacci numbers!

```c
int fib(int n) {
    if (n == 1) return 1;
    if (n == 2) return 1;
    return fib(n - 1) + fib(n - 2);
}
```

We will use `rcc` to compile [`assets/fib.c`](assets/fib.c) (which contains a simple fibonacci function) 
to `out/fib.o` , and then use `clang` to compile [`assets/fib_main.c`](assets/fib_main.c) (which reads 
an integer `n`, then print the result of our `fib(n)` and a reference `fib_ref(n)` ) , 
then link `out/fib.o` , and finally produce `out/fib_main` .

```
mkdir -p out
cargo run -r -- compile-binary assets/fib.c out/fib.o
clang assets/fib_main.c out/fib.o -o out/fib_main
```

Try some `n` :

```
$ ./out/fib_main

Calculate fibonacci sequence to: 42
Reference Fib(42) = 267914296
rcc Fib(42) = 267914296
```
