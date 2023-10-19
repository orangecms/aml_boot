use crate::protocol::{read_reg, write_reg, Handle};
use std::{thread::sleep, time::Duration};

// From S905 Public Datasheet V1.1.4
const S905_PERIPHS_MUX_BASE: u32 = 0xC883_4400;
/*
const S905_PERIPHS_PIN_MUX_0: u32 = S905_PERIPHS_MUX_BASE + 0x00b0;
const S905_PERIPHS_PIN_MUX_1: u32 = S905_PERIPHS_MUX_BASE + 0x00b4;
const S905_PERIPHS_PIN_MUX_2: u32 = S905_PERIPHS_MUX_BASE + 0x00b8;
const S905_PERIPHS_PIN_MUX_3: u32 = S905_PERIPHS_MUX_BASE + 0x00bc;
const S905_PERIPHS_PIN_MUX_4: u32 = S905_PERIPHS_MUX_BASE + 0x00c0;
const S905_PERIPHS_PIN_MUX_5: u32 = S905_PERIPHS_MUX_BASE + 0x00c4;
const S905_PERIPHS_PIN_MUX_6: u32 = S905_PERIPHS_MUX_BASE + 0x00c8;
*/
const S905_GPIOZ_OE: u32 = S905_PERIPHS_MUX_BASE + 0x0054;
const S905_GPIOZ_OUT: u32 = S905_PERIPHS_MUX_BASE + 0x0058;
/*
const S905_GPIOX_OE: u32 = S905_PERIPHS_MUX_BASE + 0x0060;
const S905_GPIOX_OUT: u32 = S905_PERIPHS_MUX_BASE + 0x0064;
*/

const REG4: u32 = S905_PERIPHS_MUX_BASE + 0x00bc;

const S905_AO_RTI_PIN_MUX_BASE: u32 = 0xC810_0000;
/*
const S905_AO_RTI_PIN_MUX_1: usize = S905_AO_RTI_PIN_MUX_BASE + 0x0014;
const S905_AO_RTI_PIN_MUX_2: usize = S905_AO_RTI_PIN_MUX_BASE + 0x0018;
*/
const S905_PULL_UP_REG3: u32 = S905_AO_RTI_PIN_MUX_BASE + 0x00F4;
const S905_PULL_UP_EN_REG3: u32 = S905_AO_RTI_PIN_MUX_BASE + 0x012C;
const S905_GPIO_AO_BASE: u32 = 0xC810_0000;
// bits 0-13: output enable; bits 16-29: output state
const S905_GPIO_AO_OUT: u32 = S905_GPIO_AO_BASE + 0x0024;

const S905_ETH_LINK: u32 = 1 << 14;
const S905_ETH_ACTIVE: u32 = 1 << 15;
const S905_ETH_LEDS: u32 = S905_ETH_LINK | S905_ETH_ACTIVE;
const S905_ETH_LEDS_MASK: u32 = !S905_ETH_LEDS;

// Let the white LED blink.
pub fn vim1_blink(h: &Handle, t: Duration) {
    // On Khadas VIM1, GPIO AO 9 is the SYS LED.
    let ao = S905_GPIO_AO_OUT;

    // Set up GPIOZ_{14,15}
    // Z14 / Z15 are ethernet link / active indicators.
    let z = S905_GPIOZ_OUT;

    // function switch to ETH_LINK_LED / ETH_ACTIVE_LED
    let v = read_reg(h, t, REG4).unwrap();
    println!("{v:08x?}");
    write_reg(h, t, REG4, v | (1 << 25) | (1 << 24)).unwrap();

    // I _think_ this _should_ be correct... but what do I know?
    let v = read_reg(h, t, S905_PULL_UP_REG3).unwrap();
    println!("{v:08x?}");
    write_reg(h, t, S905_PULL_UP_REG3, v | S905_ETH_LEDS).unwrap();

    if false {
        let v = read_reg(h, t, S905_PULL_UP_EN_REG3).unwrap();
        println!("{v:08x?}");
        write_reg(h, t, S905_PULL_UP_EN_REG3, v | S905_ETH_LEDS).unwrap();

        let v = read_reg(h, t, S905_GPIOZ_OE).unwrap();
        println!("{v:08x?}");
        // FIXME: This runs into IO or timeout errors; something crashes?!
        write_reg(h, t, S905_GPIOZ_OE, v & S905_ETH_LEDS_MASK).unwrap();
    }
    println!("Blink the SYS LED on Khadas VIM1");
    let dur = Duration::from_millis(300);
    for _ in 0..4 {
        // read_mem(h, t, ao, 4).unwrap();
        // [0xbf, 0xff, 0x3f, 0xff];
        // i.e., 0xbfff_3fff (0b1011_1111__1111_1111___0011_1111__1111_1111)
        // want  0xbdff_3dff (0b1011_1101__1111_1111___0011_1101__1111_1111)
        //                             ^on                    ^enable
        let val = 0xbdff_3dff;
        write_reg(h, t, ao, val).unwrap();
        sleep(dur);
        let val = 0xbfff_3dff;
        write_reg(h, t, ao, val).unwrap();
        sleep(dur);
        if false {
            // initial values: 0xff 0xff 0xff 0xff
            let v = read_reg(h, t, z).unwrap();
            let v = v & S905_ETH_LEDS_MASK;
            write_reg(h, t, z, v).unwrap();
            sleep(dur);
            let v = read_reg(h, t, z).unwrap();
            let v = v | S905_ETH_LEDS;
            write_reg(h, t, z, v).unwrap();
            sleep(dur);
        }
    }
}

// from AML S905D3 / A311D manual
const S905D3_PERIPHS_MUX_BASE: usize = 0xff63_4400;

const S905D3_PREG_PAD_GPIO2_EN: usize = S905D3_PERIPHS_MUX_BASE + 0x0058;
const S905D3_PREG_PAD_GPIO2_O: usize = S905D3_PERIPHS_MUX_BASE + 0x005C;

// const S905D3_AO_RTI_PIN_MUX_BASE: usize = 0xff80_0000;

// see Libre Computer AML-A311D-CC V0.2 schematics
// https://hub.libre.computer/t/libre-computer-board-hardware-schematics-links/36
const LED1: u32 = 1 << 3;
const LED2: u32 = 1 << 6;
const LED3: u32 = 1 << 7;

// NOTE: This is all active low.
pub fn lc_a311d_cc_blink(h: &Handle, t: Duration) {
    let addr = S905D3_PREG_PAD_GPIO2_EN as u32;
    let m = 0xffff_ff37;
    let v = read_reg(h, t, addr).unwrap();
    write_reg(h, t, addr, v & m).unwrap();
    println!("Blink the LEDs on Libre Computer AML-A311D-CC");
    let addr = S905D3_PREG_PAD_GPIO2_O as u32;
    let dur = Duration::from_millis(300);
    for _ in 0..4 {
        let val = LED1 | LED3;
        write_reg(h, t, addr, val).unwrap();
        sleep(dur);
        let val = LED2 | LED3;
        write_reg(h, t, addr, val).unwrap();
        sleep(dur);
        let val = LED1 | LED2;
        write_reg(h, t, addr, val).unwrap();
        sleep(dur);
        let val = LED1 | LED2 | LED3;
        write_reg(h, t, addr, val).unwrap();
        sleep(dur);
    }
}

// WIP: This should be the same as for the A311D, but runs into timeouts, then
// errors with "NoDevice".
pub fn lc_s905d3_cc_blink(h: &Handle, t: Duration) {
    let addr = S905D3_PREG_PAD_GPIO2_EN as u32;
    let m = 0xffff_ff37;
    let v = read_reg(h, t, addr).unwrap();
    write_reg(h, t, addr, v & m).unwrap();
    println!("Blink the LEDs on Libre Computer AML-S905D3-CC");
    let addr = S905D3_PREG_PAD_GPIO2_O as u32;
    let dur = Duration::from_millis(300);
    for _ in 0..4 {
        let val = LED1 | LED3;
        write_reg(h, t, addr, val).unwrap();
        sleep(dur);
        let val = LED2 | LED3;
        write_reg(h, t, addr, val).unwrap();
        sleep(dur);
        let val = LED1 | LED2;
        write_reg(h, t, addr, val).unwrap();
        sleep(dur);
        let val = LED1 | LED2 | LED3;
        write_reg(h, t, addr, val).unwrap();
        sleep(dur);
    }
}
