// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.

struct ColorClassicStruct {
    // TODO: Something goes here
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTupleStruct(u8, u8, u8);

// 在 Rust 中，#[derive(Debug)]是一个非常重要的属性，它为结构体自动实现了 Debugtrait
// Debug trait 的作用：
//  1. 调试输出：
//      - 允许使用 {:?}或 {:#?}（美化输出）格式化
//      - 在 println!, format!, dbg!等宏中使用
//  2. 测试断言：
//      - assert_eq!宏在失败时依赖Debug输出

#[derive(Debug)]
struct UnitLikeStruct;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct{
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct;

        // {:?}是调试格式说明符，它要求对应的类型必须实现 std::fmt::Debugtrait
        // 这也就是为什么前面在声明struct UnitLikeStruct时需要加上#[derive(Debug)]
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
