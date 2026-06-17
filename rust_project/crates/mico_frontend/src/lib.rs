use mico_ir::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub span: SourceSpan,
    pub line: usize,
    pub column: usize,
    pub code: &'static str,
    pub message: String,
}

impl ParseError {
    fn new(span: SourceSpan, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            span,
            line: span.line,
            column: span.column,
            code,
            message: message.into(),
        }
    }
}

pub fn parse_mico(source: &str) -> Result<Design, Vec<ParseError>> {
    let (tokens, eof_span) = lex(source);
    let mut parser = Parser::new(source, tokens, eof_span);
    parser.parse();
    if parser.errors.is_empty() {
        Ok(parser.design)
    } else {
        Err(parser.errors)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    kind: TokenKind,
    span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenKind {
    Ident(String),
    Number(String),
    Symbol(Symbol),
    Other(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Symbol {
    Arrow,
    At,
    Colon,
    Semicolon,
    Comma,
    Dot,
    LParen,
    RParen,
    LBrace,
    RBrace,
}

fn lex(source: &str) -> (Vec<Token>, SourceSpan) {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }
    let eof_span = SourceSpan {
        start: source.len(),
        end: source.len(),
        line: lexer.line,
        column: lexer.column,
    };
    (tokens, eof_span)
}

struct Lexer<'a> {
    source: &'a str,
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
            line: 1,
            column: 1,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_ws_and_comments();
        let (start, ch) = *self.chars.peek()?;
        let line = self.line;
        let column = self.column;

        if ch == '-' && self.source[start..].starts_with("->") {
            self.next_char();
            self.next_char();
            return Some(self.token(TokenKind::Symbol(Symbol::Arrow), start, line, column));
        }

        if is_ident_start(ch) {
            self.next_char();
            while let Some((_, next)) = self.chars.peek().copied() {
                if is_ident_continue(next) {
                    self.next_char();
                } else {
                    break;
                }
            }
            let end = self.next_index();
            return Some(Token {
                kind: TokenKind::Ident(self.source[start..end].to_string()),
                span: SourceSpan {
                    start,
                    end,
                    line,
                    column,
                },
            });
        }

        if ch.is_ascii_digit() {
            self.next_char();
            while let Some((_, next)) = self.chars.peek().copied() {
                if next.is_ascii_digit() {
                    self.next_char();
                } else {
                    break;
                }
            }
            let end = self.next_index();
            return Some(Token {
                kind: TokenKind::Number(self.source[start..end].to_string()),
                span: SourceSpan {
                    start,
                    end,
                    line,
                    column,
                },
            });
        }

        let kind = match ch {
            '@' => TokenKind::Symbol(Symbol::At),
            ':' => TokenKind::Symbol(Symbol::Colon),
            ';' => TokenKind::Symbol(Symbol::Semicolon),
            ',' => TokenKind::Symbol(Symbol::Comma),
            '.' => TokenKind::Symbol(Symbol::Dot),
            '(' => TokenKind::Symbol(Symbol::LParen),
            ')' => TokenKind::Symbol(Symbol::RParen),
            '{' => TokenKind::Symbol(Symbol::LBrace),
            '}' => TokenKind::Symbol(Symbol::RBrace),
            other => TokenKind::Other(other),
        };
        self.next_char();
        Some(self.token(kind, start, line, column))
    }

    fn skip_ws_and_comments(&mut self) {
        loop {
            let Some((idx, ch)) = self.chars.peek().copied() else {
                break;
            };
            if ch.is_whitespace() {
                self.next_char();
                continue;
            }
            if self.source[idx..].starts_with("//") {
                while let Some((_, c)) = self.next_char() {
                    if c == '\n' {
                        break;
                    }
                }
                continue;
            }
            break;
        }
    }

    fn next_char(&mut self) -> Option<(usize, char)> {
        let value = self.chars.next()?;
        if value.1 == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(value)
    }

    fn next_index(&mut self) -> usize {
        self.chars
            .peek()
            .map(|(idx, _)| *idx)
            .unwrap_or(self.source.len())
    }

    fn token(&mut self, kind: TokenKind, start: usize, line: usize, column: usize) -> Token {
        Token {
            kind,
            span: SourceSpan {
                start,
                end: self.next_index(),
                line,
                column,
            },
        }
    }
}

fn is_ident_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_ident_continue(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphanumeric()
}

struct Parser<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    pos: usize,
    eof_span: SourceSpan,
    design: Design,
    errors: Vec<ParseError>,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str, tokens: Vec<Token>, eof_span: SourceSpan) -> Self {
        Self {
            source,
            tokens,
            pos: 0,
            eof_span,
            design: Design::default(),
            errors: Vec::new(),
        }
    }

    fn parse(&mut self) {
        while !self.is_eof() {
            if self.at_keyword("clockdom") {
                self.parse_clockdom();
            } else if self.at_keyword("interface") {
                self.parse_interface();
            } else if self.at_keyword("extern") || self.at_keyword("module") {
                self.parse_module();
            } else if self.at_keyword("adapter") {
                self.parse_adapter();
            } else if self.at_keyword("compose") {
                self.parse_compose();
            } else {
                self.error_current(
                    "UnexpectedToken",
                    format!(
                        "unrecognized top-level declaration starting at `{}`",
                        self.token_text()
                    ),
                );
                self.recover_to_statement_end();
                if self.at_symbol(Symbol::RBrace) {
                    self.bump();
                }
            }
        }
    }

    fn parse_clockdom(&mut self) {
        self.expect_keyword("clockdom");
        let name = self.expect_ident_spanned("clock domain name");
        self.expect_symbol(Symbol::LParen, "`(`");
        let clock = self.expect_ident("clock signal");
        self.expect_symbol(Symbol::Comma, "`,`");
        let reset = self.expect_ident("reset signal");
        self.expect_symbol(Symbol::RParen, "`)`");
        if !self.expect_symbol(Symbol::Semicolon, "`;`") {
            self.recover_to_statement_end();
        }

        if let (Some((name, name_span)), Some(clock), Some(reset)) = (name, clock, reset) {
            self.design.source_map.record_clock_domain(&name, name_span);
            self.design
                .clock_domains
                .push(ClockDomain { name, clock, reset });
        }
    }

    fn parse_interface(&mut self) {
        self.expect_keyword("interface");
        let name = self.expect_ident_spanned("interface name");
        self.expect_symbol(Symbol::At, "`@`");
        let domain = self.expect_ident("clock domain");
        if !self.expect_symbol(Symbol::LBrace, "`{`") {
            self.recover_to_statement_end();
            return;
        }

        let mut fields = Vec::new();
        let mut contracts = Vec::new();
        let interface_name = name.as_ref().map(|(name, _)| name.clone());
        while !self.is_eof() && !self.at_symbol(Symbol::RBrace) {
            if self.at_keyword("producer") {
                self.bump();
                self.parse_fields(interface_name.as_ref(), Role::Producer, &mut fields);
            } else if self.at_keyword("consumer") {
                self.bump();
                self.parse_fields(interface_name.as_ref(), Role::Consumer, &mut fields);
            } else if self.at_keyword("contract") {
                self.bump();
                if let Some(contract) = self.parse_interface_contract(interface_name.as_ref()) {
                    contracts.push(contract);
                }
            } else {
                self.error_current(
                    "UnexpectedToken",
                    format!("invalid interface member `{}`", self.token_text()),
                );
                self.recover_to_member_end();
            }
        }
        self.expect_symbol(Symbol::RBrace, "`}`");

        if let (Some((name, name_span)), Some(domain)) = (name, domain) {
            self.design.source_map.record_interface(&name, name_span);
            self.design.interfaces.push(InterfaceDef {
                name,
                domain,
                fields,
                contracts,
            });
        }
    }

    fn parse_fields(
        &mut self,
        interface_name: Option<&Ident>,
        role: Role,
        fields: &mut Vec<FieldDef>,
    ) {
        loop {
            let Some((name, name_span)) = self.expect_ident_spanned("field name") else {
                self.recover_to_statement_end();
                return;
            };
            self.expect_symbol(Symbol::Colon, "`:`");
            let Some(ty) = self.expect_ident("field type") else {
                self.recover_to_statement_end();
                return;
            };
            if let Some(interface_name) = interface_name {
                self.design
                    .source_map
                    .record_interface_field(interface_name, &name, name_span);
            }
            fields.push(FieldDef {
                name,
                ty: ScalarType::parse(&ty.0),
                role: role.clone(),
            });

            if self.consume_symbol(Symbol::Comma) {
                continue;
            }
            break;
        }

        if !self.expect_symbol(Symbol::Semicolon, "`;`") {
            self.recover_to_statement_end();
        }
    }

    fn parse_interface_contract(&mut self, interface_name: Option<&Ident>) -> Option<ContractDef> {
        let name = self.expect_ident_spanned("contract name");
        if !self.expect_symbol(Symbol::Colon, "`:`") {
            self.recover_to_statement_end();
            return name.map(|(name, name_span)| {
                if let Some(interface_name) = interface_name {
                    self.design.source_map.record_interface_contract(
                        interface_name,
                        &name,
                        name_span,
                    );
                }
                ContractDef {
                    name,
                    expr: String::new(),
                }
            });
        }
        let expr = self.raw_until_statement_end();
        if !self.expect_symbol(Symbol::Semicolon, "`;`") {
            self.recover_to_statement_end();
        }
        name.map(|(name, name_span)| {
            if let Some(interface_name) = interface_name {
                self.design
                    .source_map
                    .record_interface_contract(interface_name, &name, name_span);
            }
            ContractDef { name, expr }
        })
    }

    fn parse_module(&mut self) {
        let is_extern = if self.consume_keyword("extern") {
            self.expect_keyword("module");
            true
        } else {
            self.expect_keyword("module");
            false
        };
        let name = self.expect_ident_spanned("module name");
        self.expect_symbol(Symbol::At, "`@`");
        let domain = self.expect_ident("clock domain");
        if !self.expect_symbol(Symbol::LBrace, "`{`") {
            self.recover_to_statement_end();
            return;
        }

        let mut ports = Vec::new();
        let module_name = name.as_ref().map(|(name, _)| name.clone());
        while !self.is_eof() && !self.at_symbol(Symbol::RBrace) {
            let dir = if self.consume_keyword("in") {
                Some(PortDir::In)
            } else if self.consume_keyword("out") {
                Some(PortDir::Out)
            } else {
                self.error_current(
                    "UnexpectedToken",
                    format!("invalid module port declaration `{}`", self.token_text()),
                );
                self.recover_to_member_end();
                None
            };
            let Some(dir) = dir else {
                continue;
            };
            let port_name = self.expect_ident_spanned("port name");
            self.expect_symbol(Symbol::Colon, "`:`");
            let interface = self.expect_ident("interface name");
            if !self.expect_symbol(Symbol::Semicolon, "`;`") {
                self.recover_to_statement_end();
            }
            if let (Some((name, name_span)), Some(interface)) = (port_name, interface) {
                if let Some(module_name) = &module_name {
                    self.design
                        .source_map
                        .record_module_port(module_name, &name, name_span);
                }
                ports.push(PortDef {
                    name,
                    dir,
                    interface,
                });
            }
        }
        self.expect_symbol(Symbol::RBrace, "`}`");

        if let (Some((name, name_span)), Some(domain)) = (name, domain) {
            self.design.source_map.record_module(&name, name_span);
            self.design.modules.push(ModuleDef {
                name,
                domain,
                is_extern,
                ports,
            });
        }
    }

    fn parse_adapter(&mut self) {
        self.expect_keyword("adapter");
        let name = self.expect_ident_spanned("adapter name");
        self.expect_keyword("from");
        let from_interface = self.expect_ident("source interface");
        self.expect_symbol(Symbol::At, "`@`");
        let from_domain = self.expect_ident("source domain");
        self.expect_keyword("to");
        let to_interface = self.expect_ident("destination interface");
        self.expect_symbol(Symbol::At, "`@`");
        let to_domain = self.expect_ident("destination domain");
        if !self.expect_symbol(Symbol::LBrace, "`{`") {
            self.recover_to_statement_end();
            return;
        }

        let mut kind = Ident::from("custom");
        let mut attributes = Vec::new();
        let adapter_name = name.as_ref().map(|(name, _)| name.clone());
        while !self.is_eof() && !self.at_symbol(Symbol::RBrace) {
            if self.consume_keyword("kind") {
                if let Some((parsed_kind, kind_span)) = self.expect_ident_spanned("adapter kind") {
                    if let Some(adapter_name) = &adapter_name {
                        self.design
                            .source_map
                            .record_adapter_kind(adapter_name, kind_span);
                    }
                    kind = parsed_kind;
                }
                if !self.expect_symbol(Symbol::Semicolon, "`;`") {
                    self.recover_to_statement_end();
                }
            } else if self.at_keyword("contract") {
                let contract_span = self.bump().map(|token| token.span);
                let value = self.raw_until_statement_end();
                if !self.expect_symbol(Symbol::Semicolon, "`;`") {
                    self.recover_to_statement_end();
                }
                if let (Some(adapter_name), Some(contract_span)) = (&adapter_name, contract_span) {
                    self.design.source_map.record_adapter_attribute(
                        adapter_name,
                        value.as_str(),
                        contract_span,
                    );
                }
                attributes.push((Ident::from("contract"), value));
            } else if let Some((attr_name, attr_span)) = self.consume_ident_spanned() {
                let value = self.raw_until_statement_end();
                if !self.expect_symbol(Symbol::Semicolon, "`;`") {
                    self.recover_to_statement_end();
                }
                if let Some(adapter_name) = &adapter_name {
                    self.design.source_map.record_adapter_attribute(
                        adapter_name,
                        value.as_str(),
                        attr_span,
                    );
                }
                attributes.push((attr_name, value));
            } else {
                self.error_current(
                    "UnexpectedToken",
                    format!("invalid adapter member `{}`", self.token_text()),
                );
                self.recover_to_member_end();
            }
        }
        self.expect_symbol(Symbol::RBrace, "`}`");

        if let (
            Some((name, name_span)),
            Some(from_interface),
            Some(from_domain),
            Some(to_interface),
            Some(to_domain),
        ) = (name, from_interface, from_domain, to_interface, to_domain)
        {
            self.design.source_map.record_adapter(&name, name_span);
            self.design.adapters.push(AdapterDef {
                name,
                from_interface,
                from_domain,
                to_interface,
                to_domain,
                kind,
                attributes,
            });
        }
    }

    fn parse_compose(&mut self) {
        self.expect_keyword("compose");
        let name = self.expect_ident_spanned("compose name");
        self.expect_symbol(Symbol::At, "`@`");
        let domain = self.expect_ident("clock domain");
        if !self.expect_symbol(Symbol::LBrace, "`{`") {
            self.recover_to_statement_end();
            return;
        }

        let mut instances = Vec::new();
        let mut connections = Vec::new();
        let compose_name = name.as_ref().map(|(name, _)| name.clone());
        while !self.is_eof() && !self.at_symbol(Symbol::RBrace) {
            if self.consume_keyword("inst") {
                let inst_name = self.expect_ident_spanned("instance name");
                self.expect_symbol(Symbol::Colon, "`:`");
                let module = self.expect_ident("module name");
                if !self.expect_symbol(Symbol::Semicolon, "`;`") {
                    self.recover_to_statement_end();
                }
                if let (Some((name, name_span)), Some(module)) = (inst_name, module) {
                    if let Some(compose_name) = &compose_name {
                        self.design.source_map.record_compose_instance(
                            compose_name,
                            &name,
                            name_span,
                        );
                    }
                    instances.push(InstanceDef { name, module });
                }
            } else if self.consume_keyword("connect") {
                let from = self.parse_endpoint();
                self.expect_symbol(Symbol::Arrow, "`->`");
                let to = self.parse_endpoint();
                if !self.expect_symbol(Symbol::Semicolon, "`;`") {
                    self.recover_to_statement_end();
                }
                if let (Some((from, from_span)), Some((to, to_span))) = (from, to) {
                    let connection = ConnectDef {
                        from,
                        to,
                        adapter: None,
                    };
                    if let Some(compose_name) = &compose_name {
                        self.design.source_map.record_endpoint(
                            compose_name,
                            &connection.from,
                            from_span,
                        );
                        self.design.source_map.record_endpoint(
                            compose_name,
                            &connection.to,
                            to_span,
                        );
                        self.design.source_map.record_connection(
                            compose_name,
                            &connection,
                            from_span.covering(to_span),
                        );
                    }
                    connections.push(connection);
                }
            } else if self.consume_keyword("adapt") {
                let from = self.parse_endpoint();
                self.expect_symbol(Symbol::Arrow, "`->`");
                let adapter = self.expect_ident("adapter name");
                self.expect_symbol(Symbol::Arrow, "`->`");
                let to = self.parse_endpoint();
                if !self.expect_symbol(Symbol::Semicolon, "`;`") {
                    self.recover_to_statement_end();
                }
                if let (Some((from, from_span)), Some(adapter), Some((to, to_span))) =
                    (from, adapter, to)
                {
                    let connection = ConnectDef {
                        from,
                        to,
                        adapter: Some(adapter),
                    };
                    if let Some(compose_name) = &compose_name {
                        self.design.source_map.record_endpoint(
                            compose_name,
                            &connection.from,
                            from_span,
                        );
                        self.design.source_map.record_endpoint(
                            compose_name,
                            &connection.to,
                            to_span,
                        );
                        self.design.source_map.record_connection(
                            compose_name,
                            &connection,
                            from_span.covering(to_span),
                        );
                    }
                    connections.push(connection);
                }
            } else {
                self.error_current(
                    "UnexpectedToken",
                    format!("invalid compose member `{}`", self.token_text()),
                );
                self.recover_to_member_end();
            }
        }
        self.expect_symbol(Symbol::RBrace, "`}`");

        if let (Some((name, name_span)), Some(domain)) = (name, domain) {
            self.design.source_map.record_compose(&name, name_span);
            self.design.composes.push(ComposeDef {
                name,
                domain,
                instances,
                connections,
            });
        }
    }

    fn parse_endpoint(&mut self) -> Option<(Endpoint, SourceSpan)> {
        let (instance, instance_span) = self.expect_ident_spanned("endpoint instance")?;
        self.expect_symbol(Symbol::Dot, "`.`");
        let (port, port_span) = self.expect_ident_spanned("endpoint port")?;
        let span = instance_span.covering(port_span);
        Some((Endpoint { instance, port }, span))
    }

    fn raw_until_statement_end(&mut self) -> String {
        let start = self
            .peek()
            .map(|t| t.span.start)
            .unwrap_or(self.eof_span.start);
        let mut end = start;
        while !self.is_eof()
            && !self.at_symbol(Symbol::Semicolon)
            && !self.at_symbol(Symbol::RBrace)
        {
            if let Some(token) = self.bump() {
                end = token.span.end;
            }
        }
        self.source[start..end].trim().to_string()
    }

    fn recover_to_statement_end(&mut self) {
        while !self.is_eof()
            && !self.at_symbol(Symbol::Semicolon)
            && !self.at_symbol(Symbol::RBrace)
        {
            self.pos += 1;
        }
        self.consume_symbol(Symbol::Semicolon);
    }

    fn recover_to_member_end(&mut self) {
        self.recover_to_statement_end();
    }

    fn expect_keyword(&mut self, keyword: &'static str) -> bool {
        if self.consume_keyword(keyword) {
            true
        } else {
            self.error_current("ExpectedKeyword", format!("expected keyword `{keyword}`"));
            false
        }
    }

    fn consume_keyword(&mut self, keyword: &str) -> bool {
        if self.at_keyword(keyword) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn at_keyword(&self, keyword: &str) -> bool {
        matches!(
            self.peek().map(|t| &t.kind),
            Some(TokenKind::Ident(value)) if value == keyword
        )
    }

    fn expect_ident(&mut self, label: &'static str) -> Option<Ident> {
        self.expect_ident_spanned(label).map(|(ident, _)| ident)
    }

    fn expect_ident_spanned(&mut self, label: &'static str) -> Option<(Ident, SourceSpan)> {
        match self.bump() {
            Some(Token {
                kind: TokenKind::Ident(value),
                span,
            }) => Some((Ident::from(value.as_str()), span)),
            Some(token) => {
                self.errors.push(ParseError::new(
                    token.span,
                    "ExpectedIdentifier",
                    format!("expected {label}, found `{}`", token_text(&token.kind)),
                ));
                None
            }
            None => {
                self.errors.push(ParseError::new(
                    self.eof_span,
                    "UnexpectedEof",
                    format!("expected {label}, found end of file"),
                ));
                None
            }
        }
    }

    fn consume_ident_spanned(&mut self) -> Option<(Ident, SourceSpan)> {
        match self.peek().map(|t| &t.kind) {
            Some(TokenKind::Ident(_)) => self.expect_ident_spanned("identifier"),
            _ => None,
        }
    }

    fn expect_symbol(&mut self, symbol: Symbol, label: &'static str) -> bool {
        if self.consume_symbol(symbol) {
            true
        } else {
            self.error_current("ExpectedToken", format!("expected {label}"));
            false
        }
    }

    fn consume_symbol(&mut self, symbol: Symbol) -> bool {
        if self.at_symbol(symbol) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn at_symbol(&self, symbol: Symbol) -> bool {
        matches!(
            self.peek().map(|t| &t.kind),
            Some(TokenKind::Symbol(found)) if *found == symbol
        )
    }

    fn bump(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.pos).cloned();
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn error_current(&mut self, code: &'static str, message: impl Into<String>) {
        let span = self.peek().map(|t| t.span).unwrap_or(self.eof_span);
        self.errors.push(ParseError::new(span, code, message));
    }

    fn token_text(&self) -> String {
        self.peek()
            .map(|token| token_text(&token.kind))
            .unwrap_or_else(|| "end of file".to_string())
    }
}

fn token_text(kind: &TokenKind) -> String {
    match kind {
        TokenKind::Ident(value) | TokenKind::Number(value) => value.clone(),
        TokenKind::Symbol(symbol) => symbol_text(*symbol).to_string(),
        TokenKind::Other(ch) => ch.to_string(),
    }
}

fn symbol_text(symbol: Symbol) -> &'static str {
    match symbol {
        Symbol::Arrow => "->",
        Symbol::At => "@",
        Symbol::Colon => ":",
        Symbol::Semicolon => ";",
        Symbol::Comma => ",",
        Symbol::Dot => ".",
        Symbol::LParen => "(",
        Symbol::RParen => ")",
        Symbol::LBrace => "{",
        Symbol::RBrace => "}",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_clockdom_only() {
        let d = parse_mico("clockdom Sys(clk, rst);").unwrap();
        assert_eq!(d.clock_domains.len(), 1);
        assert_eq!(d.clock_domains[0].name.0, "Sys");
    }

    #[test]
    fn parses_stream_fifo_fixture() {
        let d = parse_mico(include_str!("../../../examples/stream_fifo.mico")).unwrap();
        assert_eq!(d.clock_domains.len(), 1);
        assert_eq!(d.interfaces.len(), 1);
        assert_eq!(d.modules.len(), 3);
        assert_eq!(d.composes.len(), 1);
        assert_eq!(d.composes[0].connections.len(), 2);
    }

    #[test]
    fn parses_cdc_adapter_fixture() {
        let d = parse_mico(include_str!("../../../examples/cdc_fifo.mico")).unwrap();
        assert_eq!(d.clock_domains.len(), 2);
        assert_eq!(d.adapters.len(), 1);
        assert_eq!(d.adapters[0].kind.0, "cdc_fifo");
        assert_eq!(d.adapters[0].attributes[0].0.0, "depth");
        assert_eq!(d.adapters[0].attributes[0].1, "4");
    }

    #[test]
    fn parses_multiline_contract_and_comments() {
        let source = r#"
          clockdom Sys(clk, rst); // domain
          interface StreamU32 @Sys {
            producer payload:u32, valid:bool;
            consumer ready:bool;
            contract stable_payload:
              valid -> stable(payload) until ready;
          }
        "#;
        let d = parse_mico(source).unwrap();
        assert_eq!(d.interfaces[0].contracts.len(), 1);
        assert_eq!(
            d.interfaces[0].contracts[0].expr,
            "valid -> stable(payload) until ready"
        );
    }

    #[test]
    fn reports_line_and_column_for_syntax_errors() {
        let err = parse_mico("clockdom Sys(clk rst);").unwrap_err();
        assert_eq!(err[0].line, 1);
        assert!(err[0].column > 1);
        assert_eq!(err[0].code, "ExpectedToken");
    }

    #[test]
    fn rejects_bad_endpoint_syntax() {
        let source = r#"
          clockdom Sys(clk, rst);
          interface StreamU32 @Sys { producer payload:u32; consumer ready:bool; }
          extern module Producer @Sys { out tx: StreamU32; }
          extern module Consumer @Sys { in rx: StreamU32; }
          compose Top @Sys {
            inst p: Producer;
            inst c: Consumer;
            connect ptx -> c.rx;
          }
        "#;
        let errors = parse_mico(source).unwrap_err();
        assert!(errors.iter().any(|e| e.code == "ExpectedToken"));
    }

    #[test]
    fn recovers_missing_semicolon_boundary() {
        let source = r#"
          clockdom Sys(clk, rst)
          interface StreamU32 @Sys {
            producer payload:u32;
            consumer ready:bool;
          }
        "#;
        let errors = parse_mico(source).unwrap_err();
        assert!(errors.iter().any(|e| e.code == "ExpectedToken"));
    }

    #[test]
    fn recovers_unknown_top_level_block() {
        let source = r#"
          clockdom Sys(clk, rst);
          unknown Thing {
            field value;
          }
          interface StreamU32 @Sys {
            producer payload:u32;
            consumer ready:bool;
          }
        "#;
        let errors = parse_mico(source).unwrap_err();
        assert!(errors.iter().any(|e| e.code == "UnexpectedToken"));
    }

    #[test]
    fn recovers_malformed_contract() {
        let source = r#"
          clockdom Sys(clk, rst);
          interface StreamU32 @Sys {
            producer payload:u32, valid:bool;
            consumer ready:bool;
            contract stable_payload valid -> stable(payload) until ready;
          }
        "#;
        let errors = parse_mico(source).unwrap_err();
        assert!(errors.iter().any(|e| e.code == "ExpectedToken"));
    }

    #[test]
    fn recovers_duplicate_endpoint_tokens() {
        let source = r#"
          clockdom Sys(clk, rst);
          interface StreamU32 @Sys { producer payload:u32; consumer ready:bool; }
          extern module Producer @Sys { out tx: StreamU32; }
          extern module Consumer @Sys { in rx: StreamU32; }
          compose Top @Sys {
            inst p: Producer;
            inst c: Consumer;
            connect p.tx -> c.rx -> c.rx;
          }
        "#;
        let errors = parse_mico(source).unwrap_err();
        assert!(errors.iter().any(|e| e.code == "ExpectedToken"));
    }
}
