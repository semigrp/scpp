# SafeCpp: A Rust-based C++ Memory Safety Subset Compiler

SafeCpp is a Rust-based compiler that focuses on improving memory safety in C++ code by targeting specific error-prone features and operations. By analyzing these subsets, SafeCpp provides warnings and error messages to help developers identify and resolve memory issues in their C++ codebase.

## Key Features

SafeCpp targets the following memory error-prone subsets in C++:

### Pointer Operations

- Detect and prevent null pointer dereferences
- Identify and warn about uninitialized pointer usage
- Detect and prevent dereferences of freed pointers (dangling pointers)

### Dynamic Memory Allocation and Deallocation

- Detect memory leaks caused by allocating memory without proper deallocation
- Prevent double-free errors by tracking memory deallocations

### Array Operations

- Identify and prevent out-of-bound array accesses
- Detect and warn about confusion between array names and pointers

## Getting Started

- To get started with SafeCpp, follow these steps:

Clone the repository

```bash
git clone https://github.com/yourusername/SafeCpp.git
```

Build the compiler

```bash
cd SafeCpp
cargo build --release
```

Use SafeCpp to analyze your C++ code

```bash
./target/release/safecpp your_cpp_file.cpp
```

## Contributing

We welcome contributions to SafeCpp! If you would like to help improve memory safety in C++ code, please submit a pull request, open an issue, or contact the maintainers.

## License

SafeCpp is licensed under the MIT License.
