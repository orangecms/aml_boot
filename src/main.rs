use clap::{Parser, ValueEnum};
use std::time::Duration;

mod blinky;
mod protocol;

const USB_VID_AMLOGIC: u16 = 0x1b8e;
const USB_PID_S905X3: u16 = 0xc003;
const USB_PID_S905X4: u16 = 0xc004;
/* Memory addresses */
// This is on a TV box based on S905X4
const FB_ADDR: u32 = 0x7f80_0000;

#[derive(Clone, Debug, ValueEnum)]
enum Command {
    Nop,
    Info,
    ReadMem,
    WriteMem,
    Vim1_Blink,
    LC_A311D_CC_Blink,
    LC_S905D3_CC_Blink,
    Password,
    Fastboot,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Command to run
    #[arg(short, long)]
    cmd: Command,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    val: u8,
}

fn main() {
    let args = Args::parse();
    let cmd = args.cmd;

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
        protocol::password_test(&handle, timeout);
        return;
    }

    match cmd {
        Command::Nop => {
            protocol::nop(&handle, timeout);
        }
        Command::Info => {
            println!("\n=======\n");
            protocol::info(&handle, timeout);
            println!();
        }
        Command::ReadMem => {
            protocol::read_mem(&handle, timeout, FB_ADDR, 64).unwrap();
        }
        Command::Vim1_Blink => {
            blinky::vim1_blink(&handle, timeout);
        }
        Command::LC_A311D_CC_Blink => {
            blinky::lc_a311d_cc_blink(&handle, timeout);
        }
        Command::LC_S905D3_CC_Blink => {
            blinky::lc_s905d3_cc_blink(&handle, timeout);
        }
        Command::WriteMem => {
            // TODO: pass on hex-encoded value from CLI args
        }
        Command::Password => {
            let pw: [u8; 64] = [0xff; 64];
            protocol::password(&handle, timeout, &pw);
        }
        Command::Fastboot => {
            protocol::tpl_cmd(&handle, timeout, "fastboot");
        }
    }
}
