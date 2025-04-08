<div align="center">

  <h1><code>cxx_bindgen</code></h1>

  <p>
    <strong>Automatically generate FFI bindings for CXX interop.</strong>
  </p>
</div>

## Installation

Install CXX and CXX build for the actual bindgen generation:

```sh
cargo add cxx
cargo add cxx-build
```

Then, add the two crates of this repository, namely `cxx_bindgen` and `cxx_bindgen-build`:

```sh
cargo add --git https://github.com/FabianHummel/cxx_bindgen.git
cargo add --git https://github.com/FabianHummel/cxx_bindgen-build.git
```

## Usage

First, add the `cxx-build` and `cxx_bindgen-build` configurations to `build.rs`.
It is important to add them in this order as `cxx_bindgen-build` procedurally generates the specified FFI-file during each build.
`cxx-build` then picks up on this file and does the heavy lifting:

```rust
// build.rs
fn main() {
  cxx_bindgen_build::bridge("src/ffi.rs")
    .namespace("example")
    .generate();

  cxx_build::bridge("src/ffi.rs")
    .std("c++20")
    .compile("example-bridge");
}
```

CXX bindgen is syntactically very similar to `wasm-bindgen`, such as renaming or skipping definitions for the target.
The list below shows some common use cases where CXX bindgen can be used to automatically generate definitions:

### Top level functions

```rust
// Prints the specified name
#[cxx_bindgen::cxx_bindgen(cxx_name = "printName")]
pub fn print_name(name: String) {
    println!("Hello {}!", name);
}
```

The code above will turn into the following FFI bindings. 
Note that attributes such as comments are kept, which will ultemately also end up in the C++ header:

```rust
#[cxx::bridge(namespace = "example")]
pub mod ffi {
    extern "Rust" {
        // #region "cxx-bridge-generated-rust"
        # [doc = " Prints the specified name"] fn print_name (name : String);
        // #endregion
    }
}
```

### Structs

```rust
#[cxx_bindgen::cxx_bindgen(shared)]
#[derive(Serialize, Deserialize)]
pub enum Waveform {
    Sine,
    Square,
    Sawtooth,
    Triangle
}
```

The code above will turn into the following FFI bindings.
It even keeps `Serialize` and `Deserialize` definitions, which is important later for fixing strange import behaivour later:

```rust
#[cxx::bridge(namespace = "example")]
pub mod ffi {
    // #region "cxx-bridge-generated-shared"
    # [derive (Serialize , Deserialize)] enum Waveform { Sine , Square , Sawtooth , Triangle , }
    // #endregion
}
```

### Struct Implementations

```rust
#[cxx_bindgen::cxx_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Person {
    pub name: String,
}
```

By default, all public functions within the `impl` block are being added to the FFI-file, but they can be skipped with `cxx_bindgen::cxx_bindgen(skip)`:

```rust
#[cxx_bindgen::cxx_bindgen]
impl Person {
    pub fn print_name(&self) {
        println!("My name is {}!", self.name);
    }

    #[cxx_bindgen::cxx_bindgen(skip)]
    pub fn secret_function(&self) {
        todo!("Super secret implementation");
    }
}
```

The code above will turn into the following FFI bindings.

```rust
#[cxx::bridge(namespace = "example")]
pub mod ffi {
    extern "Rust" {
        // #region "cxx-bridge-generated-rust"
        fn print_name (self : & Person);
        type Person;
        // #endregion
    }
}
```
