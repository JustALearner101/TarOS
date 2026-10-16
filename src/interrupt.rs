use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use crate::println; // Menggunakan macro cetak milik TarOS

lazy_static! {
    /// Membuat tabel penanganan interupsi (IDT) yang statis di memori
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        // Daftarkan handler khusus jika terjadi Breakpoint (#BP)
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

/// Fungsi untuk mendaftarkan IDT ke dalam register CPU (LIDT)
pub fn init_idt() {
    IDT.load();
}

/// Handler yang dipicu saat CPU mendeteksi instruksi breakpoint (int3)
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame
) {
    println!("EXCEPTION: BREAKPOINT DETECTED");
    println!("Stack Frame Internal: {:#?}", stack_frame);
}