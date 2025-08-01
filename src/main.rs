mod rle;
use std::collections::HashMap;


fn main() {
    println!("RLE for SpeccyTrain");
    let bitmap: Vec<u8> = vec![1, 2, 2, 3, 3, 3, 4, 4, 3, 2, 1];
    rle::rle_print_bitmap_raw(bitmap.clone());
    println!("");
    let rle_ver: Vec<(u8, u8)> = rle::rle(bitmap);
    rle::rle_print_rle_z80(rle_ver.clone());
    rle::rle_write_file_rle_z80("rle.z80", "SOME_LABEL", rle_ver);
    rle::read_remy_write_file_rle_z80("test_remy.map", "REMY_BITMAP_RLE").unwrap();

    let mt_map = HashMap::new();
    rle::remap_z80_defb_file("test_remy.map.z80", "REMY_BITMAP_RLE", mt_map).unwrap();
}

