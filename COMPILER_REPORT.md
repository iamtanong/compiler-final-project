# Retro BASIC to B-code Compiler: Technical Report

## Executive Summary

This report documents a Retro BASIC compiler written in Rust that translates source programs into B-code bytecode. The compiler uses a three-stage pipeline architecture: **lexical analysis** (tokenization), **syntactic analysis** (parsing), and **code generation**. The implementation employs the `nom` parser combinator library for lexing, and a hand-written recursive descent parser for parsing and direct B-code emission.

---

## 1. Introduction & Objectives

### Purpose

The compiler translates Retro BASIC programs into intermediate bytecode (B-code) suitable for interpretation or further compilation. B-code is a low-level token-based representation that preserves the semantic structure of the source program.

### Language Specification

Retro BASIC is a simplified BASIC dialect featuring:

- **Program structure**: Lines with line numbers followed by statements
- **Statements**: assignment, conditional (IF), output (PRINT), control flow (GOTO), and termination (STOP)
- **Expressions**: binary operations (+, -) on identifiers and constants
- **Conditions**: binary comparisons (<, =) on identifiers and constants
- **Identifiers**: single uppercase letters A–Z
- **Constants & line numbers**: unsigned integers (0–1000)

### Design Goals

1. **Correctness**: Accurately translate all valid Retro BASIC programs
2. **Clarity**: Transparent pipeline from source to bytecode
3. **Maintainability**: Clean separation of concerns (lexing, parsing, code generation)
4. **Performance**: Efficient, single-pass compilation

---

## 2. Compiler Architecture

### Pipeline Overview

```
Source Input (Retro BASIC)
       ↓
   [LEXER] → Tokenization using nom
       ↓
    Token Stream
       ↓
   [PARSER] → Recursive Descent Parsing
       ↓
Direct B-code Emission via CodeGen
       ↓
   B-code Output (space-separated pairs)
```

### Module Structure

The compiler is organized into three core modules:

#### 2.1 **lexer.rs** – Lexical Analysis

- **Function**: `pub fn tokenize(input: &str) -> Result<Vec<Token>, String>`
- **Responsibility**: Convert source text into a sequence of tokens
- **Technology**: Uses `nom` parser combinator library for pattern matching and token recognition
- **Token types**: LineNum(u32), Id(char), If, Goto, Print, Stop, Plus, Minus, Less, Assign

**Key Algorithm**:

1. Skip whitespace and newlines
2. Attempt to parse line numbers (digit sequences)
3. Attempt to parse identifiers and keywords (A–Z prefix with lookahead)
4. Attempt to parse operators (+, -, <, =)
5. Return descriptive error for unrecognized characters

**Keyword Recognition**: Uses lookahead to distinguish single-letter identifiers from multi-letter keywords:

- "IF" → check if followed by 'F'
- "GOTO" → check if followed by "OTO"
- "PRINT" → check if followed by "RINT"
- "STOP" → check if followed by "TOP"

#### 2.2 **parser.rs** – Syntactic Analysis

- **Type**: `pub struct Parser { tokens, pos, codegen }`
- **Methods**: Recursive descent parsing with direct code emission
- **Responsibility**: Validate syntax and emit B-code during parsing

**Key Parsing Functions**:

- `parse_program()`: Entry point; iterates through lines until EOF
- `parse_line()`: Expects line_number + statement
- `parse_statement()`: Dispatches to statement-specific parsers
  - `parse_assignment()`: id "=" expression
  - `parse_if()`: "IF" condition target_line_number
  - `parse_print()`: "PRINT" identifier
  - `parse_goto()`: "GOTO" line_number
  - `parse_stop()`: "STOP"
- `parse_expression()`: term [(+|-) term]\*
- `parse_condition()`: term [(<|=) term]

**Design Pattern**: Direct code generation—rather than building an Abstract Syntax Tree (AST), the parser immediately emits B-code tuples via `codegen` as it parses. This single-pass approach is efficient and eliminates tree construction overhead.

#### 2.3 **codegen.rs** – Code Generation

- **Type**: `pub struct CodeGen { output: Vec<(u32, u32)> }`
- **Methods**: Emit functions for each B-code type
- **Responsibility**: Accumulate and format B-code output

**Emission Functions**:

- `emit_line(line_num)` → (10, line_num)
- `emit_id(char)` → (11, 1..26) [A=1, B=2, ..., Z=26]
- `emit_const(value)` → (12, value)
- `emit_if()` → (13, 0)
- `emit_goto(target)` → (14, target)
- `emit_print()` → (15, 0)
- `emit_stop()` → (16, 0)
- `emit_op(char)` → (17, op_code) where op_code ∈ {1(+), 2(-), 3(<), 4(=)}

**Output Format**: Space-separated token pairs (type value type value ...)

---

## 3. Key Design Decisions

### 3.1 Direct Code Generation vs. AST

**Decision**: Emit B-code directly during parsing, with no intermediate AST.

**Rationale**:

- Simpler, more concise implementation
- Single-pass compilation reduces memory usage
- For Retro BASIC's simple grammar, no tree structure benefits are needed
- Direct generation is sufficiently clear for maintenance

### 3.2 Parser Combinators (nom) for Lexing

**Decision**: Use `nom` library instead of manual state machine.

**Rationale**:

- Declarative, readable parser definitions
- Built-in error handling and composition
- Lookahead support for keyword vs. identifier disambiguation
- Standard Rust idiom in parser ecosystems

### 3.3 Recursive Descent for Parsing

**Decision**: Hand-written recursive descent parser (not parser generator).

**Rationale**:

- Retro BASIC grammar is simple and LL(1)-like
- Direct control over code generation interleaving
- Easy to debug and extend
- Good error messages tailored to language

### 3.4 Token-Centric Approach

**Decision**: First tokenize fully, then parse separately (two-phase).

**Rationale**:

- Clean separation simplifies testing
- Easier error reporting (token stream is stable for replay)
- Parser doesn't need to interact with character-level logic

---

## 4. Data Structures

### Token Enum

```rust
pub enum Token {
    LineNum(u32),  // Line number
    Id(char),      // Identifier A-Z
    If, Goto, Print, Stop,  // Keywords
    Plus, Minus, Less, Assign,  // Operators
}
```

### CodeGen Structure

```rust
pub struct CodeGen {
    output: Vec<(u32, u32)>,  // B-code pairs (type, value)
}
```

### Parser Structure

```rust
pub struct Parser {
    tokens: Vec<Token>,  // Token stream
    pos: usize,          // Current position
    codegen: CodeGen,    // Accumulates B-code
}
```

---

## 5. Example Compilation Walkthrough

### Input Program: "Print 1 to 10"

```
10 A = 1
20 IF 10 < A 60
30 PRINT A
40 A = A + 1
50 GOTO 20
60 STOP
```

### Tokenization Phase

Input: "10 A = 1 20 IF 10 < A 60 ..."

Tokens produced:

```
[LineNum(10), Id('A'), Assign, LineNum(1),
 LineNum(20), If, LineNum(10), Less, Id('A'), LineNum(60),
 LineNum(30), Print, Id('A'),
 LineNum(40), Id('A'), Assign, Id('A'), Plus, LineNum(1),
 LineNum(50), Goto, LineNum(20),
 LineNum(60), Stop]
```

### Parsing & B-code Generation

**Line 1: `10 A = 1`**

- Parser recognizes: LineNum(10) → emits (10, 10)
- Dispatches to parse_assignment
- Sees Id('A') → emits (11, 1)
- Expects Assign → emits (17, 4) [assignment operator]
- Parses expression: LineNum(1) → emits (12, 1)

B-code so far: `(10,10) (11,1) (17,4) (12,1)`

**Line 2: `20 IF 10 < A 60`**

- LineNum(20) → emits (10, 20)
- Dispatches to parse_if
- emit_if() → (13, 0)
- parse_condition:
  - LineNum(10) → emits (12, 10)
  - Less → emits (17, 3)
  - Id('A') → emits (11, 1)
- LineNum(60) (target) → emits (14, 60)

B-code: `... (10,20) (13,0) (12,10) (17,3) (11,1) (14,60)`

**Continue similarly for remaining lines...**

### Final B-code Output

```
10 10 11 1 17 4 12 1
10 20 13 0 12 10 17 3 11 1 14 60
10 30 15 0 11 1
10 40 11 1 17 4 11 1 17 1 12 1
10 50 14 20
10 60 16 0
```

(Note: Output is space-separated on one line per implementation, or one pair per line depending on formatter preference.)

---

## 6. Implementation Highlights

### 6.1 Identifier Encoding

Variables (A–Z) are encoded as integers 1–26 in B-code:

```rust
let ref_num = (id as u32 - 'A' as u32) + 1;
```

This maps 'A' → 1, 'B' → 2, ..., 'Z' → 26.

### 6.2 Keyword vs. Identifier Disambiguation

The lexer uses lookahead to distinguish keywords from identifiers:

```rust
'I' => {
    if input.starts_with('F') {
        Ok((&input[1..], Token::If))  // "IF" detected
    } else {
        Ok((input, Token::Id('I')))   // Just "I"
    }
}
```

### 6.3 Error Handling

- **Lexer**: Returns descriptive error on unexpected character
- **Parser**: Returns error with expected vs. found token
- **File I/O**: Graceful error messages for read/write failures

### 6.4 Newline Handling

The lexer explicitly handles newline characters to support multi-line programs:

```rust
if let Some(r) = rest.strip_prefix('\n') {
    remaining = r;
    continue;  // Skip the newline and continue tokenizing
}
```

---

## 7. Usage & Compilation

### Building the Release Binary

```bash
cd /path/to/project
cargo build --release
```

Output binary: `target/release/retro_basic_compiler`

### Running the Compiler

**Option 1: Write to output file**

```bash
./target/release/retro_basic_compiler input.bas output.bc
```

**Option 2: Print to stdout**

```bash
./target/release/retro_basic_compiler input.bas
```

### Verification with Lister

To verify the B-code output, use the provided `lister` disassembler:

```bash
./lister/lister output.bc
```

This converts B-code back into readable format, confirming correct compilation.

---

## 8. Testing & Validation

### Unit Test Coverage

- Lexer tokenization of keywords, identifiers, operators, numbers
- Parser handling of all statement types
- Code generation matching specification

### Integration Tests

Included test programs:

- `tests/print1-10.bas`: Loop printing numbers 1–10
- `tests/sum1-10.bas`: Accumulator pattern summing 1–10

### Validation Method

1. Compile source program to B-code
2. Disassemble with `lister` tool
3. Verify output semantically matches source

---

## 9. Limitations & Future Improvements

### Current Limitations

1. **No standard library**: Math functions, string handling unsupported
2. **Single-character identifiers**: Only A–Z supported (design spec)
3. **No optimization**: Direct emission; no peephole optimizations
4. **Error recovery**: Stops at first error (no recovery)

### Possible Extensions

1. Multi-character variable names
2. Additional operators (\*, /, MOD)
3. Array support
4. String handling
5. Function definitions
6. Scope analysis / symbol table

---

## 10. Conclusion

This Retro BASIC compiler demonstrates a clean, modular approach to language implementation using Rust and the `nom` parser combinator library. The three-stage pipeline (lexing → parsing → code generation) with direct B-code emission achieves simplicity and efficiency for this domain. The compiler successfully translates Retro BASIC programs into B-code bytecode conforming to the specification, as verified by the provided `lister` disassembler tool.

The architecture prioritizes:

- **Clarity** through separation of concerns
- **Correctness** via structured parsing and error checking
- **Maintainability** through clear data structures and modular functions
- **Performance** via single-pass compilation

---

## Appendix: B-code Type Reference

| Type Code | Token Type | Value Meaning                         |
| --------- | ---------- | ------------------------------------- |
| 10        | t_line     | Line number (1–1000)                  |
| 11        | t_id       | Variable ID (1–26 for A–Z)            |
| 12        | t_const    | Numeric constant                      |
| 13        | t_if       | IF statement marker (always 0)        |
| 14        | t_goto     | GOTO target line number               |
| 15        | t_print    | PRINT marker (always 0)               |
| 16        | t_stop     | STOP marker (always 0)                |
| 17        | t_op       | Operator code: 1(+), 2(-), 3(<), 4(=) |

---

**Report compiled:** April 2026  
**Compiler version:** 0.1.0  
**Language:** Rust (Edition 2021)  
**Dependencies:** nom v7
