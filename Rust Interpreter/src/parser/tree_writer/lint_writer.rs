#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LintColor {
    Ignore,
    Title,
    TitleBy,
    TitleAuthor,
    TitleImport,
    TitleSeparator,
    Alias1,
    Alias2,
    Alias3,
    Color
}


#[derive(Debug)]
pub struct LintData {
    pub positions: Vec<usize>,
    pub colors: Vec<LintColor>,
}

pub struct LintWriter {
    data: LintData,
    last_index: usize,
    last_color: LintColor,
}

impl LintData {
    pub fn get_iter(&self) -> impl Iterator<Item = (usize, LintColor)> + use<'_> {
        self.positions
            .iter()
            .zip(self.colors.iter())
            .map(|(&a, &b)| (a, b))
    }
}

impl LintWriter {
    pub fn new() -> Self {
        Self {
            data: LintData {
                positions: Vec::new(),
                colors: Vec::new(),
            },
            last_index: 0,
            last_color: LintColor::Ignore,
        }
    }

    pub fn write_up_to(&mut self, index: usize) {
        self.write_up_to_as(LintColor::Ignore, index);
    }

    pub fn write_up_to_as(&mut self, color: LintColor, index: usize) {
        if index < self.last_index {
            panic!(
                "index {} should be after the writing index {}",
                index, self.last_index
            )
        } else {
            self.push_color(color, index);
        }
    }

    pub fn write_as(&mut self, color: LintColor, num: usize) {
        // self.last_index += num;
        self.push_color(color, self.last_index + num);
    }

    pub fn into_data(mut self) -> LintData {
        // if self.last_color != LintColor::Ignore {
        //     self.positions.push(self.last_index);
        //     self.colors.push(LintColor::Ignore);
        // }
        // self.push_color(LintColor::Ignore, self.last_index + 1);
        if self.last_color != LintColor::Ignore {
            self.data.positions.push(self.last_index);
            self.data.colors.push(LintColor::Ignore);
        }

        self.data
    }

    fn push_color(&mut self, color: LintColor, index: usize) {
        // if indexes match -- do nothing
        if index > self.last_index {
            if self.last_color != color {
                self.data.positions.push(self.last_index);
                self.data.colors.push(color);
            }
            self.last_index = index;
            self.last_color = color;
        }
    }
}