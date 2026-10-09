/*
VGA BUFFER
Fungsi : Printing Teks
*/

use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;


#[allow(dead_code)] //Buat Supress unused warning
// Debug : Biar rust bisa print isi struct buat debugging
// Clone : Biar rust bisa copy struct.
// Copy : Biar rust bisa copy struct.
// Eq : Biar rust bisa compare struct. Eq mudahnya adalah kakak dari PartialEq.
// PartialEq : Biar rust bisa compare struct.
#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq
)]
//NOTE : Clone adalah supertrait dari Copy, jdi kalo kita make copy kita butuh clone. otherwise bakal error
//NOTE2: Untuk menggunakan Eq kita butuh PartialEq. keadaannya sama kyk Copy dan Clone
#[repr(u8)] //Biar rust paksa nyimpien sebagai u8 atau 1 byte memori
pub enum Color { //List warna
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] //Karena dia dipaksa jadi u8, transparent fungsinya biar rust gak nambahin padding atau apapun, dia ttp 1 byte (u8)
struct ColorCode(u8);

/*
ColorCode adalah struct yang menyimpan informasi warna foreground dan background.
Karena VGA text mode menggunakan 4 bit untuk foreground dan 4 bit untuk background, kita bisa menyimpan kedua informasi tersebut dalam satu byte (u8).
Foreground akan disimpan di 4 bit bawah, sedangkan background akan disimpan di 4 bit atas.
Jadi, untuk membuat ColorCode baru, kita geser background ke kiri sebanyak 4 bit dan kemudian gabungkan dengan foreground menggunakan operasi OR.
 */
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // Ngikutin C biar datanya berurutan
/*
ScreenChar adalah struct yang menyimpan informasi ASCII character dan warna foreground dan background.
nantinya mereka akan di "press" menjadi satu
*/
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;


/*
Buffer adalah struktur yang menyimpan informasi tampilan VGA.
Buffer terdiri dari 2 dimensi, yaitu 25 baris dan 80 karakter.
Setiap baris memiliki 80 karakter, dan setiap karakter memiliki informasi warna foreground dan background.
*/
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
    //Volatile berfungsi biar compiler tidak melakukan optimasi/mengubah urutan operasi
}


/*
Writer adalah struct yang menyimpan informasi tampilan VGA.
Writer memiliki posisi cursor yang berada di kolom 0 dan baris 24.
Writer memiliki warna foreground dan background yang berwarna putih.
Kasarnya dia adalah object yg kita pake untuk menulis teks ke layar.
*/
pub struct Writer {
    column_position: usize, //dia ada di kolom berapa (0 sampai 79)
    color_code: ColorCode, //Warna saat ini
    buffer: &'static mut Buffer, //referensi ke layar
}

impl Writer {
    /*
    Fungsi ini akan menulis sebuah byte ke layar.
    Jika byte adalah newline, maka akan membuat baris baru.
    Jika byte adalah printable ASCII, maka akan menulis byte ke layar.
    Jika byte bukan printable ASCII, maka akan menulis byte 0xfe ke layar.
    */
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }
    // fungsi untuk membuat baris baru
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }
    // fungsi untuk menghapus baris
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // representasi printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }

        }
    }
    // fungsi untuk menulis string pada posisi tertentu (row, col)
    #[allow(dead_code)]
    pub fn write_at(&mut self, row: usize, col: usize, s: &str) {
        let mut current_col = col;
        for byte in s.bytes() {
            if byte == b'\n' || current_col >= BUFFER_WIDTH {
                break;
            }
            match byte {
                0x20..=0x7e => {
                    let color_code = self.color_code;
                    self.buffer.chars[row][current_col].write(ScreenChar {
                        ascii_character: byte,
                        color_code,
                    });
                    current_col += 1;
                }
                _ => {}
            }
        }
    }

    // Membersihkan seluruh layar VGA dengann karakter spasi kosong.
    // Membersihkan seluruh layar VGA dengan karakter spasi kosong.
    // Setelah dibersihkan, teks berikutnya mulai dicetak dari kolom 0 pada
    // baris terbawah (baris 24), karena Writer selalu menulis ke baris terakhir.
    pub fn clear_screen(&mut self) {
        let blank = ScreenChar{
            ascii_character: b' ',
            color_code: self.color_code,
        };

        // Perulangan untuk setiap piksel karakter
        for row in 0..BUFFER_HEIGHT{
            for col in 0..BUFFER_WIDTH{
                self.buffer.chars[row][col].write(blank);
            }
        }

        self.column_position = 0;
    }

     /// Mengubah kombinasi warna teks (foreground) dan latar belakang (background)
     pub fn set_color(&mut self, foreground: Color, background: Color) {
        self.color_code = ColorCode::new(foreground, background);
     }
}

#[allow(dead_code)]
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

// Biar Writer bisa digunakan di mana saja
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[allow(dead_code)]
pub fn print_something() {
    // Cara pakenya sekarang berubah dikit karena ada Mutex
    use core::fmt::Write;
    WRITER.lock().write_string("Hello dari Global Writer!");
    write!(WRITER.lock(), " Angka: {}", 42).unwrap();
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}

