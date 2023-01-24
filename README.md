# VI

[![Cargo Crate](https://img.shields.io/crates/v/vi.svg)](https://crates.io/crates/vi)
[![Docs](https://docs.rs/vi/badge.svg)](https://docs.rs/vi)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

> A input method library for Vietnamese input engine written completely in Rust

- [What is vi?](#what-is-this)
- [Installation](#installation)
- [Examples](#examples)
- [Support](#support)
- [Project status](#project-status)
- [Creator](#creator)

## What is this?

Since typing Vietnamese on Linux is pretty painful at the momment, a better input engine is always needed. To accommodate the future engines that will be built in Rust, this library existed to transform key inputs into the Vietnamese string output.

If you wish to find out how it works, I have written a short blog post (in Vietnamese) on how the library place a tone mark when it received the user input. Read it [here](https://zerox-dg.github.io/blog/2020/07/14/Bo-dau-trong-tieng-Viet/).

## Installation

Add `vi` to your dependencies in `Cargo.toml`.

```
[dependencies]
vi = "0.3.1"
```

## Examples

With vi, you can start building your own Vietnamese IME without worrying about how Vietnamese tone mark placement works. All you have to do is to implement a keyboard listener & a key sending system.

```rust
extern crate vi;

use vi::vni;

fn main() {
    let inputs = vec![
        vec!['v', 'i', 'e', 't', '6', '5'],
        vec!['n', 'a', 'm']
    ];

    let mut result = String::new();
    for input in inputs {
        vni::transform_buffer(input.iter().cloned(), &mut result);
        result.push(' ');
    }
    
    println!("{}", result); // prints "việt nam "
}
```

Please refer to the [`examples/`](examples) directory to learn more.

## Support

- [x] **VNI**
- [x] **Telex**

## Project status

Currently, this project is still at its early stage of development. There might be some minor bugs but overall, it should be 95% functional.

## Creator

- Viet Hung Nguyen (viethungax@gmail.com) ([Github](https://github.com/ZeroX-DG))

Want to support me? Consider buying me a coffee:)

[![ko-fi](https://www.ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/Z8Z81ODLC)
