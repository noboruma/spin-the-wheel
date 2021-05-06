mod entry;

use crate::entry::*;

use std::{env, io::{stdout, Write}, iter::{Cycle, Enumerate}, thread::sleep, time::Duration};
use rand::Rng;

use crossterm::{cursor::{Hide, Show}, execute, style::Color};

const COLORS: [Color; 8] = [Color::Blue, Color::Red, Color::Cyan, Color::Grey, Color::Green, Color::White, Color::Yellow, Color::Magenta];
const QUICK_MOTION_NS: u64 = 50_000_000;
const SLOW_MOTION_NS: u64 = 100_000_000;
const ULTRA_SLOW_MOTION_NS: u64 = 250_000_000;

macro_rules! spin_for {
 ( $total_time_ns:expr, $spin_duration_ns:expr, $ratio:expr, $callback:expr ) => {
        {
            let num_spins = $total_time_ns / ($ratio * $spin_duration_ns);
            let sleep_duration = Duration::new(0, $spin_duration_ns as u32);

            for _ in 0..num_spins {
                sleep(sleep_duration);
                $callback;
            }
        }
    };
 ( $num_spins:expr, $spin_duration_ns:expr, $callback:expr ) => {
        {
            let sleep_duration = Duration::new(0, $spin_duration_ns as u32);
            for _ in 0..$num_spins {
                sleep(sleep_duration);
                $callback;
            }
        }
    };
}

fn prev_cycle(it: &mut Cycle::<Enumerate::<std::slice::Iter<Entry>>>, entries: &Vec<Entry>) {
    for _ in 0.. entries.len()-1 {
        it.next();
    }
}

fn select_entry(it: &mut Cycle::<Enumerate::<std::slice::Iter<Entry>>>, entries: &Vec::<Entry>) -> Result<()> {

    prev_cycle(it, entries);
    match it.next() {
        Some((i, e)) => {print_clear_entry(e, i, entries.len()).map_err(|_| Error::Display)?; Ok(())},
        None => Err(Error::Logic),
    }?;

    match it.next() {
        Some((i, e)) => {print_selected_entry(&e, i, entries.len()).map_err(|_| Error::Display)?; Ok(())},
        None => Err(Error::Logic),
    }?;

    Ok(())
}

fn create_entries() -> Vec<Entry> {
    // Assign a color to each entry
    let mut color = COLORS.iter().cycle();

    return env::args().skip(1).map(|x|
        Entry::new(x, color.next().unwrap().clone())
    ).collect();
}

struct CursorVisibility {}

impl CursorVisibility {
    fn new() -> CursorVisibility {
        match execute!(stdout(), Hide) {
            Ok(()) => (),
            Err(_) => println!("Failed to hide cursor"),
        };
        return CursorVisibility{};
    }
}

impl Drop for CursorVisibility {
    fn drop(&mut self) {
        match execute!(stdout(), Show) {
            Ok(()) => (),
            Err(_) => println!("Failed to set cursor back"),
        };
    }
}

fn main() -> Result<()> {

    let entries = create_entries();
    let mut selected_entry_it = entries.iter().enumerate().cycle();
    let mut rng = rand::thread_rng();
    let _cursor  = CursorVisibility::new();

    print_entries(&entries)?;

    let total_time_ns : u64 = 6_000_000_000;
    let winner : u64 = rng.gen_range(0..entries.len()) as u64;

    spin_for!(total_time_ns, QUICK_MOTION_NS, 2, select_entry(&mut selected_entry_it, &entries)?);
    spin_for!(total_time_ns, SLOW_MOTION_NS, 4, select_entry(&mut selected_entry_it, &entries)?);
    spin_for!(total_time_ns, ULTRA_SLOW_MOTION_NS, 4, select_entry(&mut selected_entry_it, &entries)?);
    spin_for!(winner, ULTRA_SLOW_MOTION_NS, select_entry(&mut selected_entry_it, &entries)?);

    Ok(())
}
