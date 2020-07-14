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

## Installation

Add `vi` to your dependencies in `Cargo.toml`.

```
[dependencies]
vi = "0.1.1"
```

## Examples

With vi, you can start building your own Vietnamese IME without worrying about how Vietnamese tone mark placement works. All you have to do is to implement a keyboard listener & a key sending system.

```rs
use vi::vni;

fn main() {
  let keys = get_keys(); // this is the keys that you received from the user

  let (_, output) = vni::transform_buffer(&keys);

  let keys_to_send = analyze_output(output); // analyze output to decide which keys to send
  send_keys(keys_to_send);  // sending keys to the active window
}
```

Please refer to the [simple example](examples/simple.rs) to learn more.

## Support

### OS

- [x] **Linux**
- [ ] **Windows**
- [ ] **MacOS**

### Typing method

- [x] **VNI**
- [ ] **Telex**

## Project status

Currently, this project is still at its early stage of development. There might be some minor bugs but overall, it should be 95% functional.

## Creator

- Viet Hung Nguyen (viethungax@gmail.com) ([Github](https://github.com/ZeroX-DG))

Want to support me? Consider buying me a coffee:)

[![ko-fi](https://www.ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/Z8Z81ODLC)
