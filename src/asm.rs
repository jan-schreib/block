use capstone::prelude::*;
use std::path::PathBuf;

// Print register names
fn reg_names<T, I>(cs: &Capstone, regs: T) -> String
    where
        T: Iterator<Item = I>,
        I: Into<u64>,
{
    let names: Vec<String> = regs.map(|x| cs.reg_name(x.into()).unwrap()).collect();
    names.join(", ")
}

/// Print instruction group names
fn group_names<T, I>(cs: &Capstone, regs: T) -> String
    where
        T: Iterator<Item = I>,
        I: Into<u64>,
{
    let names: Vec<String> = regs.map(|x| cs.group_name(x.into()).unwrap()).collect();
    names.join(", ")
}

pub fn decompile(file: PathBuf) -> String {

    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .syntax(arch::x86::ArchSyntax::Att)
        .detail(true)
        .build().unwrap();

    let insns = cs.disasm_all(CODE, 0x1000)?;
}