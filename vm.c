
#include <printf.h>
#include "vm.h"
#include "debug.h"

// Global variables bad but...
Vm vm;

/// Read the byte at IP, and increment it.
static uint8_t read_byte() {
    uint8_t byte = *vm.ip;
    vm.ip += 1;
    return byte;
}

static Value read_constant() {
    int constant_idx = read_byte();
    return vm.chunk->constants.data[constant_idx];
}

static InterpretResult run() {
// Macros are evil and so am I
#define BINARY_OP(op) \
    do {                           \
        double b = vm_stack_pop(); \
        double a = vm_stack_pop(); \
        vm_stack_push(a op b);     \
    } while (false)

    while(true) {
        // TODO -- If bytecode ends without a return, loops back again and throws an error
        //  when getting line number, before breaking on default. Fix?
#ifdef DEBUG_TRACE_EXECUTION
        printf("        ");
        for (Value *slot = vm.stack; slot < vm.stack_top; slot++) {
            printf("[ ");
            value_print(*slot);
            printf(" ]");
        }
        printf("\n");
        disassemble_instruction(vm.chunk, (int)(vm.ip - vm.chunk->bytecode));
#endif
        OpCode instr = read_byte();
        switch (instr) {
            case OP_CONSTANT: {
                Value constant = read_constant();
                vm_stack_push(constant);
                break;
            }
            case OP_CONSTANT_LONG:
                break;
            case OP_NEGATE:
                vm_stack_push(-vm_stack_pop());
                break;
            case OP_ADD:
                BINARY_OP(+);
                break;
            case OP_SUBTRACT:
                BINARY_OP(-);
                break;
            case OP_MULTIPLY:
                BINARY_OP(*);
                break;
            case OP_DIVIDE:
                BINARY_OP(/);
                break;
            case OP_RETURN:
                value_print(vm_stack_pop());
                printf("\n");
                return INTERPRET_OK;
            default:
                break;
        }
    }

#undef BINARY_OP
}

static void reset_stack() {
    vm.stack_top = vm.stack;
}

void vm_init() {
    reset_stack();
}

void vm_free() {

}

void vm_stack_push(Value value) {
    *vm.stack_top = value;
    vm.stack_top += 1;
}

Value vm_stack_pop() {
    vm.stack_top -= 1;
    return *vm.stack_top;
}

InterpretResult vm_interpret(Chunk *chunk) {
    vm.chunk = chunk;
    vm.ip = vm.chunk->bytecode;
    return run();
}