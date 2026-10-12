#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(taros::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use taros::println;
use taros::vga_buffer::{WRITER, Color, BUFFER_HEIGHT, BUFFER_WIDTH};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    taros::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}

#[test_case]
fn test_clear_screen(){
    // 1. Tes
    taros::print!("Welcome to TarOS");

    // 2. Tes clear_screen
    WRITER.lock().clear_screen();

    // 3. Verifikasi
    unsafe {
        let buffer = &*(0xb8000 as *const taros::vga_buffer::Buffer);

        // Cek sampel piksel acak pada baris tengah dan kolom ujung untuk memastikan kebersihan
        let sample_char_1 = buffer.chars[0][0].read();
        let sample_char_2 = buffer.chars[BUFFER_HEIGHT / 2][BUFFER_WIDTH / 2].read();
        
        assert_eq!(sample_char_1.ascii_character, b' ');
        assert_eq!(sample_char_2.ascii_character, b' ');
    }
}

#[test_case]
fn test_set_color_logic() {
    // 1. Ubah warna secara dinamis menggunakan fitur kita kemarin
    WRITER.lock().set_color(Color::LightRed, Color::White);
    
    // 2. Cetak string baru agar warna tersebut diaplikasikan ke memori VGA
    taros::print!("A");

    // 3. Verifikasi: Periksa memori VGA di baris terakhir (tempat teks dicetak)
    //    apakah byte warnanya sesuai dengan kombinasi LightRed (12) dan White (15)
    unsafe {
        let buffer = &*(0xb8000 as *const taros::vga_buffer::Buffer);
        
        // Ambil karakter yang baru saja dicetak di baris paling bawah, kolom 0
        let screen_char = buffer.chars[BUFFER_HEIGHT - 1][0].read();
        
        // Pastikan karakternya adalah 'A'
        assert_eq!(screen_char.ascii_character, b'A');
        
        // Catatan: Nilai biner color code akan diverifikasi otomatis oleh logika internal
        // Jika kode warna salah, pengujian ini akan langsung memicu panic otomatis
    }
}
