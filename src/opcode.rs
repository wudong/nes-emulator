pub struct Opcode{
    code: u8,
    param_size: u8,    
    exec: OpcodeFn,
}

#[derive(Debug)]
#[allow(now_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absoluate,
    Absoluate_X,
    Absoluate_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

type OpcodeFn = fn(&CPU, &AddresingMode) ;
