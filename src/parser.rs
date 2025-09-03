use crate::lexer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Program {
        body: Vec<Node>,
    },
    VariableDeclaration {
        id: String,
        init: Box<Node>,
    },
    NewExpression {
        callee: Box<Node>,
        arguments: Vec<Node>,
    },
    CallExpression {
        callee: Box<Node>,
        arguments: Vec<Node>,
    },
    MemberExpression {
        object: Box<Node>,
        property: Box<Node>,
    },
    Identifier(String),
    StringLiteral(String),
    NumericLiteral(u64),
}

impl Node {
    pub fn new_variable_declaration(id: String, init: Node) -> Self {
        Node::VariableDeclaration {
            id,
            init: Box::new(init),
        }
    }
    pub fn new_new_expression(callee: Node, arguments: Vec<Node>) -> Self {
        Node::NewExpression {
            callee: Box::new(callee),
            arguments,
        }
    }
    pub fn new_string_literal(value: String) -> Self {
        Node::StringLiteral(value)
    }
    pub fn new_member_expression(object: Node, property: Node) -> Self {
        Node::MemberExpression {
            object: Box::new(object),
            property: Box::new(property),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    body: Vec<Node>,
}

impl Program {
    pub fn new() -> Self {
        Self { body: Vec::new() }
    }

    pub fn set_body(&mut self, body: Vec<Node>) {
        self.body = body;
    }
}

pub struct Parser {
    lexer: lexer::Lexer,
    current_token: Option<lexer::Token>,
}

impl Parser {
    pub fn new(src: &str) -> Self {
        let mut lexer = lexer::Lexer::new(src);
        let first_token = lexer.next();
        Self {
            lexer,
            current_token: first_token,
        }
    }

    fn consume_next_token(&mut self) {
        self.current_token = self.lexer.next();
    }

    fn parse_arguments(&mut self) -> Vec<Node> {
        let mut args = Vec::new();

        loop {
            match &self.current_token {
                Some(lexer::Token::StringLiteral(s)) => {
                    args.push(Node::new_string_literal(s.clone()));
                    self.consume_next_token(); // consume string literal
                }
                Some(lexer::Token::Punctuator(',')) => {
                    self.consume_next_token(); // consume ','
                }
                Some(lexer::Token::Punctuator(')')) => {
                    self.consume_next_token(); // consume ')'
                    break;
                }
                _ => panic!("Unexpected token in arguments: {:?}", self.current_token),
            }
        }
        args
    }

    fn parse_member_expression(&mut self) -> Node {
        let mut members = Vec::new();
        loop {
            match &self.current_token {
                Some(lexer::Token::Identifier(name)) => {
                    let property = name.clone();
                    self.consume_next_token(); // consume identifier
                    members.push(property);
                    match &self.current_token {
                        Some(lexer::Token::Punctuator('.')) => {
                            self.consume_next_token(); // consume '.'
                        }
                        _ => break,
                    }
                }
                _ => break,
            }
        }
        let mut object = Node::Identifier(members[0].clone());
        for property in &members[1..] {
            object = Node::new_member_expression(object, Node::Identifier(property.clone()));
        }
        object
    }

    fn parse_new_expression(&mut self) -> Node {
        let callee = self.parse_member_expression();
        match &self.current_token {
            Some(lexer::Token::Punctuator('(')) => {
                self.consume_next_token(); // consume '('
                let args = self.parse_arguments();
                Node::new_new_expression(callee, args)
            }
            _ => panic!("Expected '(' after callee in new expression"),
        }
    }

    pub fn parse_assignment_expression(&mut self) -> Node {
        match &self.current_token {
            Some(lexer::Token::Keyword(k)) => match k.as_str() {
                "new" => {
                    println!("Parsing 'new' expression");
                    self.consume_next_token(); // consume 'new'
                    let m = self.parse_new_expression();
                    println!("Parsed new expression: {:?}", m);
                    m
                }
                _ => panic!("Unexpected keyword: {}", k),
            },
            Some(lexer::Token::Identifier(name)) => {
                let id = name.clone();
                Node::Identifier(id)
            }
            Some(lexer::Token::Number(n)) => Node::NumericLiteral(*n),
            _ => panic!("Expected identifier or number"),
        }
    }

    pub fn parse_variable_declaration(&mut self) -> Node {
        // 識別子を期待
        let id = match &self.current_token {
            Some(lexer::Token::Identifier(name)) => name.clone(),
            _ => panic!("Expected identifier"),
        };
        // 次のトークンに進む
        self.current_token = self.lexer.next();

        // '=' を期待
        match &self.current_token {
            Some(lexer::Token::Punctuator('=')) => {
                // 次のトークンに進む
                self.current_token = self.lexer.next();
            }
            _ => panic!("Expected '='"),
        }

        // 初期化式を解析（ここでは簡略化して識別子または数値リテラルのみを扱う）
        let init = self.parse_assignment_expression();

        // ';' を期待
        match &self.current_token {
            Some(lexer::Token::Punctuator(';')) => {
                // 次のトークンに進む
                self.current_token = self.lexer.next();
            }
            _ => panic!("Expected ';'"),
        }

        Node::new_variable_declaration(id, init)
    }

    pub fn parse_statement(&mut self) -> Node {
        match &self.current_token {
            Some(lexer::Token::Keyword(k)) => match k.as_str() {
                "const" => {
                    self.consume_next_token(); // consume 'const'
                    self.parse_variable_declaration()
                }
                _ => Node::new_string_literal("".to_string()), // ダミー
            },
            _ => Node::new_string_literal("".to_string()), // ダミー
        }
    }

    pub fn parse_source_element(&mut self) -> Option<Node> {
        if self.current_token.is_none() {
            return None;
        }

        let node = self.parse_statement();

        // 次のトークンに進む
        self.current_token = self.lexer.next();

        Some(node)
    }

    pub fn parse(&mut self) -> Program {
        let mut program = Program::new();

        let mut body = Vec::new();

        loop {
            let node = self.parse_source_element();

            match node {
                Some(n) => body.push(n),
                None => {
                    program.set_body(body);
                    return program;
                }
            }
        }
    }
}
