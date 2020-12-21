use acpi::AcpiTables;
use bootloader::BootInfo;
use memcmp::Memcmp;

pub unsafe fn validate(start: *const usize) {
    let rdsp : usize = find_rsdp(start, 1);
    if rdsp == start - 1 {
        panic!("RDSP not found");
    }
    result = AcpiTables::from_rsdp(acpi_handler(), rsdp);
    if result == Err {
        panic!("ACPI-table parsing error");
    }
}

fn acpi_handler() {
    //TODO
}

unsafe fn find_rsdp(start : *const usize, len : usize) -> usize {
    for i in 0..len - 8 {
        let addr : *const usize = start + i;
        if memcmp("RSDP PTR ", addr, 8) {
            return *addr;
        }
    }
    return start - 1;
}