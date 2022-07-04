pub enum OP {
    OP_BR = 0, // branch
    OP_ADD,    // add
    OP_LD,     // load
    OP_ST,     // store
    OP_JSR,    // jump register
    OP_AND,    // bit and
    OP_LDR,    // load register
    OP_STR,    // store register
    OP_RTI,    // unused
    OP_NOT,    // bit not
    OP_LDI,    // load indirect
    OP_STI,    // store indirect
    OP_JMP,    //jump
    OP_RES,    // reserved unused
    OP_LEA,    // load effective address
    OP_TRAP,   //execute trap
}