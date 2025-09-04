use std::iter::Peekable;

use crate::lexer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Program {
        body: Vec<Node>,
    },
    VariableDeclaration {
        id: Box<Node>,
        init: Box<Node>,
    },
    ExpressionStatement {
        expression: Box<Node>,
    },
    AssignmentExpression {
        operator: String,
        left: Box<Node>,
        right: Box<Node>,
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
    ObjectExpression {
        properties: Vec<Node>,
    },
    Property {
        key: String,
        value: Box<Node>,
    },
    Identifier(String),
    StringLiteral(String),
    NumericLiteral(u64),
}

impl Node {
    pub fn new_program(body: Vec<Node>) -> Self {
        Node::Program { body }
    }
    pub fn new_variable_declaration(id: Node, init: Node) -> Self {
        Node::VariableDeclaration {
            id: Box::new(id),
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
    pub fn new_call_expression(callee: Node, arguments: Vec<Node>) -> Self {
        Node::CallExpression {
            callee: Box::new(callee),
            arguments,
        }
    }
    pub fn new_expression_statement(expression: Node) -> Self {
        Node::ExpressionStatement {
            expression: Box::new(expression),
        }
    }
    pub fn new_assignment_expression(operator: String, left: Node, right: Node) -> Self {
        Node::AssignmentExpression {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
    pub fn new_identifier(name: String) -> Self {
        Node::Identifier(name)
    }
    pub fn new_property(key: String, value: Node) -> Self {
        Node::Property {
            key,
            value: Box::new(value),
        }
    }
}

pub struct Parser {
    lexer: Peekable<lexer::Lexer>,
}

impl Parser {
    pub fn new(src: &str) -> Self {
        let lexer = lexer::Lexer::new(src).peekable();
        Self { lexer }
    }

    fn parse_literal(&mut self) -> Option<Node> {
        let t = match self.lexer.next() {
            Some(token) => token,
            None => return None,
        };
        match t {
            lexer::Token::StringLiteral(s) => Some(Node::new_string_literal(s)),
            lexer::Token::Number(n) => Some(Node::NumericLiteral(n)),
            _ => None,
        }
    }

    fn parse_object_expression(&mut self) -> Node {
        let mut properties = Vec::new();

        loop {
            let t = match self.lexer.peek() {
                Some(token) => token.clone(),
                None => break,
            };
            match t {
                lexer::Token::Identifier(name) => {
                    self.lexer.next(); // consume identifier
                    let key = name.clone();
                    let t = match self.lexer.next() {
                        Some(token) => token,
                        None => panic!("Unexpected end of input after object key"),
                    };
                    match t {
                        lexer::Token::Punctuator(':') => {
                            let value = match self.parse_literal() {
                                Some(v) => v,
                                None => {
                                    panic!("Expected primitive value after ':' in object property")
                                }
                            };
                            properties.push(Node::new_property(key, value));
                            let t = match self.lexer.peek() {
                                Some(token) => token.clone(),
                                None => break,
                            };
                            match t {
                                lexer::Token::Punctuator(',') => {
                                    self.lexer.next(); // consume ','
                                }
                                lexer::Token::Punctuator('}') => {
                                    self.lexer.next(); // consume '}'
                                    break;
                                }
                                _ => panic!("Unexpected token in object expression: {:?}", t),
                            }
                        }
                        _ => panic!("Expected ':' after object key"),
                    }
                }
                lexer::Token::Punctuator('}') => {
                    self.lexer.next(); // consume '}'
                    break;
                }
                _ => panic!("Unexpected token in object expression: {:?}", t),
            }
        }
        Node::ObjectExpression { properties }
    }

    fn parse_arguments(&mut self) -> Vec<Node> {
        let mut args = Vec::new();

        loop {
            let t = match self.lexer.peek() {
                Some(token) => token.clone(),
                None => break,
            };
            match t {
                lexer::Token::StringLiteral(s) => {
                    args.push(Node::new_string_literal(s.clone()));
                    self.lexer.next(); // consume string literal
                }
                lexer::Token::Punctuator('{') => {
                    self.lexer.next(); // consume '{' 
                    args.push(self.parse_object_expression());
                }
                lexer::Token::Punctuator(',') => {
                    self.lexer.next(); // consume ','
                }
                lexer::Token::Punctuator(')') => {
                    self.lexer.next(); // consume ')'
                    break;
                }
                _ => panic!("Unexpected token in arguments: {:?}", t),
            }
        }
        args
    }

    fn parse_call_expression(&mut self) -> Option<Node> {
        let callee = self.parse_member_expression();
        let t = match self.lexer.peek() {
            Some(token) => token.clone(),
            None => return None,
        };
        match t {
            lexer::Token::Punctuator('(') => {
                self.lexer.next(); // consume '('
                let args = self.parse_arguments();
                Some(Node::new_call_expression(callee, args))
            }
            _ => panic!("Expected '(' after callee in call expression"),
        }
    }

    fn parse_member_expression(&mut self) -> Node {
        let mut members = Vec::new();

        loop {
            let t = match self.lexer.next() {
                Some(token) => token,
                None => panic!("Unexpected end of input while parsing member expression"),
            };
            match t {
                lexer::Token::Identifier(name) => {
                    let property = name.clone();
                    println!("Found identifier in member expression: {}", property);
                    members.push(property);
                    match &self.lexer.peek() {
                        Some(lexer::Token::Punctuator('.')) => {
                            self.lexer.next(); // consume '.'
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
        let t = match self.lexer.peek() {
            Some(token) => token,
            None => panic!("Unexpected end of input after 'new' callee"),
        };
        match t {
            lexer::Token::Punctuator('(') => {
                self.lexer.next(); // consume '('
                let args = self.parse_arguments();
                Node::new_new_expression(callee, args)
            }
            _ => panic!("Expected '(' after callee in new expression"),
        }
    }

    fn parse_left_hand_side_expression(&mut self) -> Option<Node> {
        let t = match self.lexer.peek() {
            Some(token) => token,
            None => return None,
        };
        match t {
            lexer::Token::Keyword(k) => match k.as_str() {
                "new" => {
                    println!("Parsing 'new' expression");
                    self.lexer.next(); // consume 'new'
                    return Some(self.parse_new_expression());
                }
                _ => panic!("Unexpected keyword: {}", k),
            },
            lexer::Token::Identifier(_) => self.parse_call_expression(),
            _ => self.parse_left_hand_side_expression(),
        }
    }

    pub fn parse_assignment_expression(&mut self) -> Option<Node> {
        let expr = self.parse_left_hand_side_expression();
        let t = match self.lexer.peek() {
            Some(token) => token,
            None => return expr,
        };
        println!("Peeked token in assignment expression: {:?}", t);
        match t {
            lexer::Token::Punctuator('=') => {
                self.lexer.next();
                // TODO: unwrapをなんとかする
                Some(Node::new_assignment_expression(
                    "=".to_string(),
                    expr.unwrap(),
                    self.parse_assignment_expression().unwrap(),
                ))
            }
            _ => expr,
        }
    }

    fn parse_identifier(&mut self) -> Option<Node> {
        let t = match self.lexer.next() {
            Some(token) => token,
            None => return None,
        };
        match t {
            lexer::Token::Identifier(name) => Some(Node::new_identifier(name)),
            _ => None,
        }
    }

    pub fn parse_initializer(&mut self) -> Option<Node> {
        let t = match self.lexer.next() {
            Some(token) => token,
            None => return None,
        };
        match t {
            lexer::Token::Punctuator(c) => match c {
                '=' => self.parse_assignment_expression(),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn parse_variable_declaration(&mut self) -> Node {
        // TODO: unwrapをなんとかする
        let id = self.parse_identifier().unwrap();
        // TODO: unwrapをなんとかする
        let init = self.parse_initializer().unwrap();
        Node::new_variable_declaration(id, init)
    }

    pub fn parse_statement(&mut self) -> Option<Node> {
        let t = match self.lexer.peek() {
            Some(t) => t,
            None => return None,
        };
        let node = match t {
            lexer::Token::Keyword(k) => match k.as_str() {
                "const" => {
                    self.lexer.next(); // consume 'const'
                    Some(self.parse_variable_declaration())
                }
                _ => Some(Node::new_string_literal("".to_string())), // ダミー
            },
            _ => Some(Node::new_expression_statement(
                self.parse_assignment_expression().unwrap(),
            )),
        };
        if let Some(lexer::Token::Punctuator(c)) = self.lexer.peek() {
            // ';'を消費する
            if c == &';' {
                self.lexer.next();
            }
        }
        node
    }

    pub fn parse(&mut self) -> Node {
        let mut body = Vec::new();
        loop {
            let node = self.parse_statement();

            match node {
                Some(n) => body.push(n),
                None => {
                    return Node::new_program(body.clone());
                }
            }
        }
    }
}
