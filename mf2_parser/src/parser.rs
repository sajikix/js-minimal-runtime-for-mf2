use std::collections::HashMap;

use crate::error::Error;
use crate::error::SyntaxError;
use crate::model;
use crate::validators::is_bidi_char;
use crate::validators::is_name_char;
use crate::validators::is_valid_name_string;
use crate::validators::is_ws_char;
use crate::validators::trim_tail_ws_and_bidi;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mf2Parser {
    src: Vec<char>,
    pos: usize,
}

impl Mf2Parser {
    pub fn new(src: &str) -> Self {
        Self {
            src: src.chars().collect(),
            pos: 0,
        }
    }

    // Utility methods
    fn read_whitespaces(&mut self) -> (String, bool) {
        let mut pos = self.pos;
        let mut result = String::new();
        let mut is_only_bidi_or_empty = true;
        loop {
            if pos == self.src.len() {
                break;
            }
            let c = self.src[pos];
            if !is_ws_char(c) & !is_bidi_char(c) {
                break;
            }
            if is_ws_char(c) {
                is_only_bidi_or_empty = false;
            }
            result.push(c);
            pos += 1;
        }
        (result, is_only_bidi_or_empty)
    }

    fn read_bidis(&mut self) -> String {
        let mut pos = self.pos;
        let mut result = String::new();
        while pos < self.src.len() && is_bidi_char(self.src[pos]) {
            result.push(self.src[pos]);
            pos += 1;
        }
        result
    }

    fn skip_whitespaces(&mut self) {
        let (whitespaces, _) = self.read_whitespaces();
        self.pos += whitespaces.len();
    }

    fn skip_bidis(&mut self) {
        let bidis = self.read_bidis();
        self.pos += bidis.len();
    }

    fn skip_whitespaces_required(&mut self) -> Result<(), SyntaxError> {
        let (whitespaces, is_only_bidi_or_empty) = self.read_whitespaces();
        if is_only_bidi_or_empty {
            if !whitespaces.is_empty() {
                self.pos += whitespaces.len();
            }
            return Err(SyntaxError::EmptyToken(Error::new(self.pos, None, None)));
        }
        self.pos += whitespaces.len();
        Ok(())
    }

    fn skip_whitespaces_required_if_not_followed_by(
        &mut self,
        expected: Vec<char>,
    ) -> Result<(), SyntaxError> {
        let (whitespaces, is_only_bidi_or_empty) = self.read_whitespaces();

        let next_char = self.src[self.pos + whitespaces.len()];

        if expected.contains(&next_char) {
            self.pos += whitespaces.len();
            return Ok(());
        }
        if is_only_bidi_or_empty {
            if !whitespaces.is_empty() {
                self.pos += whitespaces.len();
            }
            return Err(SyntaxError::EmptyToken(Error::new(self.pos, None, None)));
        }
        self.pos += whitespaces.len();
        Ok(())
    }

    fn read_n_chars(&mut self, n: usize) -> String {
        let s = &self.src[self.pos..self.pos + n];
        s.iter().collect()
    }

    fn expect_string(&mut self, expected: &str, consume: bool) -> Result<(), SyntaxError> {
        expected.chars().enumerate().try_for_each(|(i, c)| {
            if self.pos + i >= self.src.len() || self.src[self.pos + i] != c {
                return Err(SyntaxError::MissingSyntax(Error::new(
                    self.pos + i,
                    None,
                    Some(expected.to_string()),
                )));
            }
            Ok(())
        })?;
        if consume {
            self.pos += expected.len();
        }
        Ok(())
    }

    // parser methods
    pub fn parse(&mut self) -> Result<model::Message, SyntaxError> {
        self.pos = 0; // reset position
        let (declarations, is_match) = self.parse_declarations()?;
        if is_match {
            return Ok(model::Message::Select(
                self.parse_select_message(declarations)?,
            ));
        }

        self.skip_whitespaces();
        let quoted = declarations.len() > 0 && self.read_n_chars(2) == "{{";
        if !quoted && self.pos > 0 {
            self.pos = 0
        }

        let pattern = self.parse_pattern(quoted)?;

        if quoted {
            self.skip_whitespaces();
            if self.pos < self.src.len() {
                return Err(SyntaxError::ExtraContent(Error::new(self.pos, None, None)));
            }
        }

        let mut pattern_message = model::PatternMessage::new();
        pattern_message.set_declarations(declarations);
        pattern_message.set_pattern(pattern);
        Ok(model::Message::Pattern(pattern_message))
    }

    fn parse_declarations(&mut self) -> Result<(Vec<model::Declaration>, bool), SyntaxError> {
        let mut declarations: Vec<model::Declaration> = Vec::new();
        let mut is_match = false;
        self.skip_whitespaces();
        loop {
            if self.src[self.pos] == '.' {
                match self.read_n_chars(6).as_str() {
                    ".input" => {
                        self.pos += 6; // consume ".input"
                        let input_decl = self.parse_input_declaration()?;
                        declarations.push(model::Declaration::Input(input_decl));
                    }
                    ".local" => {
                        self.pos += 6; // consume ".local"
                        let local_decl = self.parse_local_declaration()?;
                        declarations.push(model::Declaration::Local(local_decl));
                    }
                    ".match" => {
                        self.pos += 6; // consume ".match"
                        is_match = true;
                        break;
                    }
                    _ => return Err(SyntaxError::ParseError(Error::new(self.pos, None, None))), // if message starts with a dot, it must be a declaration or match
                }
                self.skip_whitespaces();
            } else {
                break;
            }
        }
        Ok((declarations, is_match))
    }

    fn parse_input_declaration(&mut self) -> Result<model::InputDeclaration, SyntaxError> {
        self.skip_whitespaces();
        self.expect_string("{", false)?;
        let value_start = self.pos;
        let value = self.parse_expression_or_markup(false)?;
        match value {
            ParsedExpressionOrMarkup::Expression(model::Expression::Variable(var_expr)) => {
                let mut input_decl = model::InputDeclaration::new();
                input_decl.set_name(var_expr.get_variable_name());
                input_decl.set_value(var_expr);
                Ok(input_decl)
            }
            _ => Err(SyntaxError::BadInputExpression(Error::new(
                value_start,
                None,
                None,
            ))),
        }
    }
    fn parse_local_declaration(&mut self) -> Result<model::LocalDeclaration, SyntaxError> {
        self.skip_whitespaces_required()?;
        self.expect_string("$", true)?;
        let name = self.parse_name()?;
        self.skip_whitespaces();
        self.expect_string("=", true)?;
        self.skip_whitespaces();
        self.expect_string("{", false)?;
        let value = self.parse_expression_or_markup(false)?;
        match value {
            ParsedExpressionOrMarkup::Expression(expr) => {
                let mut local_decl = model::LocalDeclaration::new(expr);
                local_decl.set_name(name);
                Ok(local_decl)
            }
            _ => Err(SyntaxError::BadInputExpression(Error::new(
                self.pos, None, None,
            ))),
        }
    }

    fn parse_select_message(
        &mut self,
        declarations: Vec<model::Declaration>,
    ) -> Result<model::SelectMessage, SyntaxError> {
        self.skip_whitespaces_required()?;
        let mut selectors: Vec<model::VariableRef> = Vec::new();
        while self.src[self.pos] == '$' {
            selectors.push(self.parse_variable()?);
            self.skip_whitespaces_required()?;
        }

        if selectors.len() == 0 {
            return Err(SyntaxError::EmptyToken(Error::new(self.pos, None, None)));
        }

        let mut variants: Vec<model::Variant> = Vec::new();
        while self.pos < self.src.len() {
            variants.push(self.parse_variant()?);
            self.skip_whitespaces();
        }

        let mut select_message = model::SelectMessage::new();
        select_message.set_declarations(declarations);
        select_message.set_selectors(selectors);
        select_message.set_variants(variants);

        Ok(select_message)
    }

    fn parse_expression_or_markup(
        &mut self,
        allow_markup: bool,
    ) -> Result<ParsedExpressionOrMarkup, SyntaxError> {
        let start_pos = self.pos;
        let mut is_markup = false;
        self.pos += 1; // consume the opening '{'
        self.skip_whitespaces();

        let arg = self.parse_value(false)?;
        if arg != ParsedValue::None {
            self.skip_whitespaces_required_if_not_followed_by(vec!['}'])?;
        }

        let mut function_ref = model::FunctionRef::new();
        let mut markup = model::Markup::new();

        match self.src[self.pos] {
            '@' | '}' => {}
            ':' => {
                // parse function
                self.pos += 1; // consume the ':'
                let func_name = self.parse_identifier()?;
                function_ref.set_name(func_name);
                if let Some(options) = self.parse_options()? {
                    function_ref.set_options(options);
                }
            }
            '#' | '/' => {
                // parse markup
                if !allow_markup || arg != ParsedValue::None {
                    return Err(SyntaxError::ParseError(Error::new(start_pos, None, None)));
                }

                is_markup = true;
                self.pos += 1; // consume the '#' or '/'
                let markup_kind = if self.src[self.pos - 1] == '#' {
                    model::MarkupKind::Open
                } else {
                    model::MarkupKind::Close
                };
                markup.set_kind(markup_kind);
                let name = self.parse_identifier()?;
                markup.set_name(name);
                if let Some(options) = self.parse_options()? {
                    markup.set_options(options);
                }
            }
            _ => {
                return Err(SyntaxError::ParseError(Error::new(self.pos, None, None)));
            }
        }

        let attributes = self.parse_attributes()?;

        if (is_markup && markup.kind == model::MarkupKind::Open) && self.src[self.pos] == '/' {
            markup.set_kind(model::MarkupKind::StandAlone);
            self.pos += 1; // consume the '/'
        }

        self.expect_string("}", true)?;

        if is_markup && allow_markup {
            markup.set_attributes(attributes.unwrap_or_default());
            return Ok(ParsedExpressionOrMarkup::Markup(markup));
        }

        match arg {
            ParsedValue::Literal(literal) => {
                let mut literal_expr = model::LiteralExpression::new();
                literal_expr.set_literal(literal);
                // FIXME:
                if !function_ref.name.is_empty() {
                    literal_expr.set_function(function_ref);
                }
                if let Some(attributes_) = attributes {
                    literal_expr.set_attributes(attributes_);
                }
                Ok(ParsedExpressionOrMarkup::Expression(
                    model::Expression::Literal(literal_expr),
                ))
            }
            ParsedValue::Variable(var_ref) => {
                let mut var_expr = model::VariableExpression::new();
                var_expr.set_variable_ref(var_ref);
                // FIXME:
                if !function_ref.name.is_empty() {
                    var_expr.set_function(function_ref);
                }
                if let Some(attributes_) = attributes {
                    var_expr.set_attributes(attributes_);
                }
                Ok(ParsedExpressionOrMarkup::Expression(
                    model::Expression::Variable(var_expr),
                ))
            }
            ParsedValue::None => {
                // FunctionExpression
                // FIXME:
                if function_ref.name.is_empty() {
                    return Err(SyntaxError::EmptyToken(Error::new(start_pos, None, None)));
                }
                //
                let mut func_expr = model::FunctionExpression::new();
                func_expr.set_function(function_ref);
                if let Some(attributes_) = attributes {
                    func_expr.set_attributes(attributes_);
                }
                //
                Ok(ParsedExpressionOrMarkup::Expression(
                    model::Expression::Function(func_expr),
                ))
            }
        }
    }

    pub fn parse_pattern(&mut self, quoted: bool) -> Result<Vec<model::PatternItem>, SyntaxError> {
        if quoted {
            if self.read_n_chars(2) == "{{" {
                self.pos += 2; // consume the opening '{{'
            } else {
                return Err(SyntaxError::MissingSyntax(Error::new(
                    self.pos,
                    None,
                    Some("{{".to_string()),
                )));
            }
        }

        //
        let mut patterns: Vec<model::PatternItem> = Vec::new();
        let mut text = String::new();
        while self.pos < self.src.len() {
            match self.src[self.pos] {
                '{' => {
                    if !text.is_empty() {
                        patterns.push(model::PatternItem::String(text));
                        text = String::new();
                    }
                    let expression_or_markup = self.parse_expression_or_markup(true)?;
                    match expression_or_markup {
                        ParsedExpressionOrMarkup::Expression(expr) => {
                            patterns.push(model::PatternItem::Expression(expr));
                        }
                        ParsedExpressionOrMarkup::Markup(markup) => {
                            patterns.push(model::PatternItem::Markup(markup));
                        }
                    }
                }
                '}' => {
                    if !quoted {
                        return Err(SyntaxError::ParseError(Error::new(self.pos, None, None)));
                    }
                    if !text.is_empty() {
                        patterns.push(model::PatternItem::String(text));
                        text = String::new();
                    }
                    break; // end of pattern
                }
                _ => {
                    text.push(self.src[self.pos]);
                    self.pos += 1; // consume the char
                }
            }
        }

        if !text.is_empty() {
            patterns.push(model::PatternItem::String(text));
        }

        //
        if quoted {
            if self.read_n_chars(2) == "}}" {
                self.pos += 2; // consume the opening '{{'
            } else {
                return Err(SyntaxError::MissingSyntax(Error::new(
                    self.pos,
                    None,
                    Some("}}".to_string()),
                )));
            }
        }

        Ok(patterns)
    }

    pub fn parse_variant(&mut self) -> Result<model::Variant, SyntaxError> {
        let mut keys: Vec<model::VariantKey> = Vec::new();
        while self.pos < self.src.len() {
            if keys.len() > 0 {
                self.skip_whitespaces_required()?;
            } else {
                self.skip_whitespaces();
            }
            let next_char = self.src[self.pos];
            match next_char {
                '{' => {
                    break;
                }
                '*' => {
                    self.pos += 1; // consume the '*'
                    keys.push(model::VariantKey::CatchallKey(model::CatchallKey::new()));
                }
                _ => {
                    let key = self.parse_literal(true)?;
                    match key {
                        Some(literal) => {
                            literal.normalize();
                            keys.push(model::VariantKey::Literal(literal));
                        }
                        None => {
                            return Err(SyntaxError::EmptyToken(Error::new(self.pos, None, None)));
                        }
                    }
                }
            }
        }
        let mut variant = model::Variant::new();
        variant.set_keys(keys);
        variant.set_value(self.parse_pattern(true)?);
        return Ok(variant);
    }

    fn parse_value(&mut self, required: bool) -> Result<ParsedValue, SyntaxError> {
        match self.src[self.pos] {
            '$' => Ok(ParsedValue::Variable(self.parse_variable()?)),
            _ => {
                let start_pos = self.pos;
                let literal = self.parse_literal(required)?;
                match literal {
                    None => {
                        if required {
                            Err(SyntaxError::EmptyToken(Error::new(start_pos, None, None)))
                        } else {
                            Ok(ParsedValue::None)
                        }
                    }
                    Some(literal) => Ok(ParsedValue::Literal(literal)), // update position to the end of the literal
                }
            }
        }
    }

    fn parse_variable(&mut self) -> Result<model::VariableRef, SyntaxError> {
        self.pos += 1; // consume the '$'
        let mut var_ref = model::VariableRef::new();
        let name = self.parse_name()?;
        var_ref.set_name(name);
        Ok(var_ref)
    }

    fn parse_literal(&mut self, required: bool) -> Result<Option<model::Literal>, SyntaxError> {
        if self.src[self.pos] == '|' {
            return Ok(self.parse_quoted_literal()?);
        }

        let value = self.parse_unquoted_literal();

        if value.is_empty() {
            if required {
                return Err(SyntaxError::EmptyToken(Error::new(self.pos, None, None)));
            } else {
                return Ok(None);
            }
        }

        Ok(Some(model::Literal::new(value)))
    }

    fn parse_quoted_literal(&mut self) -> Result<Option<model::Literal>, SyntaxError> {
        self.pos += 1; // consume the '|'
        let mut value = String::new();
        while self.pos < self.src.len() {
            match self.src[self.pos] {
                '\u{5C}' => {
                    let escaped_char = self.src[self.pos + 1];
                    match escaped_char {
                        '{' | '}' | '|' | '\u{5C}' => {
                            value.push('\u{5C}'); // escape char
                            value.push(escaped_char); //escaped char
                            self.pos += 2; // consume the escape char and escaped char
                        }
                        _ => {
                            return Err(SyntaxError::BadEscape(Error::new(
                                self.pos,
                                Some(self.pos + 2),
                                None,
                            )));
                        }
                    }
                }
                '|' => {
                    self.pos += 1; // consume the '|'
                    return Ok(Some(model::Literal::new(value)));
                }
                _ => {
                    value.push(self.src[self.pos]);
                    self.pos += 1; // consume the char
                }
            }
        }
        // if parser reach here, it means parser didn't find the closing '|'
        Err(SyntaxError::MissingSyntax(Error::new(
            self.src.len(),
            None,
            Some("|".to_string()),
        )))
    }

    fn parse_name(&mut self) -> Result<String, SyntaxError> {
        match self.parse_name_value() {
            Some(name) => Ok(name),
            _ => Err(SyntaxError::EmptyToken(Error::new(self.pos, None, None))),
        }
    }

    fn parse_options(
        &mut self,
    ) -> Result<Option<HashMap<String, model::OptionValue>>, SyntaxError> {
        let mut options: HashMap<String, model::OptionValue> = HashMap::new();

        self.skip_whitespaces_required_if_not_followed_by(vec!['/', '}'])?;

        while self.pos < self.src.len() {
            let char = self.src[self.pos];
            if char == '/' || char == '}' || char == '@' {
                break; // end of options
            }
            let start_pos = self.pos;
            let name = self.parse_identifier()?;
            if options.contains_key(&name) {
                return Err(SyntaxError::DuplicateOptionName(Error::new(
                    start_pos,
                    Some(self.pos),
                    None,
                )));
            }
            self.skip_whitespaces();
            self.expect_string("=", true)?;
            self.skip_whitespaces();

            match self.parse_value(true)? {
                ParsedValue::Literal(literal) => {
                    options.insert(name, model::OptionValue::Literal(literal));
                }
                ParsedValue::Variable(var_ref) => {
                    options.insert(name, model::OptionValue::VariableRef(var_ref));
                }
                ParsedValue::None => {
                    return Err(SyntaxError::EmptyToken(Error::new(self.pos, None, None)));
                }
            }
            self.skip_whitespaces_required_if_not_followed_by(vec!['/', '}'])?;
        }
        if options.is_empty() {
            return Ok(None);
        }
        Ok(Some(options))
    }

    fn parse_attributes(
        &mut self,
    ) -> Result<Option<HashMap<String, model::AttributeValue>>, SyntaxError> {
        let mut attributes: HashMap<String, model::AttributeValue> = HashMap::new();

        while self.src[self.pos] == '@' {
            let start_pos = self.pos;
            self.pos += 1; // consume the '@'
            let name = self.parse_identifier()?;
            if attributes.contains_key(&name) {
                return Err(SyntaxError::DuplicateAttribute(Error::new(
                    start_pos,
                    Some(self.pos),
                    None,
                )));
            }
            self.skip_whitespaces_required_if_not_followed_by(vec!['=', '/', '}'])?;
            if self.src[self.pos] == '=' {
                self.pos += 1; // consume the '='
                self.skip_whitespaces();
                let value = self.parse_literal(true)?;
                if let Some(literal) = value {
                    attributes.insert(name, model::AttributeValue::Literal(literal.value));
                } else {
                    return Err(SyntaxError::EmptyToken(Error::new(self.pos, None, None)));
                }
            } else {
                // If no value is provided, treat it as a boolean attribute
                attributes.insert(name, model::AttributeValue::True(true));
            }
        }
        if attributes.is_empty() {
            return Ok(None);
        }
        Ok(Some(attributes))
    }

    fn parse_name_value(&mut self) -> Option<String> {
        self.skip_bidis();
        let mut value = String::new();

        while !is_ws_char(self.src[self.pos])
            && is_name_char(self.src[self.pos])
            && self.pos < self.src.len()
        {
            value.push(self.src[self.pos]);
            self.pos += 1; // consume char
        }
        value = trim_tail_ws_and_bidi(&value);

        if is_valid_name_string(&value) {
            return Some(value);
        } else {
            return None;
        }
    }

    fn parse_identifier(&mut self) -> Result<String, SyntaxError> {
        let mut name = self.parse_name()?;
        if self.src[self.pos] == ':' {
            self.pos += 1; // consume the ':'
            name.push(':'); // add ':' to the name
            name.push_str(&self.parse_name()?); // parse the namespace
        }
        return Ok(name);
    }

    fn parse_unquoted_literal(&mut self) -> String {
        let mut value = String::new();
        while self.pos < self.src.len() && is_name_char(self.src[self.pos]) {
            value.push(self.src[self.pos]);

            self.pos += 1; // consume char
        }
        return value;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParsedValue {
    Literal(model::Literal),
    Variable(model::VariableRef),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParsedExpressionOrMarkup {
    Expression(model::Expression),
    Markup(model::Markup),
}
