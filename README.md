# Amlogic Boot Tool

See https://github.com/superna9999/pyamlboot for reference.

## Building

Have a Rust toolchain installed with Cargo.

```sh
cargo build --release
```

## Usage

```
./target/release/aml_boot
```

Or directly:

```
cargo run --release
```

This will print help on the CLI usage.

You can work on the code and directly run it to see if your changes work, e.g.:

```sh
cargo run --release -- -c info
```

Note the `--` to escape from Caro.

TODO: switch to a sub command style at some point.

## How we got there

This tool has been stated one evening at [Chaospott](https://chaospott.de), in
part to get familiar with the protocol and have our own clean slate code base.

We had a look at existing tools which were either lacking some features or just
errored for the boards we tried them with. Memory read/write are still erroring.
Examples are a TV box based on the S905X4 (different protocol?) and the Libre
Computer S905D3-CC.

The Khadas VIM1 and Libre Computer A311D-CC work fine, e.g., blinky demo:

```sh
aml_boot -c lc-a311d-cc-blink
```

Note that other authors have already done a lot and documented their findings.
Big kudos to Neil Armstrong and others who did all the hard work before us. :)
Look at [proto-rev.md](proto-rev.md) for notes on previous and our work.

## Procol versions

So, there _is_ now a newer protocol, as people write.

[Sean Hoyt](https://twitter.com/Deadman_Android/status/1505570226540355592)
writes (Mar 20, 2022):

> As with all newer amlogic soc's it now uses "Amlogic DNL" for it's flashing
> instead of the older World cup update tool. The button on the board can take
> you to fastboot/recovery or USB burn mode. Currently it is not bootloader
> unlockable.

[Previous post in thread (with photos)]
(https://twitter.com/Deadman_Android/status/1505570224531247105):

> T-Mobile TVision Hub 2nd gen board pics and general info. Specs: Amlogic
> S905Y4, 2gb of ram (Rayson RS512M32), 8gb emmc 5.1 (Samsung KLM8G1GETF-B041)

So the mask ROM's protocol has changed from version 4 on (needs verification!)?
