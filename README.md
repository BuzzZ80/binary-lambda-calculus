# Description
This is a Rust implementation of [Binary Lambda Calculus](https://tromp.github.io/cl/Binary_lambda_calculus.html), made for fun :)

Lambdas are represented with `00`.

Function composition is represented with `01`.

Variables are unary encoded [De Bruijn Indices](https://en.wikipedia.org/wiki/De_Bruijn_index), so:

```
1: 10
2: 110
3: 1110
```
et cetera...

# Usage

Input file should consist of ascii 0's and 1's. All other characters are ignored, and can be used for comments.

Use `cargo run [FILENAME]`
