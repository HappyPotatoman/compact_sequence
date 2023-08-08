mod dna_to_ascii;
mod rna_to_ascii;

pub struct Encoder {
    mode: crate::Mode,
}

impl Encoder {
    pub fn new(mode: &crate::Mode) -> Self {
        Self { mode: mode.clone() }
    }

    pub fn encoding_map(&self) -> &'static std::collections::HashMap<String, String> {
        match self.mode {
            crate::Mode::DNA => &dna_to_ascii::DNA_ENCODING_MAP,
            crate::Mode::RNA => &rna_to_ascii::RNA_ENCODING_MAP,
        }
    }

    pub fn decoding_map(&self) -> &'static std::collections::HashMap<String, String> {
        match self.mode {
            crate::Mode::DNA => &dna_to_ascii::DNA_DECODING_MAP,
            crate::Mode::RNA => &rna_to_ascii::RNA_DECODING_MAP,
        }
    }
}