use crate::Function;

pub fn func_name(f: &Function) -> String {
    match f {
        Function::Average => "AVG",
        Function::BitAnd => "BIT_AND",
        Function::BitOr => "BIT_OR",
        Function::BitXor => "BIT_XOR",
        Function::Count => "COUNT",
        Function::CountDistinct => "COUNT",
        Function::Concat => "CONCAT",
        Function::Max => "MAX",
        Function::Min => "MIN",
    }
    .to_string()
}
