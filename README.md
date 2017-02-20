# pi-workshop-rs

This is the workshop starter project for the 'Rust on Raspberry Pi' workshop at the Raspberry Pi 5th Birthday Party.

Workshop material will be placed here shortly.

# Step 1 - Get the code

The code for this workshop lives on Github, in the repository [thejpster/pi-workshop-rs](https://github.com/thejpster/pi-workshop-rs). Open yourself a terminal window and execute these two commands.

*Note:* If the workshop organisers have prepared your workstation for you, you should find the folder `pi-workshop-rs` exists, in which case you can skip the `git checkout` and just perform the `cd`.

```
~ $ git checkout https://github.com/thejpster/pi-workshop-rs
~ $ cd pi-workshop-rs
```

# Step 2 - Check it builds

Unlike Python (but like C or C++), Rust code must be compiled from source into machine code in order to make it executable. The Rust ecosystem uses `cargo` as the build system - like `make` or `scons` in a C world, but more powerful and easier to use.

To build our code and execute our `crate` (the Rust term for a program or a library module) in one step, we execute `cargo run`.

```
~/pi-workshop-rs $ cargo run
    Updating git repository `https://github.com/thejpster/sensehat-rs`
   Compiling void v1.0.2
   Compiling cfg-if v0.1.0
   Compiling getopts v0.2.14
   Compiling bitflags v0.4.0
   Compiling byteorder v0.4.2
   Compiling byteorder v1.0.0
   Compiling libc v0.2.20
   Compiling semver v0.1.20
   Compiling measurements v0.2.1 (https://github.com/thejpster/rust-measurements#428d1426)
   Compiling pulldown-cmark v0.0.3
   Compiling bitflags v0.5.0
   Compiling rand v0.3.15
   Compiling rustc_version v0.1.7
   Compiling nix v0.6.0
   Compiling tempdir v0.3.5
   Compiling skeptic v0.5.0
   Compiling i2cdev v0.3.1
   Compiling sensehat v0.1.0 (https://github.com/thejpster/sensehat-rs#03e5ae84)
   Compiling pi-workshop-rs v0.1.0 (file:///home/pi/pi-workshop-rs)
    Finished debug [unoptimized + debuginfo] target(s) in 99.30 secs
     Running `target/debug/pi-workshop-rs`
Hello, world!
```

After a few minutes you will see that `cargo` has compiled our code, and any code that we depend upon (like the Sense Hat crate), and any code that that depends on (etc, etc) automatically. Next time you run this command, it won't compile any crates that haven't changed, so it will be quicker.

Note that if you are on an official workshop workstation and the organisers Foundation have pre-compiled the project for you (to save time), or the second time you run the command, you'll see this shorter output.

```
~/pi-workshop-rs $ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/pi-workshop-rs`
Hello, world!
```

# Step 3 - Open the code in an editor.

If you're on a Raspberry Pi, that means that sadly modern editors like Atom, Sublime Text, or VS Code are out of the question. You could go old-school and use Emacs or Vim, or perhaps try Geany (a GTK+ editor with good Rust support).

```
~/pi-workshop-rs $ geany src/main.rs &
```

We'll now be editing the code in the editor, then returning to the terminal to run our code.