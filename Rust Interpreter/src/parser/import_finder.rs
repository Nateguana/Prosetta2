use super::{imports::{ImportData, ImportParseData}, slice::Slice};

pub struct ImportFinder {
    arr: Vec<ImportParseData>,
}

impl ImportFinder {
    pub fn new(arr: &[ImportParseData]) -> Self {
        Self { arr: arr.to_vec() }
    }

    /// find imports
    pub fn find(&mut self, slice: Slice) -> Vec<ImportData> {
        let mut ret = Vec::new();

        for j in 0..slice.len() {
            let char = slice.str[j].to_ascii_lowercase();
            if char != b'\'' {
                let mut index = 0;
                while index < self.arr.len() {
                    let alias = &mut self.arr[index];
                    // check if letter matches
                    if alias.alias[alias.index as usize] == char {
                        alias.index += 1;

                        // import has finished
                        if alias.index >= 3 {
                            let import_parse_data = self.arr.swap_remove(index);
                            let element = ImportData {
                                pos: slice.pos + j - 2,
                                alias: import_parse_data.alias,
                                import: import_parse_data.import,
                                length: 3,
                            };

                            ret.push(element);

                            // imports cannot intersect
                            for ele in &mut self.arr {
                                ele.index = 0;
                            }

                            // just removed element -- try index again
                            index -= 1;
                        }
                    } else {
                        alias.index = 0;
                    }
                    index += 1;
                }
            }
        }

        ret
    }
}
