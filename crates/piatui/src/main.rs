fn main() {
    let (mut rx, tx) = pia_rs::take_connection().unwrap().unwrap();
}
