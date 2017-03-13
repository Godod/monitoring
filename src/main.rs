#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate hashindexed;
extern crate term_painter;
extern crate term;

use std::{thread, time};

use term_painter::ToStyle;
use term_painter::Color::*;

mod utils;
mod cpu;
mod hdd;
mod memory;

fn main() {
    let mut term_buffer = term::stdout().expect("Terminal not found");

    loop {
        let mut line_printed = 0;

        let cpu: Vec<cpu::Processor> = cpu::cpu_information();
        let memory: Vec<memory::Memory> = memory::mem_information();
        let hdd: Vec<hdd::HDD> = hdd::hdd_information();

        let _ = writeln!(term_buffer, "{:#^1$}", " CPU ", 75);
        line_printed += 1;

        for processor in cpu.iter() {
            let load_grid_vec = utils::get_print_grid(&processor.load, 50);
            let temp: f32 = processor.temperature as f32;
            let temp_grid_vec = utils::get_print_grid(&temp, 50);
            let freq_grid_vec = utils::get_print_grid(&processor.frequency, 50);

            let _ = writeln!(term_buffer, "CPU {}:", processor.id);
            line_printed += 1;

            // Write load parameters
            let _ = write!(term_buffer, "Load: {:7}[", " ");
            for grid in load_grid_vec.iter() {
                let _ =write!(term_buffer, "{}", BrightGreen.paint(grid));
            }
            // Prevent bug with percent, when clear terminal
            let _ = writeln!(term_buffer, "] {:5.1} %", processor.load);
            line_printed += 1;

            // Write temperature parameters
            let _ = write!(term_buffer, "Temperature: [");
            for grid in temp_grid_vec.iter() {
                let _ = write!(term_buffer, "{}", BrightRed.paint(grid));
            }
            let _ = writeln!(term_buffer, "] {:5.1} C", processor.temperature);
            line_printed += 1;

            // Write frequency
            let _ = write!(term_buffer, "Frequency: {:2}[", " ");
            for grid in freq_grid_vec.iter() {
                let _ = write!(term_buffer, "{}", BrightBlue.paint(grid));
            }
            let _ =writeln!(term_buffer, "] {:5.0} Mhz", processor.frequency);
            line_printed += 1;
        }
        let _ = writeln!(term_buffer, "");
        line_printed += 1;

        let _ = writeln!(term_buffer, "{:#^1$}", " Memory ", 75);
        line_printed += 1;

        for mem in memory.iter() {
            let load_grid_vec = utils::get_print_grid(&mem.load, 50);

            // Write load grid
            let _ = write!(term_buffer, "RAM: {:7}[", " ");
            for grid in load_grid_vec.iter() {
                let _ = write!(term_buffer, "{}", BrightGreen.paint(grid));
            }
            let _ = writeln!(term_buffer, "]");
            line_printed += 1;

            let line = format!("{:3.1} GiB / {:3.1} GiB ({:3.2}%)", mem.idle, mem.total, mem.load);
            let _ = writeln!(term_buffer, "{: ^1$}", line, 75);
            line_printed += 1;
        }

        let _ = writeln!(term_buffer, "");
        line_printed += 1;

        let _ = writeln!(term_buffer, "{:#^1$}", " HDD ", 75);
        line_printed += 1;

        for disk in hdd.iter() {
            let _ = writeln!(term_buffer, "/dev/{}:", disk.name);
            line_printed += 1;
            let _ = writeln!(term_buffer, "Size: {}", disk.size);
            line_printed += 1;

            if !disk.mount_point.is_empty() {
                let _ = writeln!(term_buffer, "Mount point: {}", disk.mount_point);
                line_printed += 1;
            }
            if !disk.fstype.is_empty() {
                let _ = writeln!(term_buffer, "FS Type: {}", disk.fstype);
                line_printed += 1;
            }

            let _ = writeln!(term_buffer, "");
            line_printed += 1;
        }

        // This is needed to clear all output
        for _ in 0..line_printed {
            let _ = term_buffer.cursor_up();
        }

        thread::sleep(time::Duration::from_millis(1500 as u64));
    }
}
