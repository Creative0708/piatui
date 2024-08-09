fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut rx, tx) = pia_rs::take_connection().unwrap();
    dbg!(rx.poll()?);

    Ok(())
}
