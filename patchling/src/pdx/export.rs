use crate::pdx::model::*;
use std::fmt::*;

impl Display for PdxRelationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            PdxRelationType::Normal => f.write_str("="),
            PdxRelationType::LessThan => f.write_str("<"),
            PdxRelationType::GreaterThan => f.write_str(">"),
            PdxRelationType::LessOrEqual => f.write_str("<="),
            PdxRelationType::GreaterOrEqual => f.write_str(">="),
            PdxRelationType::Equal => f.write_str("=="),
        }
    }
}

struct DisplayStr<'a>(&'a str);
impl<'a> Display for DisplayStr<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut requires_escape = false;
        for ch in self.0.chars() {
            match ch {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {}
                '_' | '.' | '-' | ':' | ';' | '\'' | '[' | ']' | '@' | '+' | '`' | '%' | '/'
                | '!' | ',' | '<' | '>' | '?' | '$' | 'š' | 'Š' | '’' | '|' | '^' | '*' | '&' =>
                    {}
                _ => {
                    requires_escape = true;
                    break;
                }
            }
        }

        if requires_escape {
            f.write_str("\"")?;
            for ch in self.0.chars() {
                match ch {
                    '\\' => f.write_str("\\")?,
                    '\"' => f.write_str("\"")?,
                    _ => f.write_char(ch)?,
                }
            }
            f.write_str("\"")?;
            Ok(())
        } else {
            f.write_str(self.0)
        }
    }
}

struct PdxRelationValueDisplay<'a, 'b: 'a> {
    rel: &'a PdxRelationValue<'b>,
    indent_level: usize,
    pretty_print: bool,
}
impl<'a, 'b: 'a> Display for PdxRelationValueDisplay<'a, 'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.rel {
            PdxRelationValue::Block(block) => Display::fmt(
                &PdxBlockDisplay {
                    block,
                    indent_level: self.indent_level,
                    pretty_print: self.pretty_print,
                    outer_braces: true,
                },
                f,
            ),
            PdxRelationValue::String(str) => Display::fmt(&DisplayStr(str.as_ref()), f),
            PdxRelationValue::Numeric(num) => Display::fmt(num, f),
            PdxRelationValue::Variable(var) => write!(f, "@{}", var),
            PdxRelationValue::VariableExpr(expr) => write!(f, "@\\[{}]", expr),
        }
    }
}

impl<'a> Display for PdxRelationValue<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(
            &PdxRelationValueDisplay { rel: self, indent_level: 0, pretty_print: false },
            f,
        )
    }
}

impl<'a> Display for PdxRelation<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} {}", self.tag, self.relation, self.value)
    }
}

impl<'a> Display for PdxBlockContent<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            PdxBlockContent::Relation(rel) => Display::fmt(rel, f),
            PdxBlockContent::String(str) => Display::fmt(&DisplayStr(str.as_ref()), f),
        }
    }
}

struct Indent(usize);
impl Display for Indent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for _ in 0..self.0 {
            f.write_str("    ")?;
        }
        Ok(())
    }
}

struct PdxBlockDisplay<'a, 'b: 'a> {
    block: &'a PdxBlock<'b>,
    indent_level: usize,
    pretty_print: bool,
    outer_braces: bool,
}
impl<'a, 'b: 'a> Display for PdxBlockDisplay<'a, 'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.outer_braces {
            f.write_char('{')?;
            if self.pretty_print && !self.block.contents.is_empty() {
                f.write_char('\n')?;
            } else {
                f.write_char(' ')?;
            }
        }
        for line in &self.block.contents {
            if self.pretty_print {
                Display::fmt(&Indent(self.indent_level), f)?;
            }

            match line {
                PdxBlockContent::Relation(rel) => {
                    let value = PdxRelationValueDisplay {
                        rel: &rel.value,
                        indent_level: self.indent_level + 1,
                        pretty_print: self.pretty_print,
                    };
                    write!(f, "{} {} {}", rel.tag, rel.relation, value)?;
                }
                PdxBlockContent::String(str) => Display::fmt(&DisplayStr(str.as_ref()), f)?,
            }

            if self.pretty_print {
                f.write_char('\n')?;
            } else {
                f.write_char(' ')?;
            }
        }
        if self.outer_braces {
            if self.pretty_print && !self.block.contents.is_empty() {
                Display::fmt(&Indent(self.indent_level - 1), f)?;
            }
            f.write_char('}')?;
        }
        Ok(())
    }
}

impl<'a> Display for PdxBlock<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(
            &PdxBlockDisplay {
                block: self,
                indent_level: 0,
                pretty_print: false,
                outer_braces: true,
            },
            f,
        )
    }
}

impl<'a> PdxBlock<'a> {
    pub fn display_file(&self, outer_braces: bool, pretty_print: bool) -> impl Display + '_ {
        PdxBlockDisplay { block: self, indent_level: 0, pretty_print, outer_braces }
    }
}
