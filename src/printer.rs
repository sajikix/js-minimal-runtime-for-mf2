use mf2_parser::model::{Expression, Message, PatternItem, PatternMessage};
use std::collections::HashMap;

/// MessageFormat v2のフォーマット処理を行うトレイト
pub trait FormatValue: ToString + Clone {}

/// MessageFormat v2のフォーマッター
pub struct MF2Printer;

impl MF2Printer {
    /// MessageFormat v2のパターンメッセージをフォーマット
    pub fn format_pattern<T: FormatValue>(
        pattern_msg: &PatternMessage,
        variables: &HashMap<String, T>,
    ) -> String {
        let mut result = String::new();

        for item in pattern_msg.pattern.iter() {
            match item {
                PatternItem::String(s) => {
                    result.push_str(s);
                }
                PatternItem::Expression(Expression::Variable(var_expr)) => {
                    let var_name = var_expr.get_variable_name();
                    if let Some(value) = variables.get(&var_name) {
                        result.push_str(&value.to_string());
                    } else {
                        // 変数が見つからない場合は、{$name}の形式で出力
                        result.push_str(&format!("{{${}}}", var_name));
                    }
                }
                PatternItem::Expression(Expression::Literal(_lit_expr)) => {
                    // リテラル式の処理（今後の拡張用）
                    result.push_str("[literal]");
                }
                PatternItem::Expression(Expression::Function(_func_expr)) => {
                    // 関数式の処理（今後の拡張用）
                    result.push_str("[function]");
                }
                PatternItem::Markup(_) => {
                    // マークアップの処理（今後の拡張用）
                    result.push_str("[markup]");
                }
            }
        }

        result
    }

    /// MessageFormat v2のメッセージ全体をprint
    pub fn print<T: FormatValue>(message: &Message, variables: &HashMap<String, T>) -> String {
        match message {
            Message::Pattern(pattern_msg) => Self::format_pattern(pattern_msg, variables),
            Message::Select(_select_msg) => {
                // SelectMessageの処理は今後実装
                "[select message not implemented]".to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl FormatValue for String {}

    #[test]
    fn test_format_simple_pattern() {
        // テストケースは必要に応じて追加
    }
}
