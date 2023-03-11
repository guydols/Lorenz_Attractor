# Rust implementation and visualization of the Lorenz Attractor using Macroquad

This project is a Rust implementation and visualization of the Lorenz Attractor using the Macroquad game development framework. 

## What is the Lorenz Attractor?

The Lorenz Attractor is a mathematical model that describes the behavior of a simple system of three coupled nonlinear differential equations. It was first introduced by mathematician Edward Lorenz in 1963, and has since become one of the most famous and widely studied examples of a chaotic system.

Screenshot:
![](Screenshot.png?raw=true)

## How to use the program

To run the program, you must have Rust installed on your computer. You can download Rust from the official website at https://www.rust-lang.org/tools/install.

Once Rust is installed, you can clone this repository and run the program using the following command:

	cargo run

Or use --release for a more efficient binary

	cargo run --release

This will compile the Rust code and start the program, which will open a window displaying the Lorenz Attractor visualization.

## Dependencies

This project depends on the Macroquad game development framework, which is included as a dependency in the `Cargo.toml` file.

## License

This project is licensed under the GNU GPL - see the LICENSE file for details.
