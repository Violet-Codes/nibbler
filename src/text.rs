use crate::parser;

#[derive(Debug, Clone)]
pub struct TextInfo{
    pub file: String,
    pub line: usize,
    pub index: usize
}

#[derive(Debug, Clone)]
pub struct TextIter<Iter>{
    pub iter: Iter,
    pub info: TextInfo
}

impl<Iter: Iterator<Item = char>> Iterator for TextIter<Iter> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Option::None => Option::None,
            Option::Some('\n') => { self.info.line += 1; self.info.index = 0; Option::Some('\n') },
            Option::Some(c) => { self.info.index += 1; Option::Some(c) }
        }
    }
}

pub const fn get_text_info<Iter, Err>()
    -> parser![TextIter<Iter>, Err, TextInfo]
{
    |iter| Ok(iter.info.clone())
}