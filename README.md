# VI

> A back-end for Vietnamese input engine written completely in Rust

## What is this?

Since typing Vietnamese on Linux is pretty painful at the momment, a better input engine is always needed. To accommodate the future engines that will be built in Rust, this back-end existed to transform key inputs into the Vietnamese string output.

## How to use?

There're 2 ways to use this back-end. Either you take care of your own engine & buffer management or you can use the engine in this back-end. Refer to [simple example](examples/simple.rs) if you plan on building your own engine or refer to [engine example](examples/engine.rs) if you wish to utilize the engine provided by this back-end.

The method that this engine use it backspace, which means that to modify a word, it send a bunch of backspaces before inserting the new content for the word. E.g.

```
viet5 -> viet -> vie -> việ -> việt
```

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
