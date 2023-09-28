use std::{thread::sleep, time::Duration};

const USB_VID_AMLOGIC: u16 = 0x1b8e;
const USB_PID_S905X3: u16 = 0xc003;
const USB_PID_S905X4: u16 = 0xc004;

/* Request types - just one per direction */
// see https://vovkos.github.io/doxyrest/samples/libusb-sphinxdoc/enum_libusb_endpoint_direction.html#doxid-group-libusb-desc-1ga86c880af878493aa8f805c2aba654b8b
// IN
const REQ_TYPE_AMLIN: u8 = 0xc0;
// OUT
const REQ_TYPE_AMLOUT: u8 = 0x40;

/* Actual commands */
const REQ_WRITE_MEM: u8 = 0x01;
const REQ_READ_MEM: u8 = 0x02;

const REQ_IDENTIFY_HOST: u8 = 0x20;

const REQ_TPL_CMD: u8 = 0x30;
const REQ_PASSWORD: u8 = 0x35;
const REQ_NOP: u8 = 0x36;

/* Memory addresses */
// This is on a TV box based on S905X4
const FB_ADDR: u32 = 0x7f80_0000;

// from https://dn.odroid.com/S905/DataSheet/S905_Public_Datasheet_V1.1.4.pdf
// const SYS_AHB_BASE: u32 = 0xC800_0000;
// FIXME: Not working on S905X, taken from pyamlboot PROTOCOL.md
// also found in khadas update tool
// const CHIP_ID_ADDR_X: u32 = SYS_AHB_BASE + 0x0001_3c24;

// from khadas tools / update (objdump is your friend :))
const CHIP_ID_ADDR_S905X: u32 = 0xd900_d400;

fn int_to_bool_str(v: u8) -> &'static str {
    match v {
        1 => "yes",
        _ => "no",
    }
}

enum Command {
    Nop,
    Info,
    ReadMem,
    WriteMem,
    Vim1_Blink,
    LC_A311D_CC_Blink,
    Password,
    Fastboot,
}

type Handle = rusb::DeviceHandle<rusb::GlobalContext>;

fn nop(handle: &Handle, timeout: Duration) {
    println!("nop");

    let buf: [u8; 0] = [0; 0];
    let r = handle.write_control(REQ_TYPE_AMLOUT, REQ_NOP, 0x0, 0x0, &buf, timeout);
    match r {
        Ok(_) => println!("Ok"),
        Err(_) => println!("Nope"),
    }
}

fn info(handle: &Handle, timeout: Duration) {
    println!("Read chip information\n");
    let mut buf: [u8; 8] = [0; 8];
    match handle.read_control(
        REQ_TYPE_AMLIN,
        REQ_IDENTIFY_HOST,
        0x0,
        0x0,
        &mut buf,
        timeout,
    ) {
        Ok(_) => {
            println!("  ROM version:   {}.{}", buf[0], buf[1]);
            println!("  Stage version: {}.{}", buf[2], buf[3]);
            println!("  Need password: {}", int_to_bool_str(buf[4]));
            println!("  Password OK:   {}", int_to_bool_str(buf[5]));
            println!();
        }
        Err(e) => println!("chip_id err: {e:?}"),
    }
    let is_vim1 = true; // Khadas VIM1 board
    if is_vim1 {
        println!("Chip ID:");
        read_mem(handle, timeout, CHIP_ID_ADDR_S905X, 12).unwrap();
    }
}

// We can read max. 64 bytes at a time.
fn read_mem(handle: &Handle, timeout: Duration, addr: u32, size: u8) -> Result<(), &'static str> {
    let addr_l = addr as u16;
    let addr_h = (addr >> 16) as u16;
    println!("read memory @{addr_h:04x}{addr_l:04x}");
    if size > 64 {
        return Err("Memory read size is 64 max");
    }
    let mut buf = vec![0; size as usize];
    match handle.read_control(
        REQ_TYPE_AMLIN,
        REQ_READ_MEM,
        addr_h,
        addr_l,
        &mut buf,
        timeout,
    ) {
        Ok(_) => {
            println!("read_mem: {buf:02x?}");
        }
        Err(e) => println!("read_mem err: {e:?}"),
    }
    Ok(())
}

fn write_mem(
    handle: &Handle,
    timeout: Duration,
    addr: u32,
    buf: &[u8],
) -> Result<(), &'static str> {
    let addr_l = addr as u16;
    let addr_h = (addr >> 16) as u16;
    println!("write to memory @{addr_h:04x}{addr_l:04x}");
    if buf.len() > 64 {
        return Err("Memory write size is 64 max");
    }

    match handle.write_control(REQ_TYPE_AMLOUT, REQ_WRITE_MEM, addr_h, addr_l, buf, timeout) {
        Ok(n) => {
            println!("write_mem success, {n} bytes");
        }
        Err(e) => println!("write_mem err: {e:?}"),
    }
    Ok(())
}

// Just for reference; untested as per pyamlboot
// Password size is 64 bytes
fn password(handle: &Handle, timeout: Duration, buf: &[u8; 64]) {
    println!("password: {buf:02x?}");
    let r = handle.write_control(REQ_TYPE_AMLOUT, REQ_PASSWORD, 0x0, 0x0, buf, timeout);
    println!("{r:?}");
}

// The command needs 0-byte termination, hence CString.
fn tpl_cmd(handle: &Handle, timeout: Duration, cmd: &str) {
    println!("tpl_cmd {cmd}");
    let cmd = std::ffi::CString::new(cmd).expect("C sucks");
    let buf = cmd.as_bytes_with_nul();
    let res = handle.write_control(
        REQ_TYPE_AMLOUT,
        REQ_TPL_CMD,
        0,
        1, // aka sub code - always 1 though?
        buf,
        timeout,
    );
    println!("{res:?}");
}

// NOTE: not yet working, just an attempt
fn password_test(handle: &Handle, timeout: Duration) {
    nop(handle, timeout);

    let pw: [u8; 64] = [0xff; 64];
    password(handle, timeout, &pw);

    /*
    let size = 16;
    let mut buf = vec![0; size as usize];
    for cmd in 0xa0..=0xff {
        println!("Try command {cmd:02x}");
        match handle.read_control(REQ_TYPE_AMLIN, cmd, 0, 0, &mut buf, timeout) {
            Ok(n) => {
                println!("res ({n}): {buf:02x?}");
            }
            Err(e) => println!("err: {e:?}"),
        }
    }
    */
}

// these are also taken from khadas update tool
// const X_ADDR3: u32 = 0xfffc_d400;
// const X_ADDR4: u32 = 0xffff_fc84;

// GPIO AO 0-9,
const GPIO_AO_BASE: u32 = 0xC810_0000;
const GPIO_AO_OUT: u32 = GPIO_AO_BASE + 0x0024;

fn vim1_blink(handle: &Handle, timeout: Duration) {
    // On Khadas VIM1, GPIO AO 9 supposedly is the SYS LED.
    // This seems to be a bit different? It lets the white LED blink.
    let addr = GPIO_AO_OUT;
    // read_mem(handle, timeout, addr, 4).unwrap();
    // [0xff, 0x3f, 0xff, 0xbf];
    // 9-0: output enable
    // 25-16: OUT
    println!("Blink the SYS LED on Khadas VIM1");
    let dur = Duration::from_millis(300);
    for _ in 0..4 {
        let buf: [u8; 4] = [0xff, 0x3d, 0xff, 0xbd];
        write_mem(handle, timeout, addr, &buf).unwrap();
        sleep(dur);
        let buf: [u8; 4] = [0xff, 0x3f, 0xff, 0xbf];
        write_mem(handle, timeout, addr, &buf).unwrap();
        sleep(dur);
    }
}

// from AML A311D manual
const PERIPHS_MUX_BASE: usize = 0xff63_4400;
const AO_RTI_PIN_MUX_BASE: usize = 0xff80_0000;

const PREG_PAD_GPIO2_O: usize = PERIPHS_MUX_BASE + 4 * 0x0017;
const PREG_PAD_GPIO2_EN: usize = PERIPHS_MUX_BASE + 4 * 0x0016;

// see Libre Computer AML-A311D-CC V0.2 schematics
const LED1: u8 = 1 << 3;
const LED2: u8 = 1 << 6;
const LED3: u8 = 1 << 7;

// NOTE: This is all active low.
fn lc_a311d_cc_blink(handle: &Handle, timeout: Duration) {
    let addr = PREG_PAD_GPIO2_EN as u32;
    let buf: [u8; 4] = [0b0011_0111, 0xff, 0xff, 0xff];
    write_mem(handle, timeout, addr, &buf).unwrap();
    println!("Blink the LEDs on Libre Computer AML-A311D-CC");
    let addr = PREG_PAD_GPIO2_O as u32;
    let dur = Duration::from_millis(300);
    for _ in 0..4 {
        let buf: [u8; 4] = [LED1 | LED3, 0x00, 0x00, 0x00];
        write_mem(handle, timeout, addr, &buf).unwrap();
        sleep(dur);
        let buf: [u8; 4] = [LED2 | LED3, 0x00, 0x00, 0x00];
        write_mem(handle, timeout, addr, &buf).unwrap();
        sleep(dur);
        let buf: [u8; 4] = [LED1 | LED2, 0x00, 0x00, 0x00];
        write_mem(handle, timeout, addr, &buf).unwrap();
        sleep(dur);
        let buf: [u8; 4] = [LED1 | LED2 | LED3, 0x00, 0x00, 0x00];
        write_mem(handle, timeout, addr, &buf).unwrap();
        sleep(dur);
    }
}

// TODO: clap (command line argument parser)
fn main() {
    let cmd = Command::LC_A311D_CC_Blink;

    println!("Searching for Amlogic USB devices...");
    let dev = rusb::devices()
        .unwrap()
        .iter()
        .find(|dev| {
            let des = dev.device_descriptor().unwrap();
            let vid = des.vendor_id();
            let pid = des.product_id();

            vid == USB_VID_AMLOGIC && (pid == USB_PID_S905X3 || pid == USB_PID_S905X4)
        })
        .expect("Cannot find Amlogic USB device");
    let des = dev.device_descriptor().unwrap();
    let vid = des.vendor_id();
    let pid = des.product_id();

    let s_type = if pid == USB_PID_S905X3 {
        "S905X, S905X2 or S905X3"
    } else {
        "S905X4"
    };
    println!(
        "Found {vid:04x}:{pid:04x} ({s_type}) on bus {:03}, device {:03}",
        dev.bus_number(),
        dev.address(),
    );

    // TODO: Not sure if this is sensible, or whether to use different
    // timeouts per command...
    let timeout = Duration::from_millis(2500);
    let handle = dev.open().expect("Error opening USB device {e:?}");

    if let Ok(p) = handle.read_product_string_ascii(&des) {
        println!("Product string: {p}");
    }

    if pid == USB_PID_S905X4 {
        password_test(&handle, timeout);
        return;
    }

    // TODO: write_mem, toggle some GPIO / LED on VIM1
    match cmd {
        Command::Nop => {
            nop(&handle, timeout);
        }
        Command::Info => {
            println!("\n=======\n");
            info(&handle, timeout);
            println!();
            // CPU power states, p47
            println!("Power states (?):");
            read_mem(&handle, timeout, 0xc810_00e0, 8).unwrap();
        }
        Command::ReadMem => {
            read_mem(&handle, timeout, FB_ADDR, 64).unwrap();
        }
        Command::Vim1_Blink => {
            vim1_blink(&handle, timeout);
        }
        Command::LC_A311D_CC_Blink => {
            lc_a311d_cc_blink(&handle, timeout);
        }
        Command::WriteMem => {
            // TODO: pass on hex-encoded value from CLI args
        }
        Command::Password => {
            let pw: [u8; 64] = [0xff; 64];
            password(&handle, timeout, &pw);
        }
        Command::Fastboot => {
            tpl_cmd(&handle, timeout, "fastboot");
        }
    }
}
