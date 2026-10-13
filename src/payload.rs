use crate::layout::{Chunk, Corpus, Record, Source};
use miniz_oxide::inflate::decompress_to_vec_zlib_with_limit;
use rand::{Rng, rngs::StdRng};

pub type Generator = fn(&mut StdRng) -> String;

pub struct Payloads<'a> {
    pub corpuses: &'a [Corpus],
    pub records: &'a [Record],
    pub chunks: &'a [Chunk],
    pub compressed_bytes: &'a [u8],
    pub generators: &'a [Generator],
}

impl Payloads<'_> {
    pub fn render(&self, source: Source, rng: &mut StdRng) -> Result<String, String> {
        match source {
            Source::Branch { .. } => unreachable!("sample a leaf before rendering"),
            Source::Generator(index) => Ok(self.generators[index as usize](rng)),
            Source::Corpus(index) => {
                let corpus = &self.corpuses[index as usize];
                let selected = rng.random_range(0..u64::from(corpus.record_count)) as usize;
                self.record_text(corpus.first_record as usize + selected)
            }
        }
    }

    fn decode_chunk(&self, index: usize) -> Result<Vec<u8>, String> {
        let chunk = &self.chunks[index];
        let bytes = &self.compressed_bytes
            [chunk.offset as usize..chunk.offset as usize + chunk.compressed_len as usize];
        let decoded = decompress_to_vec_zlib_with_limit(bytes, chunk.decoded_len as usize)
            .map_err(|e| format!("cannot decompress chunk {index}: {e:?}"))?;
        if decoded.len() != chunk.decoded_len as usize {
            return Err(format!("invalid decoded length for chunk {index}"));
        }
        Ok(decoded)
    }

    pub fn record_text(&self, index: usize) -> Result<String, String> {
        let record = &self.records[index];
        let mut decoded = self.decode_chunk(record.chunk as usize)?;
        let range = record.offset as usize..record.offset as usize + record.len as usize;
        if decoded.get(range.clone()).is_none() {
            return Err(format!("invalid byte range for record {index}"));
        }
        // Reuse the decompression buffer; validate only the selected record's UTF-8.
        decoded.copy_within(range, 0);
        decoded.truncate(record.len as usize);
        String::from_utf8(decoded).map_err(|e| format!("invalid UTF-8 for record {index}: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miniz_oxide::deflate::compress_to_vec_zlib;
    use rand::SeedableRng;

    #[test]
    fn retrieval_uses_byte_ranges_and_only_the_selected_chunk() {
        let text = "前\nhello\n世界";
        let bytes = compress_to_vec_zlib(text.as_bytes(), 9);
        let chunks = [
            Chunk {
                offset: 0,
                compressed_len: bytes.len() as u32,
                decoded_len: text.len() as u32,
            },
            // This unselected chunk is deliberately corrupt.
            Chunk {
                offset: 0,
                compressed_len: 1,
                decoded_len: 1,
            },
        ];
        let records = [Record {
            chunk: 0,
            offset: 4,
            len: 12,
        }];
        let payloads = Payloads {
            corpuses: &[Corpus {
                first_record: 0,
                record_count: 1,
            }],
            records: &records,
            chunks: &chunks,
            compressed_bytes: &bytes,
            generators: &[],
        };
        assert_eq!(
            payloads
                .render(Source::Corpus(0), &mut StdRng::seed_from_u64(42))
                .unwrap(),
            "hello\n世界"
        );
    }

    #[test]
    fn corrupt_lengths_ranges_and_utf8_are_errors() {
        let bytes = compress_to_vec_zlib(b"abc", 9);
        let chunks = [Chunk {
            offset: 0,
            compressed_len: bytes.len() as u32,
            decoded_len: 4,
        }];
        let records = [Record {
            chunk: 0,
            offset: 2,
            len: 2,
        }];
        let payloads = Payloads {
            corpuses: &[],
            records: &records,
            chunks: &chunks,
            compressed_bytes: &bytes,
            generators: &[],
        };
        assert!(
            payloads
                .record_text(0)
                .unwrap_err()
                .contains("decoded length")
        );
        let chunks = [Chunk {
            decoded_len: 2,
            ..chunks[0]
        }];
        let payloads = Payloads {
            chunks: &chunks,
            ..payloads
        };
        assert!(payloads.record_text(0).unwrap_err().contains("decompress"));
        let chunks = [Chunk {
            decoded_len: 3,
            ..chunks[0]
        }];
        let payloads = Payloads {
            chunks: &chunks,
            ..payloads
        };
        assert!(payloads.record_text(0).unwrap_err().contains("byte range"));

        let bytes = compress_to_vec_zlib("é".as_bytes(), 9);
        let chunks = [Chunk {
            offset: 0,
            compressed_len: bytes.len() as u32,
            decoded_len: 2,
        }];
        let records = [Record {
            chunk: 0,
            offset: 1,
            len: 1,
        }];
        let payloads = Payloads {
            records: &records,
            chunks: &chunks,
            compressed_bytes: &bytes,
            ..payloads
        };
        assert!(payloads.record_text(0).unwrap_err().contains("UTF-8"));
    }
}
