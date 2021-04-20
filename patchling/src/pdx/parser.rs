use crate::pdx::{PdxBlock, PdxBlockContent, PdxRelation, PdxRelationType, PdxRelationValue};
use anyhow::*;
use std::{borrow::Cow, str::FromStr};

struct ParserCtx<'a, 'b> {
    source: &'a [u8],
    source_str: &'a str,
    cursor: usize,

    file_name: &'b str,
    cur_line: usize,
    cur_col: usize,
}
impl<'a, 'b> ParserCtx<'a, 'b> {
    fn new(file_name: &'b str, src: &'a str) -> Self {
        ParserCtx {
            source: src.as_bytes(),
            source_str: src,
            cursor: 0,
            file_name,
            cur_line: 1,
            cur_col: 1,
        }
    }

    /// Advances the cursor by a given amount.
    fn advance_cur(&mut self, count: usize) -> Result<()> {
        assert_ne!(count, 0);
        ensure!(
            self.cursor + count <= self.source.len(),
            "{}:{}:{}: Unexpected end of PDX source file.",
            self.file_name,
            self.cur_line,
            self.cur_col,
        );
        for _ in 0..count {
            match self.source[self.cursor] {
                b'\r' => {}
                b'\n' => {
                    self.cur_line += 1;
                    self.cur_col = 1;
                }
                _ => {
                    self.cur_col += 1;
                }
            }
        }
        self.cursor += count;
        Ok(())
    }

    /// Skips all whitespace before this point.
    fn skip_whitespace(&mut self) -> Result<()> {
        while self.cursor < self.source.len() {
            match self.source[self.cursor] {
                // skip normal whitespace
                b' ' | b'\t' | b'\r' | b'\n' => {}
                // skip comments
                b'#' => {
                    while self.source[self.cursor] != b'\n' {
                        self.advance_cur(1)?;
                    }
                }
                // we're done!
                _ => break,
            }
            self.advance_cur(1)?;
        }
        Ok(())
    }

    /// Checks if a token is present, and if so, advances past it.
    fn peek_tok(&mut self, expected: &[u8]) -> Result<bool> {
        if self.cursor + expected.len() > self.source.len() {
            Ok(false)
        } else if !self.source_str.is_char_boundary(self.cursor + expected.len()) {
            Ok(false)
        } else {
            let cursor = &self.source[self.cursor..self.cursor + expected.len()];
            Ok(cursor == expected)
        }
    }

    /// Checks if a token is present, and if so, advances past it.
    fn check_tok(&mut self, expected: &[u8]) -> Result<bool> {
        if self.peek_tok(expected)? {
            self.advance_cur(expected.len())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Parses a quoted string.
    fn parse_quoted_str(&mut self) -> Result<Option<Cow<'a, str>>> {
        if !self.check_tok(b"\"")? {
            Ok(None)
        } else {
            // finds the end of the string, and decides if this can be directly spliced
            let mut count = 0;
            let mut contains_escapes = false;
            while self.cursor + count < self.source.len()
                && self.source[self.cursor + count] != b'\"'
            {
                if self.source[self.cursor + count] == b'\\' {
                    contains_escapes = true;
                    count += 1;
                }
                count += 1;
            }

            ensure!(
                self.cursor + count <= self.source.len(),
                "{}:{}:{}: Found unterminated string.",
                self.file_name,
                self.cur_line,
                self.cur_col,
            );

            // parses the actual string itself.
            let tok = &self.source_str[self.cursor..self.cursor + count];
            self.advance_cur(count + 1)?;
            if contains_escapes {
                let mut owned = String::new();
                let mut has_escape = false;
                for ch in tok.chars() {
                    if has_escape {
                        match ch {
                            '\"' | '\\' => {}
                            _ => owned.push('\"'),
                        }
                        owned.push(ch);
                    } else if ch == '\\' {
                        has_escape = false;
                    } else {
                        owned.push(ch);
                    }
                }
                assert!(!has_escape);
                Ok(Some(Cow::Owned(owned)))
            } else {
                Ok(Some(Cow::Borrowed(tok)))
            }
        }
    }

    /// Parses a complex variable.
    fn parse_variable(&mut self) -> Result<Option<PdxRelationValue<'a>>> {
        self.skip_whitespace()?;

        if self.check_tok(b"@")? {
            if self.check_tok(b"\\[")? {
                let mut count = 0;
                while self.cursor + count < self.source.len() {
                    match self.source[self.cursor + count] {
                        b']' => break,
                        _ => {}
                    }
                    count += 1;
                }

                let res = &self.source_str[self.cursor..self.cursor + count];
                self.advance_cur(count + 1)?;
                Ok(Some(PdxRelationValue::VariableExpr(Cow::Borrowed(res))))
            } else {
                Ok(Some(PdxRelationValue::Variable(self.parse_value_id()?)))
            }
        } else {
            Ok(None)
        }
    }

    /// Parses a key identifier.
    // TODO: This is based on CWTools' parser. We might need to improve this further, depending.
    fn parse_key_id(&mut self) -> Result<Cow<'a, str>> {
        self.skip_whitespace()?;

        if let Some(str) = self.parse_quoted_str()? {
            Ok(str)
        } else {
            let mut count = 0;
            while self.cursor + count < self.source.len() {
                match self.source[self.cursor + count] {
                    b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' => {}
                    b'_' | b':' | b'@' | b'.' | b'\"' | b'-' | b'\'' | b'[' | b']' | b'!'
                    | b'<' | b'>' | b'$' | b'^' | b'&' => {}
                    _ => break,
                }
                count += 1;
            }

            ensure!(
                count != 0,
                "{}:{}:{}: Could not parse identifier.",
                self.file_name,
                self.cur_line,
                self.cur_col
            );

            let res = &self.source_str[self.cursor..self.cursor + count];
            self.advance_cur(count)?;
            Ok(Cow::Borrowed(res))
        }
    }

    /// Parses a value identifier.
    // TODO: This is based on CWTools' parser. We might need to improve this further, depending.
    fn parse_value_id(&mut self) -> Result<Cow<'a, str>> {
        self.skip_whitespace()?;

        if let Some(str) = self.parse_quoted_str()? {
            Ok(str)
        } else {
            let mut count = 0;
            for (idx, ch) in self.source_str[self.cursor..].char_indices() {
                match ch {
                    'a'..='z' | 'A'..='Z' | '0'..='9' => {}
                    '_' | '.' | '-' | ':' | ';' | '\'' | '[' | ']' | '@' | '+' | '`' | '%'
                    | '/' | '!' | ',' | '<' | '>' | '?' | '$' | 'š' | 'Š' | '’' | '|' | '^'
                    | '*' | '&' => {}
                    _ => {
                        count = idx;
                        break;
                    }
                }
            }

            ensure!(
                count != 0,
                "{}:{}:{}: Could not parse identifier.",
                self.file_name,
                self.cur_line,
                self.cur_col
            );

            let res = &self.source_str[self.cursor..self.cursor + count];
            self.advance_cur(count)?;
            Ok(Cow::Borrowed(res))
        }
    }

    fn check_end(&mut self) -> Result<bool> {
        self.skip_whitespace()?;
        Ok(self.cursor == self.source.len())
    }
}

impl PdxRelationType {
    fn parse(ctx: &mut ParserCtx<'_, '_>) -> Result<Option<Self>> {
        ctx.skip_whitespace()?;

        if ctx.check_tok(b"==")? {
            Ok(Some(PdxRelationType::Equal))
        } else if ctx.check_tok(b"<=")? {
            Ok(Some(PdxRelationType::LessOrEqual))
        } else if ctx.check_tok(b">=")? {
            Ok(Some(PdxRelationType::GreaterOrEqual))
        } else if ctx.check_tok(b"<")? {
            Ok(Some(PdxRelationType::LessThan))
        } else if ctx.check_tok(b">")? {
            Ok(Some(PdxRelationType::GreaterThan))
        } else if ctx.check_tok(b"=")? {
            Ok(Some(PdxRelationType::Normal))
        } else {
            Ok(None)
        }
    }
}

impl<'a> PdxBlockContent<'a> {
    fn parse(ctx: &mut ParserCtx<'a, '_>) -> Result<Self> {
        let key = ctx.parse_key_id()?;
        if let Some(relation) = PdxRelationType::parse(ctx)? {
            ctx.skip_whitespace()?;
            let value = if ctx.peek_tok(b"{")? {
                PdxRelationValue::Block(PdxBlock::parse_bracketed(ctx)?)
            } else if let Some(var) = ctx.parse_variable()? {
                var
            } else {
                let raw_value = ctx.parse_value_id()?;
                if let Ok(float_value) = f64::from_str(&raw_value) {
                    PdxRelationValue::Numeric(float_value)
                } else {
                    PdxRelationValue::String(raw_value)
                }
            };
            Ok(PdxBlockContent::Relation(PdxRelation { tag: key, value, relation }))
        } else {
            Ok(PdxBlockContent::String(key))
        }
    }
}

impl<'a> PdxBlock<'a> {
    fn parse_bracketed(ctx: &mut ParserCtx<'a, '_>) -> Result<Self> {
        let mut contents = Vec::new();
        if ctx.check_tok(b"{")? {
            loop {
                ctx.skip_whitespace()?;
                if ctx.check_tok(b"}")? {
                    break;
                } else {
                    contents.push(PdxBlockContent::parse(ctx)?);
                }
            }
        } else {
            panic!("no opening bracket?");
        }
        Ok(PdxBlock { contents })
    }

    pub fn parse_file(file_name: &str, file_data: &'a [u8]) -> Result<Self> {
        let mut ctx = ParserCtx::new(file_name, std::str::from_utf8(file_data)?);
        ctx.check_tok(b"\xEF\xBB\xBF")?; // remove UTF-8 BOM if one exists.

        let mut contents = Vec::new();
        loop {
            if ctx.check_end()? {
                break;
            }
            contents.push(PdxBlockContent::parse(&mut ctx)?);
        }
        Ok(PdxBlock { contents })
    }
}
