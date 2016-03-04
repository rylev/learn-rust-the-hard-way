# Learn Rust The Hard Way

This is an implementation of Zed Shaw's Learn X The Hard Way for the Rust
Programming Language.

## Installing Rust

### On Mac and Linux
If you are on a Mac or Linux machine only open a terminal and type
```
$ curl -sSf https://static.rust-lang.org/rustup.sh | sh
```
This will download a script and proceed with the installation. If everything goes right, you should see
```
Welcome to Rust.

This script will download the Rust compiler and its package manager, Cargo, and
install them to /usr/local. You may install elsewhere by running this script
with the --prefix=<path> option.

The installer will run under ‘sudo’ and may ask you for your password. If you do
not want the script to run ‘sudo’ then pass it the --disable-sudo flag.

You may uninstall later by running /usr/local/lib/rustlib/uninstall.sh,
or by running this script again with the --uninstall flag.

Continue? (y/N)
```

From here just usual yes, yes, yes... and follow the prompts :smile:

### On Windows
Please install the appropriate installer from [Here](https://www.rust-lang.org/downloads.html)

### Feeling fancy __wanna build it__

***Warning: Mad Code Ahead!***

1. Make sure you have installed the dependencies:

   * `g++` 4.7 or `clang++` 3.x
   * `python` 2.7 (but not 3.x)
   * GNU `make` 3.81 or later
   * `curl`
   * `git`

2. Clone the [source] with `git`:

   ```
   sh
   $ git clone https://github.com/rust-lang/rust.git
   $ cd rust
   ```

[source]: https://github.com/rust-lang/rust

3. Build and install:

    ```
    sh
    $ ./configure
    $ make && make install
    ```

    > ***Note:*** You may need to use `sudo make install` if you do not
    > normally have permission to modify the destination directory. The
    > install locations can be adjusted by passing a `--prefix` argument
    > to `configure`. Various other options are also supported – pass
    > `--help` for more information on them.

    When complete, `make install` will place several programs into
    `/usr/local/bin`: `rustc`, the Rust compiler, and `rustdoc`, the
    API-documentation tool. This install does not include [Cargo],
    Rust's package manager, which you may also want to build.

[Cargo]: https://github.com/rust-lang/cargo

#### Building on Windows

There are two prominent ABIs in use on Windows: the native (MSVC) ABI used by
Visual Studio, and the GNU ABI used by the GCC toolchain. Which version of Rust
you need depends largely on what C/C++ libraries you want to interoperate with:
for interop with software produced by Visual Studio use the MSVC build of Rust;
for interop with GNU software built using the MinGW/MSYS2 toolchain use the GNU
build.


##### MinGW

[MSYS2](http://msys2.github.io/) can be used to easily build Rust on Windows:

1. Grab the latest MSYS2 installer and go through the installer.

2. From the MSYS2 terminal, install the `mingw64` toolchain and other required
   tools.

   ```
   sh
   # Update package mirrors (may be needed if you have a fresh install of MSYS2)
   $ pacman -Sy pacman-mirrors
   ```

Download [MinGW from
here](http://mingw-w64.org/doku.php/download/mingw-builds), and choose the
`threads=win32,exceptions=dwarf/seh` flavor when installing. After installing,
add its `bin` directory to your `PATH`. This is due to [#28260](https://github.com/rust-lang/rust/issues/28260), in the future,
installing from pacman should be just fine.

   ```
   # Make git available in MSYS2 (if not already available on path)
   $ pacman -S git

   $ pacman -S base-devel
   ```

3. Run `mingw32_shell.bat` or `mingw64_shell.bat` from wherever you installed
   MSYS2 (i.e. `C:\msys`), depending on whether you want 32-bit or 64-bit Rust.

4. Navigate to Rust's source code, configure and build it:

   ```
   sh
   $ ./configure
   $ make && make install
   ```

##### MSVC

MSVC builds of Rust additionally require an installation of Visual Studio 2013
(or later) so `rustc` can use its linker. Make sure to check the “C++ tools”
option. In addition, `cmake` needs to be installed to build LLVM.

With these dependencies installed, the build takes two steps:

```
sh
$ ./configure
$ make && make install
```
## Compiling your first program

TODO: Instructions on compiling

## How to read the guide

TODO: Tips on how to read the guide.

Each chapter is based on a corresponding chapter in Learn C The Hard Way. The
order of the chapters however may vary.

## Acknowledgements

Shout out to Zed Shaw for the providing the template.
