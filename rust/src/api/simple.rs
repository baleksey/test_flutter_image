use std::{io::Cursor};
use byteorder::{LittleEndian, WriteBytesExt};
//==============================================================================
pub struct BMPimage {
    pub width: u32,
    pub height: u32,
    pub bmp: Vec<u8>,
}

//==============================================================================
impl BMPimage {
    pub fn new(width: u32, height: u32, mut pixels: Vec<u8>) -> BMPimage {
        let base_header_size = 54;
        let total_header_size = base_header_size;
        let bitmap_size = width * height * 4;
        let file_length = total_header_size + bitmap_size;

        let additional_capacity = file_length as usize - pixels.len();
        pixels.reserve(additional_capacity);

        let old_len = pixels.len();
        unsafe {  pixels.set_len(file_length as usize); }

        pixels.copy_within(..old_len, total_header_size as usize);
        let mut cursor = Cursor::new(&mut pixels[0..total_header_size as usize]);

        cursor.write_u8(0x42).unwrap(); // "B"
        cursor.write_u8(0x4D).unwrap(); // "M"
        cursor.write_u32::<LittleEndian>(file_length).unwrap(); // file length
        cursor.write_u32::<LittleEndian>(0).unwrap(); // reserved
        cursor.write_u32::<LittleEndian>(total_header_size).unwrap(); // start of the bitmap data
        cursor.write_u32::<LittleEndian>(40).unwrap(); // info header size
        cursor.write_u32::<LittleEndian>(width).unwrap(); // width
        cursor.write_u32::<LittleEndian>(height).unwrap(); // height
        cursor.write_u16::<LittleEndian>(1).unwrap(); // planes
        cursor.write_u16::<LittleEndian>(32).unwrap(); // bits per pixel
        cursor.write_u32::<LittleEndian>(0).unwrap(); // compression
        cursor.write_u32::<LittleEndian>(bitmap_size).unwrap(); // bitmap size
        cursor.write_u32::<LittleEndian>(0).unwrap(); // horizontal resolution
        cursor.write_u32::<LittleEndian>(0).unwrap(); // vertical resolution
        cursor.write_u32::<LittleEndian>(0).unwrap(); // colors in color table
        cursor.write_u32::<LittleEndian>(0).unwrap(); // important color count

        BMPimage {
            width,
            height,
            bmp: pixels,
        }
    }
}


#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

// #[flutter_rust_bridge::frb()] 
pub async fn render_image(width:i64, height:i64) -> BMPimage {
    let tt = std::time::Instant::now();
    let mut img = vec![240; (width * height * 4) as usize];
    // let res = BMPimage{
    //     width: width as u32,
    //     height: height as u32,
    //     bmp: img,
    // };
    let res = BMPimage::new(width as u32, height as u32, img); //-- it's not optimal as well. It wold be great if you show your version of fast pixels to bmp convertsion.
    println!("IN RUST: {:?}", tt.elapsed());
    
    res
}