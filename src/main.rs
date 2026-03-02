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

    println!("{}", arch_logo);
    println!("Welcome to TarOS - Arch Edition!");

    loop {}
}