<h1 align="center">Glutin Starter</h1>

<p align="center">Glutin is a crate used to provide an OpenGL Context on as many platforms as possible. Glutin Starter makes it easy to get started with a basic glutin setup.<p>

## **Why**

This starter might not be necessary for most people but when I first tried using [glutin](https://docs.rs/glutin/0.24.0/glutin/) I encountered multiple problems so I decided to create a basic skeleton that could be used for any app using glutin.

This starter takes a basic glutin [example](https://github.com/rust-windowing/glutin/tree/master/glutin_examples) and isolates it to the basic parts it needs to work.

## **Usage**

To use this starter:

1. Clone the repository or download it.
2. Change to the glutin-starter repo or open it in your favorite code editor.
3. Modify the name, version, authors, description, and license as desired (make sure to leave the build, dependencies, and build-dependencies as they are).
4. Run `cargo run` or `cargo build` and then `cargo run`. The first time you run this it will take a while as it has to compile crates.
5. You'll know it works if when you run it you see a multi-colored triangle on a pink background. The console will also show mouse events as they happen real time. From here you can start creating your glutin-based masterpiece!

## **Note**

You might want to remove Cargo.lock from the .gitignore file, it's only added here so that you can fully build the project yourself.

## **License**

MIT