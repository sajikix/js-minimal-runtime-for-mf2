use crate::parser::Program;

pub struct Runtime {}
impl Runtime {
    pub fn new() -> Self {
        Self {}
    }
    pub fn execute(&self, program: Program) {
        for node in program.body {
            println!("{:?}", node);
            // 実行ロジックをここに実装
        }
    }
}
