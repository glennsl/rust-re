//use std::str;

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

use super::compile::{
    Instruction,
    Char,
    Range,
    Any,
    Fork,
    Jump,
    ConditionalJumpEq,
    ConditionalJumpLE,
    Increment,
    SaveStart,
    SaveEnd,
    AssertStart,
    AssertEnd,
    AssertWordBoundary,
    AssertNonWordBoundary,
    Accept
};

pub fn print_expression_tree(expression: &Expression) {
    print_expression_tree_recursive(expression, 0);
}

fn print_expression_tree_recursive(expression: &Expression, indent: uint) {
    do indent.times {
        print("  ");
    }

    match *expression {
        Literal(c) => println!("Literal({})", c),
        AnyLiteral => println("Any"),
        RangeLiteral(start, end) => println!("Range({}, {})", start, end),
        CharacterClass(ref ranges) => {
            println("CharacterClass");
            for &(start, end) in ranges.iter() {
                do (indent + 1).times {
                    print("  ");
                }
                print!("[{}, {}]", start as u32, end as u32);
                print!(" <{}, {}>", start, end);
                println("");
            }
        }
        Concatenate(ref left, ref right) => {
            println("Concatenate");
            print_expression_tree_recursive(*left, indent + 1);
            print_expression_tree_recursive(*right, indent + 1);
        }
        Alternate(ref left, ref right) => {
            println("Alternate");
            print_expression_tree_recursive(*left, indent + 1);
            print_expression_tree_recursive(*right, indent + 1);
        }
        SubExpression(ref e, capture_slot) => {
            match capture_slot {
                Some(slot) => println!("Group {}", slot),
                None => println("Non-capture Group")
            }
            print_expression_tree_recursive(*e, indent + 1);
        }
        Question(ref e, typ) => {
            println("Question " + quantifier_type_to_str(typ));
            print_expression_tree_recursive(*e, indent + 1);
        }
        Star(ref e, typ) => {
            println("Star " + quantifier_type_to_str(typ));
            print_expression_tree_recursive(*e, indent + 1);
        }
        Plus(ref e, typ) => {
            println("Plus " + quantifier_type_to_str(typ));
            print_expression_tree_recursive(*e, indent + 1);
        }
        ExactRepetition(ref e, count, typ) => {
            println!("ExactRepetition {} {}", count,  quantifier_type_to_str(typ));
            print_expression_tree_recursive(*e, indent + 1);
        }
        UnboundedRepetition(ref e, low, typ) => {
            println!("Unboundedrepetition {}- {}", low, quantifier_type_to_str(typ));
            print_expression_tree_recursive(*e, indent + 1);
        }
        BoundedRepetition(ref e, low, high, typ) => {
            println!("BoundedRepetition {}-{} {}", low, high, quantifier_type_to_str(typ));
            print_expression_tree_recursive(*e, indent + 1);
        }
        EAssertStart => println("AssertStart"),
        EAssertEnd => println("AssertEnd"),
        EAssertWordBoundary => println("AssertWordBoundary"),
        EAssertNonWordBoundary => println("AssertNonWordBoundary")
    }
}

fn quantifier_type_to_str(typ: QuantifierType) -> &'static str {
    match typ {
        Greedy => "Greedy",
        NonGreedy => "NonGreedy"
    }
}

pub fn print_code(instructions: &[Instruction]) {
    for (pc, instruction) in instructions.iter().enumerate() {
        print!("{} ", pc);
        match *instruction {
            Char(c) => println!("CHR {}", c),
            Any => println("ANY"),
            Range(start, end) => println!("RNG {} {}", start, end),
            Fork(left, right) => println!("FRK {} {}", left, right),
            Jump(new_pc) => println!("JMP {}", new_pc),
            ConditionalJumpEq(register, value, new_pc) => println!("JEQ {} {} {}", register, value, new_pc),
            ConditionalJumpLE(register, value, new_pc) => println!("JLE {} {} {}", register, value, new_pc),
            Increment(register) => println!("INC {}", register),
            SaveStart(group) => println!("SVS {}", group),
            SaveEnd(group) => println!("SVE {}", group),
            Accept => println("ACC"),
            AssertStart => println("AS^"),
            AssertEnd => println("AS$"),
            AssertWordBoundary => println("ASb"),
            AssertNonWordBoundary => println("ASB")
        }
    }
}