String Internalization Mechanism in Rust
========================================

This is a string internalization mechanism in Rust, used by my virtual machine
project [Sky VM](https://github.com/kkshinkai/skyvm).

Features and Todos:

- [x] Support basic string interpolation.
- [x] Thread-safty.
  - [x] Use `std::sync::Mutex` to synchronize access to the internalization
        symbol table.
  - [ ] Write a lock-free implementation hash table alternative for better
        performance.
- [ ] Better test coverage (Especially for concurrency scenarios).

License
-------

The Unlicense