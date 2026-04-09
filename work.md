# Retro BASIC Compiler – Rust Implementation

A compiler that translates **Retro BASIC** source programs into **B-code bytecode**.

## Overview

This project implements a complete compiler for the simplified BASIC language specification in Rust. It features:

- **Lexer**: Token stream generation using `nom` parser combinators
- **Parser**: Recursive descent parser with direct B-code emission
- **Code Generator**: Direct bytecode output in the B-code format
- **Clean Pipeline**: Source → Tokens → B-code (single-pass compilation)

## Quick Start

### Build

```bash
cargo build --release
```

Binary: `target/release/retro_basic_compiler`

### Usage

**Compile to file:**

```bash
./target/release/retro_basic_compiler program.bas output.bc
```

**Print to stdout:**

```bash
./target/release/retro_basic_compiler program.bas
```

### Verify Output

Use the provided `lister` disassembler to verify compilation:

```bash
./lister/lister output.bc
```

## Language Features

### Statements

- **Assignment**: `A = 1`, `X = Y + 5`
- **Conditional**: `IF 10 < A 60` (jumps to line 60 if condition true)
- **Print**: `PRINT A` (output variable)
- **Goto**: `GOTO 50` (unconditional branch)
- **Stop**: `STOP` (program termination)

### Operators

- **Arithmetic**: `+`, `-` (binary operations)
- **Comparison**: `<`, `=` (for conditions)

### Data Types

- **Identifiers**: Single uppercase letters (A–Z)
- **Numbers**: Unsigned integers (0–1000)
- **Line numbers**: 1–1000

## Example Programs

### print1-10.bas – Loop Example

```basic
10 A = 1
20 IF 10 < A 60
30 PRINT A
40 A = A + 1
50 GOTO 20
60 STOP
```

Compiles to B-code, which disassembles to the same program structure.

### sum1-10.bas – Accumulator Example

```basic
10 A = 1
20 S = 0
30 IF 10 < A 70
40 S = S + A
50 A = A + 1
60 GOTO 30
70 PRINT S
80 STOP
```

## Project Structure

```
src/
  main.rs       – CLI entry point, file I/O orchestration
  lexer.rs      – Tokenization (nom-based)
  parser.rs     – Parsing and code generation (recursive descent)
  codegen.rs    – B-code emission and formatting

tests/
  print1-10.bas – Test program: print 1 to 10
  sum1-10.bas   – Test program: sum 1 to 10

lister/
  lister        – Disassembler (verify output)

COMPILER_REPORT.md – Technical documentation (3+ pages)
```

## Technical Highlights

### Architecture

- **Three-stage pipeline**: Lexer → Parser → CodeGen
- **Direct code generation**: No intermediate AST; bytecode emitted during parsing
- **Single-pass compilation**: Efficient, linear time complexity

### Tools & Libraries

- **Language**: Rust (Edition 2021)
- **Parser library**: `nom` v7 (parser combinators)
- **Build system**: Cargo

### B-code Format

Output is space-separated token pairs (type, value):

```
10 10 11 1 17 4 12 1 10 20 13 0 12 10 17 3 11 1 14 60 ...
```

Where type codes are:

- 10 = line number
- 11 = identifier (1–26 for A–Z)
- 12 = constant
- 13 = IF marker
- 14 = GOTO target
- 15 = PRINT
- 16 = STOP
- 17 = operator (1=+, 2=-, 3=<, 4==)

## Documentation

For detailed compiler architecture, design decisions, data structures, and examples:
**See [COMPILER_REPORT.md](COMPILER_REPORT.md)** (3–4 pages)

## Testing

### Quick Test

```bash
# Compile test program
./target/release/retro_basic_compiler tests/print1-10.bas /tmp/out.bc

# Verify with lister
./lister/lister /tmp/out.bc
```

### Expected Output from Lister

```
10 A = 1
20 IF 10 < A GOTO 60
30 PRINT A
40 A = A + 1
50 GOTO 20
60 STOP
```

## Error Handling

The compiler provides clear error messages for:

- **Syntax errors** (missing token, unexpected token)
- **File I/O errors** (file not found, permission denied)
- **Invalid characters** (unexpected input)

Example:

```
Error: Expected identifier, got LineNum(123)
```

## Future Extensions

Possible enhancements:

- Multi-character variable names
- Additional operators (\*, /, MOD)
- Array support
- String handling
- Functions and subroutines
- Optimization passes

---

**Version**: 0.1.0  
**Compiler Date**: April 2026  
**Status**: Complete & tested
