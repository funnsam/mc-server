pub struct Packet<'a> {
    pub kind: u8,
    pub content: &'a [u8]
}

impl<'a> Packet<'a> {
    pub fn new(k: u8) -> Packet<'a> {
        Packet { kind: k, content: &[] }
    }

    pub fn from_slice(d: &'a [u8]) -> Packet<'a> {
        Packet { kind: d[0], content: &d[1..] }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        [&[self.kind], self.content].concat()
    }

    pub fn append(&mut self, d: &[u8]) {
        self.content = [self.content, d].concat().leak()
    }
}
