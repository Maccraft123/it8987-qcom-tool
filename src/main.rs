use i2cdev::linux::LinuxI2CDevice;
use i2cdev::core::I2CDevice;
use clap::{Parser, Subcommand};
use std::{io, fs};
use std::thread::sleep;
use std::time::Duration;
use std::path::PathBuf;

fn find_i2cdev() -> io::Result<PathBuf> {
    fs::read_dir("/sys/devices/platform/soc@0/bc0000.geniqup/b94000.i2c")?
        .flatten()
        .map(|entry| PathBuf::from(entry.file_name()).file_name().unwrap().to_owned() )
        .find(|name| name.to_str().unwrap().starts_with("i2c-"))
        .map(|path| PathBuf::from("/dev/").join(path))
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "Failed to find i2c bus"))
}

fn set_reg(dev: &mut LinuxI2CDevice, num: u8, val: u8) {
    dev.smbus_write_byte_data(num, val).unwrap()
}

fn reg(dev: &mut LinuxI2CDevice, num: u8) -> u8 {
    dev.smbus_read_byte_data(num).unwrap()
}

fn reg16(dev: &mut LinuxI2CDevice, num: u8) -> u16 {
    dev.smbus_read_word_data(num).unwrap()
}

const REG_MODERN_STANDBY: u8 = 0x02;
// sleep enter: set_reg(dev, 0x2, 0x1);
// sleep exit: set_reg(dev, 0x2, 0x2);
// display off: set_reg(dev, 0x2, 0x3);
// display on: set_reg(dev, 0x2, 0x4)

const REG_IRQ_REASON: u8 = 0x05;
// common:
// 0x30: fan1 status change
// 0x31: fan2 status change
// 0x32: fan1 speed change
// 0x33: fan2 status change
// 0x34: completed lut update
// 0x35: completed fan profile switch
// 0x36: thermistor 1 thershold cross
// 0x37: thermistor 2 thershold cross
// 0x38: thermistor 3 thershold cross
// 0x39: thermistor 4 thershold cross
// 0x3a: thermistor 5 thershold cross
// 0x3b: thermistor 6 thershold cross
// 0x3c: thermistor 7 thershold cross
// 0x3d: recovered from reset

// lenovo yoga slim 7x specific:
// 0x91: fn+q
// 0x92: fn+m
// 0x93: fn+space
// 0x94: fn+r
// 0x95: fnlock on
// 0x96: fnlock off
// 0x97: fn+n
// 0x9a: ai (?)
// 0x9b: npu (?)

// honor magicbook art 14 specific:
// 0x01: brightness down
// 0x02: brightness up
// 0x04: mute
// 0x05: volume down
// 0x06: volume up
// 0x07: mic off? "Mic function" in dsdt
// 0x08: camera toggle
// 0x0a: hi center
// 0x0b: sidebar
// 0x0c: screenshot
// 0x0e: touchpad enable
// 0x0f: touchpad disable
// 0x10: "cycle cut lcd refresh rate function"
// 0x12: touchscreen disable
// 0x13: touchscreen enable
// 0x14, 0x15, 0x16: keyboard backlight
// 0x17: auto keyboard backlight
// 0x40: "set thermal table to table 9 normal mode"
// 0x41: "set thermal table to table 3 fan_failure2"
// 0x42: "set thermal table to table 4 fan_failure1"
// 0x43: "set thermal table to table 5 HOT_protect"
// 0x44: "set thermal table to table 7 Normal_Mode30C"
// 0x45: "set thermal table to table 6 Safety_Mode"
// 0x46: "set thermal table to MFG mode"
// 0x54: "Turn off Tcon when 4S shutdown"
// 0x5a: camera in
// 0x5b: camera out
// 0x5c: camera forward
// 0x5d: camera reverse

const REG_IRQ_ENABLE: u8 = 0x35;
// enable: 1
// disable: 0

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    ListenToEvents,
    Read8 {
        reg: Vec<u8>,
    },
    Write8 {
        reg: u8,
        val: u8,
    },
}

fn main() {
    let args = Args::parse();
    let path = find_i2cdev().unwrap();
    println!("Using i2c device {:?}", path);
    let mut dev = LinuxI2CDevice::new(path, 0x76).unwrap();

    match args.cmd {
        Command::ListenToEvents => {
            set_reg(&mut dev, REG_IRQ_ENABLE, 1);
            loop {
                let data = dev.smbus_read_byte_data(REG_IRQ_REASON).unwrap();
                match data {
                    _ => println!("data {:x?}", data),
                }
                sleep(Duration::from_millis(100));
            }
        },
        Command::Read8{reg} => {
            for r in reg {
                let val = dev.smbus_read_byte_data(r).unwrap();
                println!("{:x} = {:x}", r, val);
            }
        },
        Command::Write8{reg, val} => {
            dev.smbus_write_byte_data(reg, val).unwrap();
        },
    }

}