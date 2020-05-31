# Languages benchmarks

Gathering of code execution tests for such languages (their versions and compilers):
- C
- Java
- Rust
- Go
- NodeJS
- Dart
- Python
- PHP

### Current benchmarks code

Main idea in benchmarking is to put runtime calculation inside the code not mesure code execution with tools such as `time` which for some cases will include launching VM time, compilation, et.c. and not actual code execution time.

| Name | Description | Code base |
| --- | --- | --- |
| Prime numbers | Count amount of prime numbers there are up to 10m number | https://github.com/soulman-is-good/benchmarks/tree/master/prime |

### How to run

Different languages require different approach to run (compile/interpret).

#### C language

C has different compilers. I run on MacBook with `clang` and brew `gcc` compilers with no extra flags

#### Java

Java is compiled with `javac` compiler. No extra options

#### Rust

Rust is also compiled w/o extra options

#### Go

Go same as Rust and Java

#### NodeJS

Just run node {file.js}

#### Dart

Dart can be run in realtime and can be compiled AOT or binart. I use binary compilation with `dart2native` official tool

#### Python

I test both versions of python - 2.7 and 3<br>
`python {file.py}`

#### PHP

Run `php {file.php}`
