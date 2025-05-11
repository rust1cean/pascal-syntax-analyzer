fn main() -> anyhow::Result<()> {
    let data = std::fs::read("input.txt")?;
    let data = String::from_utf8(data).unwrap();

    // for c in data.chars() {
    // println!("{c:?}");
    // }

    Ok(())
}
