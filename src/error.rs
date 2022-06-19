#[derive(Debug, Clone, Copy)]
// a place (or position) in the source file
pub struct SourcePosition {
    // the line number
    pub line: usize,

    // the column number
    pub col: usize,
}

impl SourcePosition {
    pub fn new(line: usize, col: usize) -> Self {
        SourcePosition {
            line: line,
            col: col,
        }
    }
}

impl Default for SourcePosition {
    fn default() -> Self {
        SourcePosition { line: 1, col: 0 }
    }
}
