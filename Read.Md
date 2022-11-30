## Rust

# Rust is a modern systems programming language developed by the Mozilla Corporation. It is intended to be a language for highly concurrent and highly secure systems. It compiles to native code; hence, it is blazingly fast like C and C++.

# can build web applications and Mobile applications

# Problum with c and c++

# 1. Difficult to write secure code

# 2. Difficult to write mutli-thread code

# Rust solves all these problums and more

# Don,t have garbage collector by design That improves performance and memory usage

## Support for Web Assembly (WASM)

# Web Assembly helps to execute high computation intensive algorithms in the browser, on embedded devices, or anywhere else. It runs at the speed of native code. Rust can be compiled to Web Assembly for fast, reliable execution

# Automatic type casting is not allowed in Rust.

# By default, variables are immutable − read only in Rust if want to make it mutable then use mut keyword

## Data Types

# 1. Scalar Types

# 2. Compound Types

# Scalar Types

# 1. Integer

# 2. Floating-Point

# 3. Boolean

# 4. Character

# Integer Types

# 1. Signed Integer

# 2. Unsigned Integer

# Signed Integer

# 1. i8

# 2. i16

# 3. i32

# 4. i64

# 5. i128

# 6. isize

# Unsigned Integer

# 1. u8

# 2. u16

# 3. u32

# 4. u64

# 5. u128

# 6. usize

# Floating-Point Types

# 1. f32

# 2. f64

# Boolean Type

# 1. bool

# Character Type

# 1. char

# Compound Types

# 1. Tuple

# 2. Array

!@#$%^&\*()\_+{}|:"<>?[]\;',./`~

# Name convention for variable and function

# letter , number , underscore

# can not start with number and special character

# is a case sensitive

# let and const keyword

# let is used to create a variable and change it value later is shadowing

# const is used to create a constant and can not change it value later is not shadowing

# Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of the code need to know about.

# Major Operatios

# 1. Arithmetic

# 2. Comparison

# 3. Logical

# 4. Bitwise

# 5. Conditional

# NOTE − The ++ and -- operators are not supported in Rust.

# Tuple

# 1. A tuple is a compount Data Type is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

# Array

# 1. An array is a collection of elements of the same type. Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples.

# an array's length must be known at compile time.

# Ownership

# 1. Every value in Rust has a variable that's called its owner. There can only be one owner at a time. When the owner goes out of scope, the value will be dropped. The variable that owns a value is called its owner.

# stack and heap are used for memery allocation in rust

# stack is used for small memery allocation is know at compile time

# heap is used for large memery allocation also dynamic memery allocation is unknown at compile time memory is less orginized

## Transferring Ownership

# assign one varible to another variable

# Passing value to a function

# Returning value from a function

# Memory safety is achieved by tight control on who can use what and when restrictions.

# In case of primitive data types values are copied to the stack and the original value is not affected.

##  Borrowing

# 1. Borrowing is a way of allowing multiple parts of your code to access one piece of data without needing to copy that data into memory multiple times. pass value to function without taking ownership of it just by referience using &  can also be mutable by using &mut

# SLice 
# A slice is a pointer to a block of memory. Slices can be used to access portions of data stored in contiguous memory blocks.

# Struct
# A struct is a custom data type that lets you name and package together multiple related values that make up a meaningful group. If you’re familiar with an object-oriented language, a struct is like an object’s data attributes. struct keyword is used to declare a structure. Since structures are statically typed, every field in the structure must be associated with a data type. it is Immutable by default is  Object Oriented Programming

## Enum
# An enum is a custom data type that lets you define a type by enumerating its possible values. Enums are useful when you know all possible values a variable can have. Enums are also useful when you want to define a type that can be one of several different types. Enums are a feature of many programming languages, and they’re similar to structs. The major difference between an enum and a struct is how they’re defined and how they’re used. Enums are defined using the enum keyword, and their variants are separated with commas. Each variant can be any type, including a different enum. Enums are useful when you know all possible values a variable can have. Enums are also useful when you want to define a type that can be one of several different types. Enums are a feature of many programming languages, and they’re similar to structs. The major difference between an enum and a struct is how they’re defined and how they’re used. Enums are defined using the enum keyword, and their variants are separated with commas. Each variant can be any type, including a different enum.

# let can only be call in main function