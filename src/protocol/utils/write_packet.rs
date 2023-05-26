pub fn write_str16(og: &str) -> Result<Vec<u8>, ucs2::Error> {
    let mut buf = vec![0_u16; og.len()];
    let n = ucs2::encode(og, &mut buf)?;
    let mut u8buf = Vec::with_capacity(n+2);
    u8buf.push((n >> 8) as u8);
    u8buf.push(n as u8);
    for i in 0..n {
        let a = buf[i].to_be_bytes();
        u8buf.push(a[0]);
        u8buf.push(a[1]);
    }
    Ok(u8buf)
}
