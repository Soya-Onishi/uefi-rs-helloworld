#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use log::info;
use uefi::proto::console::text::Output;
use uefi::{prelude::*, alloc::exit_boot_services, table::boot::MemoryMapKey};
use uefi::table::boot::MemoryType;
use uefi::table::SystemTable;
use uefi::Result;
use uefi_services::{print, println};

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let boot_services = system_table.boot_services();

    print_memory_map(&boot_services).unwrap();

    boot_services.stall(20_000_000);

    Status::SUCCESS
}

fn print_memory_map(boot_services: &BootServices) -> Result {
    let memmap_size = boot_services.memory_map_size();
    let entry_nums = (memmap_size.map_size + memmap_size.entry_size - 1) / memmap_size.entry_size;

    // メモリマップを格納するためのバッファサイズを計算する
    // 現状のエントリが入る分だけだと操作の過程で溢れる可能性があるため、
    // エントリを余計にとる（EXTRA_ENTRY_NUM）
    let extra_entry_num = 8;
    let buffer_size = (entry_nums + extra_entry_num) * memmap_size.entry_size;

    // メモリマップを格納するためのバッファを確保する
    // 確保したバッファにメモリマップを格納する
    let memmap_buf = boot_services.allocate_pool(MemoryType::BOOT_SERVICES_DATA, buffer_size)?;
    let memmap_buf = unsafe { core::slice::from_raw_parts_mut(memmap_buf, buffer_size) };
    let (key, _) = boot_services.memory_map(memmap_buf)?;

    for entry in 0..2 {
        let start = entry * memmap_size.entry_size;
        let end = start + memmap_size.entry_size;
        for idx in start..end {
            print!(" {:#04x}", memmap_buf[idx]);
            if idx % 8 == 7 {
                println!("");
            }
        }

        println!("");
    }
    Ok(())
}