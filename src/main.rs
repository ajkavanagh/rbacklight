use std::env;
use std::cmp;
use std::process;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::fs;


// really want to build up the constants, but concat! only allows literals,
// and const_concat! (crate) requires non-stable rust.
//const DRIVER: &'static str = "intel_backlight";
//const DRIVER_PATH: &'static str = "/sys/class/backlight/intel_backlight";
const BRIGHTNESS: &'static str = "/sys/class/backlight/intel_backlight/brightness";
const MAX_BRIGHTNESS: &'static str = "/sys/class/backlight/intel_backlight/max_brightness";
const DEFAULT_DELTA: u32 = 10;   // in percent
const DEFAULT_SET: u32 = 70; // in percent

const VALID_COMMANDS: &[&'static str] = &["get", "set", "inc", "dec", "help"];


struct Config {
    command: String,
    value: u32,
}


impl Config {

    fn new(args: &[String]) -> Result<Config, String> {
        let num_args = args.len();
        if num_args == 1 {
            return Ok(Config {command: "get".to_string(), value: 10});
        }
        let command = args[1].to_lowercase();
        if !VALID_COMMANDS.iter().any(|&s| s == command) {
            return Err(format!("Command '{}' not valid", command));
        }
        if num_args == 2 {
            if command == "set" {
                return Ok(Config {command: command, value: DEFAULT_SET});
            } else {
                return Ok(Config {command: command, value: DEFAULT_DELTA});
            }
        }
        match args[2].parse::<u32>() {
            Ok(v) => if v <= 100 {
                Ok(Config {command: command, value: v})
            } else {
                Err("Value should be 0 - 100".to_string())
            },
            Err(err) => Err(format!("Couldn't parse the value due to '{}'", err)),
        }
    }
}


/// Reads a u64 value from the sysfs path
fn read_sysfs(path: &'static str) -> Result<u32, String> {
    match fs::read_to_string(path) {
        Err(err) => Err(format!("Couldn't read from '{}' due to '{}'", path, err)),
        Ok(s) => s.trim_right().parse::<u32>()
            .and_then(|v| { Ok(v) })
            .or_else(|err| { Err(format!("Couldn't parse value: '{}'", err)) }),
    }
}


/// Writes a u64 value to the sysfs path
fn write_sysfs(path: &'static str, value: u32) -> Result<(), String> {
    match OpenOptions::new().write(true).open(path) {
        Err(err) => Err(format!("Couldn't open '{}' due to '{}'", path, err)),
        Ok(mut f) => f.write_fmt(format_args!("{}", value))
            .or_else(|err| {
                Err(format!("Cound't write '{}' to '{}' because '{}'", value, path, err))
            }),
    }
}


fn maximum() -> Result<u32, String> {
    read_sysfs(MAX_BRIGHTNESS)
}


fn current() -> Result<u32, String> {
    maximum().and_then(|max| {
        read_sysfs(BRIGHTNESS).and_then(|v| { Ok(v *100 / max) })
    })
}


/// get the current backlight as a percentage
fn get() -> Result<(), String> {
    current()
        .and_then(|c| {
            println!("{}", c);
            Ok(())
        })
        .or_else(|err| { Err(err) })
}


/// set the percentage value 0-100 as a percentage of MAX_BRIGHTNESS
fn set(value: u32) -> Result<(), String> {
    let actual = cmp::min(100, value);
    maximum().
        and_then(|max| {
            write_sysfs(BRIGHTNESS, cmp::min(max, (max / 100) * (actual + 1)))
        })
        .or_else(|err| { Err(err) })
}


/// increment the backlight as a percentage
fn inc(value: u32) -> Result<(), String> {
    let delta = cmp::min(100, value);
    current()
        .and_then(|c| { set(c + delta) })
        .or_else(|err| { Err(err) })
}


/// decrement the backlight as a percentage
fn dec(value: u32) -> Result<(), String> {
    current()
        .and_then(|c| {
            if value > c { set(0) } else { set(c - value) }
        })
        .or_else(|err| { Err(err) })
}


fn usage() -> Result<(), String> {
    eprintln!("Usage: rbacklight [get|set|inc|dec] [amount%]");
    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Couldn't parse arguments: {}", err);
        usage().unwrap();
        process::exit(1);
    });
    let result = match config.command.as_ref() {
        "get" => get(),
        "set" => set(config.value),
        "inc" => inc(config.value),
        "dec" => dec(config.value),
        _ => usage(),
    };
    result.unwrap_or_else(|err| {
        eprintln!("Couldn't run command: '{}' because '{}'", config.command, err);
        usage().unwrap();
        process::exit(1);
    });
}
