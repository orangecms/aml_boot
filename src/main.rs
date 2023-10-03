use clap::{Parser, Subcommand, ValueEnum};
use std::time::Duration;

mod blinky;
mod protocol;

const USB_VID_AMLOGIC: u16 = 0x1b8e;
const USB_PID_GX_CHIP: u16 = 0xc003;
const USB_PID_AML_DNL: u16 = 0xc004;

/* Memory addresses */
// This is on a TV box based on S905X4
// const FB_ADDR: u32 = 0x7f80_0000;

#[allow(non_camel_case_types)]
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Board {
    Khadas_Vim1,
    LC_A311D_CC,
    LC_S905D3_CC,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    Nop,
    ChipGen,
    Info,
    ChipInfo,
    ChipId,
    PowerStates,
    ReadMem {
        #[arg(index = 1, value_parser=clap_num::maybe_hex::<u32>)]
        address: u32,

        #[arg(index = 2, default_value_t = 4)]
        count: u8,
    },
    WriteMem {
        #[arg(index = 1, value_parser=clap_num::maybe_hex::<u32>)]
        address: u32,

        #[arg(index = 2, value_parser=clap_num::maybe_hex::<u32>)]
        value: u32,
    },
    Blinky {
        board: Board,
    },
    Password,
    Fastboot,
}

/// Amlogic mask ROM loader tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Command to run
    #[command(subcommand)]
    cmd: Command,
}

fn main() {
    let cmd = Cli::parse().cmd;

    println!("Searching for Amlogic USB devices...");
    let dev = rusb::devices()
        .unwrap()
        .iter()
        .find(|dev| {
            let des = dev.device_descriptor().unwrap();
            let vid = des.vendor_id();
            let pid = des.product_id();

            vid == USB_VID_AMLOGIC && (pid == USB_PID_GX_CHIP || pid == USB_PID_AML_DNL)
        })
        .expect("Cannot find Amlogic USB device");
    let des = dev.device_descriptor().unwrap();
    let vid = des.vendor_id();
    let pid = des.product_id();

    let s_type = if pid == USB_PID_GX_CHIP {
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

    if pid == USB_PID_AML_DNL {
        protocol::password_test(&handle, timeout);
        return;
    }

    match cmd {
        Command::Nop => {
            protocol::nop(&handle, timeout);
        }
        Command::ChipGen => {
            println!("\n=======\n");
            protocol::chip_gen(&handle, timeout);
            println!();
        }
        Command::Info => {
            println!("\n=======\n");
            protocol::info(&handle, timeout);
            println!();
        }
        Command::ChipInfo => {
            println!("\n=======\n");
            protocol::chip_info(&handle, timeout);
            println!();
        }
        Command::ChipId => {
            println!("\n=======\n");
            protocol::chip_id(&handle, timeout);
            println!();
        }
        Command::PowerStates => {
            println!("\n=======\n");
            protocol::power_states(&handle, timeout);
            println!();
        }
        Command::ReadMem { address, count } => {
            protocol::read_mem(&handle, timeout, address, count).unwrap();
            // println!("{v:?}");
        }
        Command::WriteMem { address, value } => {
            let v = value.to_le().to_ne_bytes();
            println!("{address:x}  {value:x}");
            protocol::write_mem(&handle, timeout, address, &v).unwrap();
        }
        /* TODO
        Command::FBTest => {
            protocol::read_mem(&handle, timeout, FB_ADDR, 64).unwrap();
        }
        */
        Command::Blinky { board } => match board {
            Board::Khadas_Vim1 => blinky::vim1_blink(&handle, timeout),
            Board::LC_A311D_CC => blinky::lc_a311d_cc_blink(&handle, timeout),
            Board::LC_S905D3_CC => blinky::lc_s905d3_cc_blink(&handle, timeout),
        },
        Command::Password => {
            let pw = [0xffu8; 64];
            protocol::password(&handle, timeout, &pw);
        }
        Command::Fastboot => {
            protocol::tpl_cmd(&handle, timeout, "fastboot");
        }
    }
}
