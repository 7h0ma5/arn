
pub struct DataLink {
    test: isize
}

type Address = isize;

pub struct Frame {
    destination: Address,
    source: Address,
    data: String
}
