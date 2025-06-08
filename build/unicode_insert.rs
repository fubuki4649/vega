pub trait UnicodeInsert {
    fn insert_str_unicode(&mut self, char_idx: usize, insert: &str);
}

impl UnicodeInsert for String {
    fn insert_str_unicode(&mut self, char_idx: usize, insert: &str) {
        // Convert character index to byte index safely
        if let Some(byte_idx) = self.char_indices().nth(char_idx).map(|(i, _)| i) {
            self.insert_str(byte_idx, insert);
        } else {
            self.push_str(insert); // Append if index is past end
        }
    }
}