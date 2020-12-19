use acpi::AcpiTables;

pub unsafe fn validate() {
    result = AcpiTables::from_rsdp(acpi_handler(), find_rsdp());
    if result == Err {
        panic!("ACPI-table parsing error");
    }
}

fn acpi_handler() {
    // FIXME
}

fn find_rsdp() -> usize {
    return 0xFF;    // FIXME
}