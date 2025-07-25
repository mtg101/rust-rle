mod rle;


fn main() {
    println!("RLE for SpeccyTrain");
    let bitmap: Vec<u8> = vec![1, 2, 2, 3, 3, 3, 4, 4, 3, 2, 1];
    rle::rle_print_bitmap_raw(bitmap.clone());
    println!("");
    let rle_ver: Vec<(u8, u8)> = rle::rle(bitmap);
    rle::rle_print_rle_z80(rle_ver.clone());
    rle::rle_write_file_rle_z80(rle_ver);
}

