use std::string::FromUtf8Error;

pub fn read_str16(data: &[u8], bufsize: usize) -> Result<String, FromUtf8Error> {
    let len = i16::from_be_bytes(data[0..2].try_into().unwrap());

    let mut be_buf = Vec::with_capacity(len as usize);
    for i in 0..len as usize {
        be_buf.push(u16::from_be_bytes(data[i*2+2..i*2+4].try_into().unwrap()));
    }

    let mut raw = vec![0; bufsize];
    let n = ucs2::decode(&be_buf, &mut raw).unwrap();

    String::from_utf8(raw[0..n].to_vec())
}
