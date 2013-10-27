use EAssertStart = super::parse::AssertStart;
use EAssertEnd = super::parse::AssertEnd;
use EAssertWordBoundary = super::parse::AssertWordBoundary;
use EAssertNonWordBoundary = super::parse::AssertNonWordBoundary;
use super::parse::{
	Expression,
	Literal,
	AnyLiteral,
	RangeLiteral,
	CharacterClass,
	Concatenate,
	Alternate,
	SubExpression,
	Question,
	Star,
	Plus,
	ExactRepetition,
	UnboundedRepetition,
	BoundedRepetition
};
use super::parse::{
	QuantifierType,
	Greedy,
	NonGreedy
};

pub enum Instruction {
    Char(char),
    Range(char, char),
    Any,
    Fork(uint, uint),
    Jump(uint), 
    ConditionalJumpEq(uint, uint, uint), // (register, value, pc)
    ConditionalJumpLE(uint, uint, uint), // (register, value, pc)
    Increment(uint),
    SaveStart(uint),
    SaveEnd(uint),
    AssertStart,
    AssertEnd,
    AssertWordBoundary,
    AssertNonWordBoundary,
    Accept
}

// TODO: Also defined in parse
#[inline]
fn do_alternate(stack: &mut ~[Expression]) {
    while stack.len() > 1 {
        let (right, left) = (stack.pop(), stack.pop());
        stack.push(Alternate(~left, ~right));
    }
}

pub fn compile(expression: &Expression) -> ~[Instruction] {
    let mut code = ~[];
    compile_recursive(expression, &mut code, &mut 0);
    code.push(Accept);
    return code;
}

pub fn count_registers(code: &[Instruction]) -> uint {
	let mut max = 0;

	for instruction in code.iter() {
		match *instruction {
			Increment(register) |
			ConditionalJumpEq(register, _, _) |
			ConditionalJumpLE(register, _, _) => {
				if register > max {
					max = register;
				}
			},
			_ => ()
		}
	}

	return max + 1;
}

fn compile_recursive(expression: &Expression, code: &mut ~[Instruction], registers: &mut uint) {

    match *expression {
        Literal(c) => {
        	code.push(Char(c));
        }
        AnyLiteral => {
        	code.push(Any);
        }
        RangeLiteral(start, end) => {
        	code.push(Range(start, end));
        }
        CharacterClass(ref ranges) => {
            let mut stack = ranges.map(|r| {
                match r {
                    &(start, end) if start == end => Literal(start),
                    &(start, end) => RangeLiteral(start, end)
                }
            });
            do_alternate(&mut stack);
            compile_recursive(&stack.pop(), code, registers);
        }
        Concatenate(ref left, ref right) => {
            compile_recursive(*left, code, registers);
            compile_recursive(*right, code, registers);
        }
        Alternate(ref left, ref right) => {
            let pc = code.len();
            code.push(Fork(0, 0));
            compile_recursive(*left, code, registers);
            code[pc] = Fork(pc + 1, code.len() + 1);
            let jump_pc = code.len();
            code.push(Jump(0));
            compile_recursive(*right, code, registers);
            code[jump_pc] = Jump(code.len());
        }
        SubExpression(ref e, capture_slot) => {
        	match capture_slot {
        		Some(slot) => {
		        	code.push(SaveStart(slot));
		        	compile_recursive(*e, code, registers);
		        	code.push(SaveEnd(slot));
        		}
        		None => compile_recursive(*e, code, registers)
        	}
        }
        Question(ref e, typ) => {
            let pc = code.len();
            code.push(Fork(0, 0));
            compile_recursive(*e, code, registers);
            let instr = fork(typ, pc + 1, code.len()); 
            code[pc] = instr;
        }
        Star(ref e, typ) => {
            let pc = code.len();
            code.push(Fork(0, 0));
            compile_recursive(*e, code, registers);
            let instr = fork(typ, pc + 1, code.len() + 1); 
            code[pc] = instr;
            code.push(Jump(pc));
        }
        Plus(ref e, typ) => {
            let pc = code.len();
            compile_recursive(*e, code, registers);
            let instr = fork(typ, pc, code.len() + 1); 
            code.push(instr);
        }
        ExactRepetition(ref e, count, typ) => {
        	let register = *registers;
        	*registers += 1;
            let pc = code.len();
            code.push(ConditionalJumpEq(0, 0, 0));
            code.push(Fork(0, 0));
            compile_recursive(*e, code, registers);
            code.push(Increment(register));
            code[pc] = ConditionalJumpEq(register, count, code.len() + 1);
            code[pc + 1] = fork(typ, pc + 2, code.len() + 1);
            code.push(Jump(pc));
        }
        UnboundedRepetition(ref e, low, typ) => {
        	let register = *registers;
        	*registers += 1;
            let pc = code.len();
            code.push(ConditionalJumpLE(register, low, pc + 2));
            code.push(Fork(0, 0));
            compile_recursive(*e, code, registers);
            code.push(Increment(register));
            code[pc + 1] = fork(typ, pc + 2, code.len() + 1);
            code.push(Jump(pc));
        }
        BoundedRepetition(ref e, low, high, typ) => {
        	let register = *registers;
        	*registers += 1;
            let pc = code.len();
            code.push(ConditionalJumpEq(0, 0, 0));
            code.push(ConditionalJumpLE(register, low, pc + 3));
            code.push(Fork(0, 0));
            compile_recursive(*e, code, registers);
            code.push(Increment(register));
            code[pc] = ConditionalJumpEq(register, high, code.len() + 1);
            code[pc + 2] = fork(typ, pc + 3, code.len() + 1);
            code.push(Jump(pc));
        }
        EAssertStart => code.push(AssertStart),
        EAssertEnd => code.push(AssertEnd),
        EAssertWordBoundary => code.push(AssertWordBoundary),
        EAssertNonWordBoundary => code.push(AssertNonWordBoundary)
    }
}

#[inline]
fn fork(typ: QuantifierType, greedy: uint, nongreedy: uint) -> Instruction {
	match typ {
		Greedy => Fork(greedy, nongreedy),
		NonGreedy => Fork(nongreedy, greedy)
	}
}