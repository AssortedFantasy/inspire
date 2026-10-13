#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Node {
    pub weight: u32,
    pub source: Source,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    Branch {
        first_child: u32,
        child_count: u32,
        total_weight: u64,
    },
    Corpus(u32),
    Generator(u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Name {
    pub offset: u32,
    pub len: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Corpus {
    pub first_record: u32,
    pub record_count: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Record {
    pub chunk: u32,
    pub offset: u32,
    pub len: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Chunk {
    pub offset: u32,
    pub compressed_len: u32,
    pub decoded_len: u32,
}
