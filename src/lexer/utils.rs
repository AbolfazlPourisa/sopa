use super::lexer::Lexer;

impl<'a> Lexer<'a> {
    pub fn get_current_line(&self) -> String {
        self.lines[self.line - 1].to_string()
    }
}