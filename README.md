# Learn Rust The Hard Way

This is an implementation of Zed Shaw's Learn X The Hard Way for the Rust
Programming Language.

## Installing Rust

Till official release it out, it is recommended to use the nightly builds. This is primarily because of stability issues blocking some features in releases. *Racer* provides code completion for [Rust](http://www.rust-lang.org/) and it depends on some unstable features in standard library. If you don't want racer, you can install the release (alpha/beta)

- Install Rust Compiler
An easy way to install the nightly binaries for Linux and Mac is to run this in your shell:
  $ curl -s https://static.rust-lang.org/rustup.sh | sudo sh -s -- --channel=nightly

On Windows you can use the MSI installer from downloads page

- Install Rust Source
Extract the Rust Source code in folder of your choice, (preferrably inside the Rust Installation Directory)
Set ```RUST_SRC_PATH``` environment variable to point to the 'src' dir in your rust source installation

- Install Racer 
Follow instructions to install racer for editor completion setup
[racer](https://github.com/sarvex/racer)

## Compiling your first program

TODO: Instructions on compiling

## How to read the guide

TODO: Tips on how to read the guide.

Each chapter is based on a corresponding chapter in Learn C The Hard Way. The
order of the chapters however may vary.

## Acknowledgements

Shout out to Zed Shaw for the providing the template.
