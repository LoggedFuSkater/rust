use crate::Obfuscateable;

impl <T> Obfuscateable for T
    where T : ToString {
    fn obf(&self) -> String {
        super::obf(&self.to_string())
    }

    fn obfp(&self, padding_bytes: u8) -> String {
        super::obfp(&self.to_string(), padding_bytes)
    }
}
