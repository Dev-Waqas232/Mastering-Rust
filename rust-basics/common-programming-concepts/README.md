# Common Programming Concepts

## Variables

- Variables in rust are by default immutable

- We can make it mutable by using mut keyword after let keyword

- We can't reassign the value of different type to mutable variable

- We can also define constants using const keyword ( we only use them for hardcoded values )

- We also need to define the type of the const variable

- Shadowing means that we can redefine a variable with the same name and the variable defined later will be used and the pre one will be shadowed by the compiler

## Data types

- There are 4 Scalar data types, integers, floating-point numbers, boolean and characters

- There are two compund data types, tuple and array

- Tuple have fixed length, once defined, they can't grow or shrink

- Every element in tuple can have his own data type

- Array have fixed length in rust, and also has same data types for all elements across it

## Functions

- Rust uses snake case convention for naming of functions and variables

- Function declarations must contain the type of each parameter

- If a line ends without semicolon, it is an expression and got automatically returned at the end of the block
