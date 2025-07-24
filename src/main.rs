mod rle;


fn main() {
    println!("RLE for SpeccyTrain");
    let v: Vec<u8> = vec![1, 2, 3];
    rle::rle(v);
}

