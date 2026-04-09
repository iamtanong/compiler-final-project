// Type codes
const LINE_CODE: u32 = 10;
const ID_CODE: u32 = 11;
const CONST_CODE: u32 = 12;
const IF_CODE: u32 = 13;
const GOTO_CODE: u32 = 14;
const PRINT_CODE: u32 = 15;
const STOP_CODE: u32 = 16;
const OP_CODE: u32 = 17;

// Operator codes
const OP_PLUS: u32 = 1;
const OP_MINUS: u32 = 2;
const OP_LT: u32 = 3;
const OP_EQ: u32 = 4;

/// Code generator for B-code
///
/// Produces B-code output as pairs of (type, value)
pub struct CodeGen {
    output: Vec<(u32, u32)>,
}

impl CodeGen {
    pub fn new() -> Self {
        CodeGen { output: Vec::new() }
    }

    pub fn emit_line(&mut self, line_num: u32) {
        self.output.push((LINE_CODE, line_num));
    }

    pub fn emit_id(&mut self, id: char) {
        let ref_num = (id as u32 - 'A' as u32) + 1; // A=1, B=2, ..., Z=26
        self.output.push((ID_CODE, ref_num));
    }

    pub fn emit_const(&mut self, value: u32) {
        self.output.push((CONST_CODE, value));
    }

    pub fn emit_if(&mut self) {
        self.output.push((IF_CODE, 0));
    }

    pub fn emit_goto(&mut self, target: u32) {
        self.output.push((GOTO_CODE, target));
    }

    pub fn emit_print(&mut self) {
        self.output.push((PRINT_CODE, 0));
    }

    pub fn emit_stop(&mut self) {
        self.output.push((STOP_CODE, 0));
    }

    pub fn emit_op(&mut self, op: char) {
        let op_code = match op {
            '+' => OP_PLUS,
            '-' => OP_MINUS,
            '<' => OP_LT,
            '=' => OP_EQ,
            _ => panic!("Invalid operator: {}", op),
        };
        self.output.push((OP_CODE, op_code));
    }

    pub fn format_output(&self) -> String {
        let mut result = String::new();
        for (type_code, value) in &self.output {
            result.push_str(&format!("{} {} ", type_code, value));
        }
        result
    }
}
