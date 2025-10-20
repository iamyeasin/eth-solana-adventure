use anchor_lang::prelude::*;

declare_id!("GKZn2fiYRkW8onFCNdtmVL4SYE99YHTiQQXaL1BR3zaa");

#[program]
pub mod day2 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>,
                        a: u64,
                        b: u64,
                    c: String) -> Result<()> {
        msg!("You sent {} and {} and {:?}", a, b, c);
        // msg!("Addition: {}", a - b);
        // msg!("Subtraction: {}", a - b);
        // msg!("Multiplication: {}", a * b);
        // msg!("Division: {}", a / b);
        msg!("sqrt {}", a.isqrt() as u64);
        msg!("log10 {}", a.ilog10() as u64);
        Ok(())
    }

    pub fn array(_ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Array contents: {:?}", arr);
        Ok(())
    }

    pub fn sum(_ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let sum = a + b;
        let x_safe = a.checked_add(b).unwrap();
        msg!("Sum: {}, Safe: {}", sum, x_safe);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
