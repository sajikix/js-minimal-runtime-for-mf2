use crate::parser::Node;
use crate::parser::Program;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use core::fmt::Display;
use core::fmt::Formatter;
use core::ops::Add;
//

use crate::printer::{FormatValue, MF2Printer};
use mf2_parser::parser::Mf2Parser;

#[derive(Debug, Clone, PartialEq)]
pub struct MessageFormatInstance {
    locale: String,
    message: String,
}

impl MessageFormatInstance {
    fn new(locale: String, message: String) -> Self {
        Self { locale, message }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessageFormatMethod {
    instance: MessageFormatInstance,
    method: String,
}

impl MessageFormatMethod {
    fn new(instance: MessageFormatInstance, method: String) -> Self {
        Self { instance, method }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    /// https://262.ecma-international.org/#sec-numeric-types
    Number(u64),
    /// https://262.ecma-international.org/#sec-ecmascript-language-types-string-type
    StringLiteral(String),
    //
    MessageFormatInstance(MessageFormatInstance),
    //
    MessageFormatMethod(MessageFormatMethod),
    // Object type for JS objects
    Object(HashMap<String, RuntimeValue>),
}

impl Add<RuntimeValue> for RuntimeValue {
    type Output = RuntimeValue;

    fn add(self, rhs: RuntimeValue) -> RuntimeValue {
        if let (RuntimeValue::Number(left_num), RuntimeValue::Number(right_num)) = (&self, &rhs) {
            return RuntimeValue::Number(left_num + right_num);
        }
        // selfかrhsがMessageFormatInstanceの場合は、空文字を返す
        if let RuntimeValue::MessageFormatInstance(_) = &self {
            return RuntimeValue::StringLiteral("".to_string());
        }
        if let RuntimeValue::MessageFormatInstance(_) = &rhs {
            return RuntimeValue::StringLiteral("".to_string());
        }

        RuntimeValue::StringLiteral(self.to_string() + &rhs.to_string())
    }
}

impl Display for RuntimeValue {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        let s = match self {
            RuntimeValue::Number(value) => format!("{}", value),
            RuntimeValue::StringLiteral(value) => value.to_string(),
            RuntimeValue::MessageFormatInstance(_) => "[object Intl.MessageFormat]".to_string(),
            RuntimeValue::MessageFormatMethod(_) => "[object Intl.MessageFormatMethod]".to_string(),
            RuntimeValue::Object(_) => "[object Object]".to_string(),
        };
        write!(f, "{}", s)
    }
}

impl FormatValue for RuntimeValue {}

pub struct Environment {
    variables: HashMap<String, RuntimeValue>,
    outer: Option<Rc<RefCell<Environment>>>,
    nest: u32,
}

impl Environment {
    pub fn new(outer: Option<Rc<RefCell<Environment>>>) -> Self {
        let nest = outer.as_ref().map(|o| o.borrow().nest + 1).unwrap_or(0);
        Self {
            variables: HashMap::new(),
            outer,
            nest,
        }
    }

    pub fn define_var(&mut self, name: String, value: RuntimeValue) {
        self.variables.insert(name.clone(), value);
    }

    pub fn get_var(&self, name: &str) -> Option<RuntimeValue> {
        if let Some(value) = self.variables.get(name) {
            Some(value.clone())
        } else if let Some(outer) = &self.outer {
            outer.borrow().get_var(name)
        } else {
            None
        }
    }
}

pub struct Runtime {}
impl Runtime {
    pub fn new() -> Self {
        Self {}
    }
    pub fn execute(&mut self, program: Program) {
        let env = Rc::new(RefCell::new(Environment::new(None)));
        let mut result = None;
        for node in program.body {
            result = self.eval(Some(node), env.clone());
        }
        if let Some(res) = result {
            println!("> {}", res.to_string());
        }
    }

    pub fn eval(
        &mut self,
        _node: Option<Node>,
        env: Rc<RefCell<Environment>>,
    ) -> Option<RuntimeValue> {
        let node = match _node {
            Some(n) => n,
            None => return None,
        };
        match node {
            Node::ExpressionStatement { expression } => self.eval(Some(*expression), env.clone()),

            Node::VariableDeclaration { id, init } => {
                let var_name = match &*id {
                    Node::Identifier(name) => name.clone(),
                    _ => panic!("Expected identifier in variable declaration"),
                };
                let init_result = self.eval(Some(*init), env.clone());
                env.borrow_mut().define_var(
                    var_name,
                    init_result.unwrap_or(RuntimeValue::StringLiteral("undefined".to_string())),
                );
                None
            }
            Node::NewExpression { callee, arguments } => {
                let callee_result = self.eval(Some(*callee), env.clone());
                let mut args_result = Vec::new();
                for arg in arguments {
                    if let Some(arg_value) = self.eval(Some(arg), env.clone()) {
                        args_result.push(arg_value);
                    }
                }
                if callee_result
                    == Some(RuntimeValue::StringLiteral(
                        "Intl.MessageFormat".to_string(),
                    ))
                {
                    if args_result.len() == 2 {
                        if let (
                            RuntimeValue::StringLiteral(locale),
                            RuntimeValue::StringLiteral(message),
                        ) = (&args_result[0], &args_result[1])
                        {
                            return Some(RuntimeValue::MessageFormatInstance(
                                MessageFormatInstance::new(locale.clone(), message.clone()),
                            ));
                        }
                    }
                }
                None
            }

            Node::MemberExpression { object, property } => {
                let object_result = match self.eval(Some(*object), env.clone()) {
                    Some(value) => value,
                    None => return None,
                };
                let property_result = match self.eval(Some(*property), env.clone()) {
                    Some(value) => value,
                    None => return Some(object_result),
                };

                if let RuntimeValue::MessageFormatInstance(instance) = object_result {
                    return Some(RuntimeValue::MessageFormatMethod(MessageFormatMethod::new(
                        instance,
                        if let RuntimeValue::StringLiteral(method) = property_result {
                            method
                        } else {
                            panic!("Expected StringLiteral for method name");
                        },
                    )));
                }
                Some(object_result + RuntimeValue::StringLiteral(".".to_string()) + property_result)
            }
            Node::CallExpression { callee, arguments } => {
                let callee = match self.eval(Some(*callee), env.clone()) {
                    Some(value) => value,
                    None => return None,
                };

                let first_arg = arguments.get(0).unwrap().clone();

                // calleeがMessageFormatMethodの場合、対応するメソッドを呼び出す
                if let RuntimeValue::MessageFormatMethod(mf_method) = callee {
                    return Some(RuntimeValue::StringLiteral(
                        self.call_intl_message_format_method(mf_method, first_arg, env.clone()),
                    ));
                }
                None
            }
            Node::ObjectExpression { properties } => {
                let mut object = HashMap::new();
                for prop in properties {
                    if let Node::Property { key, value } = prop {
                        if let Some(val) = self.eval(Some(*value), env.clone()) {
                            object.insert(key, val);
                        }
                    }
                }
                Some(RuntimeValue::Object(object))
            }
            Node::Identifier(name) => {
                match env.borrow_mut().get_var(&name.to_string()) {
                    Some(v) => Some(v),
                    // 変数名が初めて使用される場合は、まだ値は保存されていないので、文字列として扱う
                    // たとえば、var a = 42; のようなコードの場合、aはStringLiteralとして扱われる
                    None => Some(RuntimeValue::StringLiteral(name.to_string())),
                }
            }
            Node::NumericLiteral(value) => Some(RuntimeValue::Number(value)),
            Node::StringLiteral(value) => Some(RuntimeValue::StringLiteral(value.clone())),
            _ => None,
        }
    }

    fn call_intl_message_format_method(
        &mut self,
        method: MessageFormatMethod,
        args: Node,
        env: Rc<RefCell<Environment>>,
    ) -> String {
        if method.method != "format" {
            return "".to_string(); // 未対応のメソッドは空文字を返す
        }
        // argsをevalして、変数マップを取得
        let variables = match self.eval(Some(args), env.clone()) {
            Some(RuntimeValue::Object(map)) => map,
            _ => HashMap::new(),
        };

        let mut mf2_parser = Mf2Parser::new(&method.instance.message);
        let ast = mf2_parser.parse();

        // ASTをvariablesを使ってフォーマット
        match ast {
            Ok(message) => MF2Printer::print(&message, &variables),
            _ => method.instance.message.to_string(),
        }
    }
}
