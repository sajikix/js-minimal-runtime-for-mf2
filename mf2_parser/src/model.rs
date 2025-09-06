use std::collections::HashMap;

// Reference: https://github.com/unicode-org/message-format-wg/tree/main/spec/data-model
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Pattern(PatternMessage),
    Select(SelectMessage),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternMessage {
    declarations: Vec<Declaration>,
    pub pattern: Vec<PatternItem>,
}

impl PatternMessage {
    pub fn new() -> Self {
        Self {
            declarations: Vec::new(),
            pattern: Vec::new(),
        }
    }
    pub fn set_declarations(&mut self, declarations: Vec<Declaration>) {
        self.declarations = declarations;
    }
    pub fn set_pattern(&mut self, pattern: Vec<PatternItem>) {
        self.pattern = pattern;
    }
    pub fn pattern(&self) -> &Vec<PatternItem> {
        &self.pattern
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectMessage {
    declarations: Vec<Declaration>,
    selectors: Vec<VariableRef>,
    variants: Vec<Variant>,
}

impl SelectMessage {
    pub fn new() -> Self {
        Self {
            declarations: Vec::new(),
            selectors: Vec::new(),
            variants: Vec::new(),
        }
    }
    pub fn set_declarations(&mut self, declarations: Vec<Declaration>) {
        self.declarations = declarations;
    }
    pub fn set_selectors(&mut self, selectors: Vec<VariableRef>) {
        self.selectors = selectors;
    }
    pub fn set_variants(&mut self, variants: Vec<Variant>) {
        self.variants = variants;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    keys: Vec<VariantKey>,
    value: Vec<PatternItem>,
}

impl Variant {
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            value: Vec::new(),
        }
    }
    pub fn set_keys(&mut self, keys: Vec<VariantKey>) {
        self.keys = keys;
    }
    pub fn set_value(&mut self, value: Vec<PatternItem>) {
        self.value = value;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatchallKey {
    value: Option<String>,
}

impl CatchallKey {
    pub fn new() -> Self {
        Self { value: None }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputDeclaration {
    name: String,
    value: VariableExpression,
}

impl InputDeclaration {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            value: VariableExpression::new(),
        }
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_value(&mut self, value: VariableExpression) {
        self.value = value;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDeclaration {
    name: String,
    value: Expression,
}

impl LocalDeclaration {
    pub fn new(value: Expression) -> Self {
        Self {
            name: String::new(),
            value,
        }
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Markup {
    name: String,
    pub kind: MarkupKind,
    options: HashMap<String, OptionValue>,
    attributes: HashMap<String, AttributeValue>,
}

impl Markup {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            kind: MarkupKind::Open,
            options: HashMap::new(),
            attributes: HashMap::new(),
        }
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_kind(&mut self, kind: MarkupKind) {
        self.kind = kind;
    }
    pub fn set_options(&mut self, options: HashMap<String, OptionValue>) {
        self.options = options;
    }
    pub fn set_attributes(&mut self, attributes: HashMap<String, AttributeValue>) {
        self.attributes = attributes;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    pub value: String,
}

impl Literal {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn normalize(&self) -> String {
        self.value.clone()
        // TODO: Implement normalization logic
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableRef {
    name: String,
}

impl VariableRef {
    pub fn new() -> Self {
        Self {
            name: String::new(),
        }
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralExpression {
    arg: Literal,
    function: Option<FunctionRef>,
    attributes: HashMap<String, AttributeValue>,
}

impl LiteralExpression {
    pub fn new() -> Self {
        Self {
            arg: Literal::new(String::new()),
            function: None,
            attributes: HashMap::new(),
        }
    }
    pub fn set_literal(&mut self, literal: Literal) {
        self.arg = literal;
    }
    pub fn set_function(&mut self, function: FunctionRef) {
        self.function = Some(function);
    }
    pub fn set_attributes(&mut self, attributes: HashMap<String, AttributeValue>) {
        self.attributes = attributes;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableExpression {
    arg: VariableRef,
    function: Option<FunctionRef>,
    attributes: HashMap<String, AttributeValue>,
}

impl VariableExpression {
    pub fn new() -> Self {
        Self {
            arg: VariableRef::new(),
            function: None,
            attributes: HashMap::new(),
        }
    }
    pub fn set_variable_ref(&mut self, variable_ref: VariableRef) {
        self.arg = variable_ref;
    }
    pub fn set_function(&mut self, function: FunctionRef) {
        self.function = Some(function);
    }
    pub fn set_attributes(&mut self, attributes: HashMap<String, AttributeValue>) {
        self.attributes = attributes;
    }
    pub fn get_variable_name(&self) -> String {
        self.arg.name.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionExpression {
    function: FunctionRef,
    attributes: HashMap<String, AttributeValue>,
}

impl FunctionExpression {
    pub fn new() -> Self {
        Self {
            function: FunctionRef::new(),
            attributes: HashMap::new(),
        }
    }
    pub fn set_function(&mut self, function: FunctionRef) {
        self.function = function;
    }
    pub fn set_attributes(&mut self, attributes: HashMap<String, AttributeValue>) {
        self.attributes = attributes;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionRef {
    pub name: String,
    options: HashMap<String, OptionValue>,
}

impl FunctionRef {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            options: HashMap::new(),
        }
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_options(&mut self, options: HashMap<String, OptionValue>) {
        self.options = options;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionValue {
    Literal(Literal),
    VariableRef(VariableRef),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariantKey {
    Literal(Literal),
    CatchallKey(CatchallKey),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarkupKind {
    Open,
    StandAlone,
    Close,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeValue {
    Literal(String),
    True(bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
    Input(InputDeclaration),
    Local(LocalDeclaration),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternItem {
    String(String),
    Expression(Expression),
    Markup(Markup),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Literal(LiteralExpression),
    Variable(VariableExpression),
    Function(FunctionExpression),
}
