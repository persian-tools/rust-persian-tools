
# Contributing to Rust Persian Tools

First off, thanks for taking the time to contribute! ❤️

## Common rules

- You can [email me](mailto://alighahremani1377@gmail.com) before doing anything else.
  - <alighahremani1377@gmail.com>
- Never do a pull request on master/main/stable branches. (do it on 'dev' branch)
- Before any commit
  - make sure you are passing all tests by running: 'make test'
  - make sure the code is formatted correctly by running 'make fmt'
    - you can config your text editor/IDE to run 'cargo fmt' file save event
  - make sure there is no clippy warning (optional)
- If you need to do a breaking change let me know before doing anything. (we will make a new branch for it and add that to next major update X.x.x)

## Add a module

1. Add module-name to Cargo.toml at features section with list of dependencies
2. Add module-name to Cargo.toml at features under 'full' feature
3. Add module-name to Makefile to the end of build task
4. Add your module build test to end of file (checkout other build task)
5. Add 'pub mod module-name' to end of lib.rs
6. Add your module-name to the beginning of the lib.rs under '#[cfg(not(any())]'
7. Make a Pull request
8. Implement your feature
9. Add tests for your feature
10. Add standard function docs to your public functions

## Fix a bug

1. Make an [issue](https://github.com/persian-tools/rust-persian-tools/issues) on this repository and describe this bug (make sure its actually a bug and not a feature😄)
2. Write a test and make it fail
3. Make a pull request
4. Try to fix it and pass the test or tell me to do that

## Update database

Lets say there is a new bank or city or... out there and its not exist in this library

### If you have a Github account

Make an [issue](https://github.com/persian-tools/rust-persian-tools/issues) on this repository and explain what is missing or wrong.

### If you don't have a Github account

[Email me](mailto://alighahremani1377@gmail.com)
