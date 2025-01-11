use codegen::{sum_macro, sum_macro_name_args_idea};

fn sum(lhs: u32, rhs: u32) -> u32 {
    lhs + rhs
}

fn main() {
    let result1 = sum(1, 2);
    let result2 = sum_macro!(1, 2);
    let result3 = sum_macro_name_args_idea!(1, 2);
}
