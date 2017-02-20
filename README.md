# pi-workshop-rs

This is the workshop starter project for the 'Rust on Raspberry Pi' workshop at the Raspberry Pi 5th Birthday Party. Thanks to [Cambridge Consultants](http://www.cambridgeconsultants.com) for their support.

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

# Step 4 - Make a Sense Hat object

We'll be using the [Sense Hat](https://www.raspberrypi.org/products/sense-hat/) for this workshop - the same board that Tim Peake took to the International Space Station. It provides an 8x8 LED RGB display and some sensors we can read.

In `src/main.rs`, you'll see our main function. This is the function that gets called when our program is run. Change the body of the `main` function to look like this:

```rust
let hat = sensehat::SenseHat::new();
println!("Hello, world");
```

Make sure you indent your code properly, to keep it neat and tidy.

What this line does it create a new variable, called `hat`. The variable is initialised by the `new()` function associated with the `SenseHat` struct which lives in the `sensehat` crate imported at the top of the file.

If you run this code, you'll see the compiler complain that you've made a variable but never used it. Good point, Rust!

# Step 5 - Try and read the temperature

The `SenseHat` struct [has an API](http://example.com/URL_REQUIRED_HERE) (TODO ADD URL!) very similar to the [Python Sense Hat driver](https://pythonhosted.org/sense-hat/).

First up, we notice that the `new` function on the `SenseHat` struct returns a `SenseHatResult`. This is shorthand for `Result<SenseHat, SenseHatError>` and what this means is that the function could return one of two things. If it works OK, you get a `SenseHat`, which is an object we can use to do things. If it doesn't work OK (perhaps you don't have the I2C drivers loaded, or your on a PC not a Raspberry Pi), then you get a SenseHatError. Rust enforces you to check which you've got before allowed to call any methods on the returned object.

Let's test that theory, by modifying our `main.rs` file like this:

```rust
let mut hat = sensehat::SenseHat::new();
let temp = hat.get_temperature_from_humidity();
println!("Hello, world");
```

Oh-oh. The compiler is sad.

```
~/pi-workshop-rs $ cargo run
   Compiling pi-workshop-rs v0.1.0 (file:///home/jgp/work/pi_party/pi-workshop-rs)
error: no method named `get_temperature_from_humidity` found for type `std::result::Result<sensehat::SenseHat, sensehat::SenseHatError>` in the current scope
 --> src/main.rs:5:20
  |
5 |     let temp = hat.get_temperature_from_humidity();
  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

error: Could not compile `pi-workshop-rs`.

To learn more, run the command again with --verbose.
```

Rust correctly points out that we have a `Result` object, and you can't call `get_temperature_from_humidity()` on one of those.

We have a couple of options for checking whether our `new()` called was OK or not:

* `unwrap` - we can abort the program if we get `Err`
* `expect` - we can abort the program if we get `Err`, and report an error message.
* `if let` - we can execute some code if we get one specific alternative.
* `match` - like a switch statement in C, we can execute different things depending on whether the `Result` is `Ok` or `Err`

Calling `unwrap` is a little unhelpful, because it won't be immediately obvious why the program has aborted. The other options are a little verbose at this stage, so let's use `expect`. It's a function which takes a `Result` and either returns the type embedded in the `Ok` variant, or aborts with the given message.

Change the code to read:

```rust
let mut hat = sensehat::SenseHat::new().expect("Couldn't find Sense Hat");
let temp = hat.get_temperature_from_humidity();
println!("Hello, world");
```

Success! But, unfortunately we didn't actually do anything with our temperature.

```
~/pi-workshop-rs $ cargo run
   Compiling pi-workshop-rs v0.1.0 (file:///home/pi/pi-workshop-rs)
warning: unused variable: `temp`, #[warn(unused_variables)] on by default
 --> src/main.rs:5:9
  |
5 |     let temp = hat.get_temperature_from_humidity();
  |         ^^^^

    Finished debug [unoptimized + debuginfo] target(s) in 7.30 secs
     Running `target/debug/pi-workshop-rs`
Hello, world!
```

# Step 6 - Formatting our readings

In other languages, a function like `get_temperature_from_humidity()` might give you a number of some sort (maybe a floating point number). But, what would the units be? The comments might tell you, but what if you mis-read them? Issues with misunderstanding the units represented by plain numbers in computer programs [have serious consequences](http://edition.cnn.com/TECH/space/9909/30/mars.metric.02/).

In Rust, we have a system for richly expressing exactly what our values *mean*, but without adding any run time overhead. In this case, the `get_temperature_from_humidity()` function returns a `sensehat::Temperature` object (which is actually re-exported from a fork of the excellent [measurements crate](https://docs.rs/measurements/0.2.1/measurements/struct.Temperature.html)). This `Temperature` object has methods like `as_celcius()` and `as_fahrenheit()` which return floating point numbers in the specified units. It also has a default formatting implementation, which picks a unit and puts in the return string. 

Let's use that, but first, we have another Result to unwrap (because reading the temperature can fail - say, if you'd unplugged the SenseHat, or the sensor chip was damaged).

```rust
let mut hat = sensehat::SenseHat::new().expect("Couldn't find Sense Hat");
let temp = hat.get_temperature_from_humidity().expect("Couldn't read temp");
println!("Hello, world! It's {}", temp);
```

This gives us:

```
~/pi-workshop-rs $ cargo run
   Compiling pi-workshop-rs v0.1.0 (file:///home/pi/pi-workshop-rs)
    Finished debug [unoptimized + debuginfo] target(s) in 7.30 secs
     Running `target/debug/pi-workshop-rs`
Hello, world! It's 34.5 °C
```

Ooh, a unicode degree symbol. It's like we're in the future or something.

# Step 7 - Go crazy

Now it's time to explore the rest of the `SenseHat` API. Can you get the humidity? What does the default formatting for that look like? Can you print out the atomospheric pressure of the room in PSI?

Go have fun, with complete type safety behind you every step of the way.
