#![no_std]
#![no_main]
#![feature(abi_efiapi)]

extern crate alloc;

use log::info;
use uefi::proto::console::text::Output;
use uefi::{prelude::*, alloc::exit_boot_services, table::boot::MemoryMapKey};
use uefi::table::boot::{MemoryType, MemoryDescriptor};
use uefi::table::SystemTable;
use uefi::Result;
use uefi_services::{print, println};
use alloc::vec::Vec;
use alloc::vec;

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let boot_services = system_table.boot_services();
    unsafe { uefi::alloc::init(boot_services); }

    let memmap = print_memory_map(&boot_services).unwrap();
    for entry in memmap.iter() {
        println!("Phys: {:#010x}", entry.phys_start);
    }

    boot_services.stall(20_000_000);

    Status::SUCCESS
}

fn print_memory_map(boot_services: &BootServices) -> Result<Vec<MemoryDescriptor>> {
    let memmap_size = boot_services.memory_map_size();
    let entry_nums = (memmap_size.map_size + memmap_size.entry_size - 1) / memmap_size.entry_size;

    // メモリマップを格納するためのバッファサイズを計算する
    // 現状のエントリが入る分だけだと操作の過程で溢れる可能性があるため、
    // エントリを余計にとる（EXTRA_ENTRY_NUM）
    let extra_entry_num = 8;
    let buffer_size = (entry_nums + extra_entry_num) * memmap_size.entry_size;

    // メモリマップを格納するためのバッファを確保する
    // 確保したバッファにメモリマップを格納する
    // uefi::allocクレートを利用することで
    // UEFIのAllocatePoolなどを直接利用する必要がなくなっている
    let mut memmap_buf: Vec<u8> = vec![0; buffer_size];
    let (_, iter) = boot_services.memory_map(memmap_buf.as_mut_slice())?;
    let memmap: Vec<MemoryDescriptor> = iter.copied().collect();

    Ok(memmap)
}