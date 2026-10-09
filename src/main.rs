#![no_std]
#![no_main]

mod vga_buffer;

// Use "cargo run --target x86_64-taros.json" to run
use core::panic::PanicInfo;



/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // 1. Bersihkan layar bawaan BIOS/Bootloader saat OS pertama kali menyala
    vga_buffer::WRITER.lock().clear_screen();

    // ASCII art dipotong jadi 23 baris biar muat di VGA 80x25
    // Max lebar: 67 kolom (aman di 80 kolom)
    let arch_logo = r#"
                           .,:;;++++++;:,.       )iii+:::;iii))+i='
                        .:;++=iiiiiiiiii=++;.    =::,,,:::=i));=+'
                      ,;+==ii)))))))))))ii==+;,      ,,,:=i))+=:
                    ,;+=ii))))))IIIIII))))ii===;.    ,,:=i)=i+
                   ;+=ii)))IIIIITIIIIII))))iiii=+,   ,:=));=,
                 ,+=i))IIIIIITTTTTITIIIIII)))I)i=+,,:+i)=i+
                ,+i))IIIIIITTTTTTTTTTTTI))IIII))i=::i))i='
               ,=i))IIIIITLLTTTTTTTTTTIITTTTIII)+;+i)+i`
               =i))IIITTLTLTTTTTTTTTIITTLLTTTII+:i)ii:'
              +i))IITTTLLLTTTTTTTTTTTTLLLTTTT+:i)))=,
              =))ITTTTTTTTTTTLTTTTTTLLLLLLTi:=)IIiii;
             .i)IIITTTTTTTTLTTTITLLLLLLLT);=)I)))))i;
             :))IIITTTTTLTTTTTTLLHLLLLL);=)II)IIIIi=:
             :i)IIITTTTTTTTTLLLHLLHLL)+=)II)ITTTI)i=
             .i)IIITTTTITTLLLHHLLLL);=)II)ITTTTII)i+
             =i)IIIIIITTLLLLLLHLL=:i)II)TTTTTTIII)i'
           +i)i)))IITTLLLLLLLLT=:i)II)TTTTLTTIII)i;
         +ii)i:)IITTLLTLLLLT=;+i)I)ITTTTLTTTII))i;
        =;)i=:,=)ITTTTLTTI=:i))I)TTTLLLTTTTTII)i;
      +i)ii::,  +)IIITI+:+i)I))TTTTLLTTTTTII))=,
    :=;)i=:,,    ,i++::i))I)ITTTTTTTTTTIIII)=+'
  .+ii)i=::,,   ,,::=i)))iIITTTTTTTTIIIII)=+
 ,==)ii=;:,,,,:::=ii)i)iIIIITIIITIIII))i+:'
"#;

    // 2. Ubah warna menjadi Cyan untuk mencetak logo Arch Linux
    vga_buffer::WRITER.lock().set_color(vga_buffer::Color::Cyan, vga_buffer::Color::Black);
    println!("{}", arch_logo);

    // 3. Ubah warna menjadi Hijau Terang untuk teks sambutan sukses
    vga_buffer::WRITER.lock().set_color(vga_buffer::Color::LightGreen, vga_buffer::Color::Black);
    println!("Welcome to TarOS - Arch Edition!");

    // 4. Kembalikan ke warna default (Kuning) agar teks berikutnya konsisten
    vga_buffer::WRITER.lock().set_color(vga_buffer::Color::Yellow, vga_buffer::Color::Black);
    loop {}
}