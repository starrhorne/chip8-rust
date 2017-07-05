![Screenshot](/screenshot.png?raw=true "The emulator running 'blinky'")

## Introduction

This is a chip-8 VM built in rust. If you're reading this, chances are that you're thinking of writing your own chip8 emulator. You should! It gives you a great feel for how home computers worked back in the late 70s. It's also a nice project for people new to Rust, because you don't need any of the language's more advanced features (like generics and traits). 

## Resources

These were the most useful chip-8 references I found. 

* [Mastering Chip-8](http://mattmik.com/files/chip8/mastering/chip8.html)
* [Cowgod's Chip-8 Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
* [How to write an emulator](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/) 

## Requirements

You need to have sdl2 installed with headers. On Ubuntu 17.04, this did the trick:

```
sudo apt-get install libsdl2-dev libsdl2-gfx-dev
```

## Usage

Clone this repository, then run:

```
cargo run /path/to/game
```

You can find public-domain games [here](https://www.zophar.net/pdroms/chip8/chip-8-games-pack.html). 

## Comments

Feel free to let me know if you have any questions or comments about this code.
At the time I'm writing this, I'm pretty new to Rust so it may not be 100% idiomatic. You can reach me on twitter @StarrHorne, or by opening an issue on this repo. 


## Credits

Most of the SDL-related code was taken from the sdl2 crate's documentation and examples. I also used Mike Zaby's [rust chip8 repository](https://github.com/mikezaby/chip-8.rs) as a reference, though I didn't use any of his code.  

