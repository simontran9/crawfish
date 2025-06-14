<div align="center">
  <img width="200px" src="docs/crawfish.png">
  <h1>crawfish</h1>
  <p>simple programming language for the layman</p>
</div>

## Installation (building from source)

### Requirements

- [Rust Compiler](https://gcc.gnu.org/)
- [GNU Make](https://www.gnu.org/software/make/)

### Steps

1. Git clone the repository

```sh
git clone https://github.com/simontran7/crawfish.git
```

2. `cd` into the `crawfish/` directory, then build the project with zig's build system

```sh
cd crawfish/
make
```

4. Move the `build/crawfish` compiler binary to a desired location (e.g. in `/Users/<your name>`), then add it to your `PATH` by adding the following line to your `.bashrc` file

```sh
# in your .bashrc
export PATH=$PATH:<path to the crawfish compiler executable>
```

## Usage

- Language tour: `docs/tour-of-crawfish.md`
- Compiler design: `docs/compiler-design.md`

## Credits

Many thanks to the creators of the resources that I used. There are far too many resources that I used to list (since I scour through a tons of resources to find the "conventional" way of structuring my compiler, even down to the struct names...), but some notable ones include:

