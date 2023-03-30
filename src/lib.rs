use std::vec::Vec;

#[derive(Debug, PartialEq, Eq)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];
    for s in inputs.iter() {
        match s {
            CalculatorInput::Value(n) => {
                stack.push(*n);
            }
            op => match op {
                CalculatorInput::Add => {
                    if stack.len() < 2 {
                        return None;
                    }
                    let result = operation(i32::checked_add, &stack[stack.len() - 2..])?;
                    update_stack(&mut stack, result);
                }
                CalculatorInput::Subtract => {
                    if stack.len() < 2 {
                        return None;
                    }
                    let result = operation(i32::checked_sub, &stack[stack.len() - 2..])?;
                    update_stack(&mut stack, result);
                }
                CalculatorInput::Multiply => {
                    if stack.len() < 2 {
                        return None;
                    }
                    let result = operation(i32::checked_mul, &stack[stack.len() - 2..])?;
                    update_stack(&mut stack, result);
                }
                CalculatorInput::Divide => {
                    if stack.len() < 2 {
                        return None;
                    }
                    let result = operation(i32::checked_div, &stack[stack.len() - 2..])?;
                    update_stack(&mut stack, result);
                }
                _ => return None,
            },
        }
    }
    if stack.len() > 1 {
        return None;
    }
    stack.pop()
}

fn update_stack(stack: &mut Vec<i32>, value: i32) -> () {
    *stack = stack[..stack.len() - 2].to_owned();
    stack.push(value);
}

fn operation(op: fn(i32, i32) -> Option<i32>, nums: &[i32]) -> Option<i32> {
    return op(nums[0], nums[1]);
}
