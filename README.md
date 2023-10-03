# Amlogic Boot Tool

This tool talks to Amlogic's mask ROM loader over USB.

## Accessing the loader

Many SBCs have a button to press on power-on to enter the loader mode. Some
products may come with a button as well or require entering certain commands
into the stock bootloader. We have found them to be an inconsistent mess. 🤷

## Compatibility

SoCs up to generation 3 (S905X, S905X2, S905{X,Y,D}3, etc) should be supported.
Those show as product string "GX-CHIP".

On some platforms, the commands may (partially) not work or behave different.

**NOTE: Since the protocols are not public, we had to find our ways.
Contributions and opening issues are welcome.**

For details, see the sections on [how we got there](#how-we-got-there) and the
[protocol versions](#protocol-versions).

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

Note the `--` to escape from Cargo.

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

### Previous work

See <https://github.com/superna9999/pyamlboot> for reference.

Corresponding talk by Neil Armstrong at Embedded Linux Conference Europe 2020:
<https://www.youtube.com/watch?v=u0-swEMDFp0>

> U-Boot: Porting and Maintaining a Bootloader for a Multimedia SoC Family

Slides:
<https://elinux.org/images/e/ef/ELC-E_2020_U-Boot_porting_and_maintaining_a_bootloader_for_a_multimedia_SoC_family.pdf>

## Protocol versions

So, there _is_ now a newer protocol, as people write.

[Sean Hoyt](https://twitter.com/Deadman_Android/status/1505570226540355592)
(Mar 20, 2022):

> As with all newer amlogic soc's it now uses "Amlogic DNL" for it's flashing
> instead of the older World cup update tool. The button on the board can take
> you to fastboot/recovery or USB burn mode. Currently it is not bootloader
> unlockable.

[Previous post in thread (with photos)](
https://twitter.com/Deadman_Android/status/1505570224531247105):

> T-Mobile TVision Hub 2nd gen board pics and general info. Specs: Amlogic
> S905Y4, 2gb of ram (Rayson RS512M32), 8gb emmc 5.1 (Samsung KLM8G1GETF-B041)

So the mask ROM's protocol has changed from version 4 on (needs verification!)?
