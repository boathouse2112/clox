#include <printf.h>
#include <stdlib.h>
#include "common.h"
#include "compiler.h"
#include "scanner.h"
#include "chunk.h"

#ifdef DEBUG_PRINT_CODE
#include "debug.h"
#endif

typedef struct {
    Token current;
    Token previous;
    bool had_error;
    bool panic_mode;
} Parser;

typedef enum {
    PRECEDENCE_NONE,
    PRECEDENCE_ASSIGNMENT,
    PRECEDENCE_OR,
    PRECEDENCE_AND,
    PRECEDENCE_EQUALITY,
    PRECEDENCE_COMPARISON,
    PRECEDENCE_TERM,
    PRECEDENCE_FACTOR,
    PRECEDENCE_UNARY,
    PRECEDENCE_CALL,
    PRECEDENCE_PRIMARY
} Precedence;

typedef void (*ParseFn)();

typedef struct {
    ParseFn prefix;
    ParseFn infix;
    Precedence precedence;
} ParseRule;

Parser parser;
Chunk *compiling_chunk;

static Chunk *current_chunk() {
    return compiling_chunk;
}

static void error_at(Token *token, const char *message) {
    if (parser.panic_mode) return;
    fprintf(stderr, "[line %d] Error", token->line);
    if (token->type == TOKEN_EOF) {
        fprintf(stderr, " at end");
    } else if (token->type == TOKEN_ERROR) {
    // Nothing.
    } else {
        fprintf(stderr, " at '%.*s'", token->length, token->start);
    }
    fprintf(stderr, ": %s\n", message);
    parser.had_error = true;
}

static void error(const char *message) {
    error_at(&parser.previous, message);
}

static void error_at_current(const char *message) {
    error_at(&parser.current, message);
}

static void advance() {
    parser.previous = parser.current;

    while (true) {
        parser.current = scanner_scan_token();
        if (parser.current.type != TOKEN_ERROR) break;

        error_at_current(parser.current.start);
    }
}

static void consume(TokenType type, const char *message) {
    if (parser.current.type == type) {
        advance();
        return;
    } else {
        error_at_current(message);
    }
}

static void emit_byte(uint8_t byte) {
    chunk_push(current_chunk(), byte, parser.previous.line);
}

//static void emit_bytes(uint8_t byte_1, uint8_t byte_2) {
//    emit_byte(byte_1);
//    emit_byte(byte_2);
//}

static void emit_constant(Value value) {
    chunk_push_constant(current_chunk(), value, parser.previous.line);
}

static void emit_return() {
    emit_byte(OP_RETURN);

#ifdef DEBUG_PRINT_CODE
    if (!parser.had_error) {
        disassemble_chunk(current_chunk(), "code");
    }
#endif
}

static void end_compiler() {
    emit_return();
}

static void expression();
static ParseRule *get_rule(TokenType type);
static void parse_precedence(Precedence precedence);

static void expression() {
    parse_precedence(PRECEDENCE_ASSIGNMENT);
}

static void number() {
    double value = strtod(parser.previous.start, NULL);
    emit_constant(value);
}

static void unary() {
    TokenType op_type = parser.previous.type;

    // Compile the operand.
    parse_precedence(PRECEDENCE_UNARY);

    // Emit the op instruction
    switch (op_type) {
        case TOKEN_MINUS:
            emit_byte(OP_NEGATE);
            break;
        default:
            return;
    }
}

static void binary() {
    TokenType op_type = parser.previous.type;
    ParseRule *rule = get_rule(op_type);
    parse_precedence((Precedence)(rule->precedence + 1));

    switch (op_type) {
        case TOKEN_PLUS:
            emit_byte(OP_ADD);
            return;
        case TOKEN_MINUS:
            emit_byte(OP_SUBTRACT);
            return;
        case TOKEN_STAR:
            emit_byte(OP_MULTIPLY);
            return;
        case TOKEN_SLASH:
            emit_byte(OP_DIVIDE);
            return;
        default: return;
    }
}

static void grouping() {
    // Assume '(' has already been consumed
    expression();
    consume(TOKEN_RIGHT_PAREN, "Expect ')' after expression.");
}

ParseRule rules[] = {
        [TOKEN_LEFT_PAREN] = {grouping, NULL, PRECEDENCE_NONE},
        [TOKEN_RIGHT_PAREN] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_LEFT_BRACE] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_RIGHT_BRACE] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_COMMA] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_DOT] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_MINUS] = {unary, binary, PRECEDENCE_TERM},
        [TOKEN_PLUS] = {NULL, binary, PRECEDENCE_TERM},
        [TOKEN_SEMICOLON] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_SLASH] = {NULL, binary, PRECEDENCE_FACTOR},
        [TOKEN_STAR] = {NULL, binary, PRECEDENCE_FACTOR},
        [TOKEN_BANG] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_BANG_EQUAL] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_EQUAL] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_EQUAL_EQUAL] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_GREATER] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_GREATER_EQUAL] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_LESS] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_LESS_EQUAL] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_IDENTIFIER] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_STRING] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_NUMBER] = {number, NULL, PRECEDENCE_NONE},
        [TOKEN_AND] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_CLASS] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_ELSE] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_FALSE] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_FOR] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_FUN] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_IF] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_NIL] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_OR] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_PRINT] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_RETURN] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_SUPER] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_THIS] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_TRUE] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_VAR] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_WHILE] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_ERROR] = {NULL, NULL, PRECEDENCE_NONE},
        [TOKEN_EOF] = {NULL, NULL, PRECEDENCE_NONE},
};

static void parse_precedence(Precedence precedence) {
    advance();
    ParseFn prefix_rule = get_rule(parser.previous.type)->prefix;
    if (prefix_rule == NULL) {
        error("Expect expression.");
        return;
    }

    prefix_rule();

    // We compare precedence to the current [=infix] operator, not the old unary prefix.
    while (precedence <= get_rule(parser.current.type)->precedence) {
        advance();
        ParseFn infix_rule = get_rule(parser.previous.type)->infix;
        infix_rule();
    }
}

static ParseRule *get_rule(TokenType type) {
    return &rules[type];
}

bool compiler_compile(const char *source, Chunk *chunk) {
    scanner_init(source);
    compiling_chunk = chunk;
    parser.had_error = false;

    advance();
    expression();
    consume(TOKEN_EOF, "Expect end of expression");

    end_compiler();
    return !parser.had_error;
}
