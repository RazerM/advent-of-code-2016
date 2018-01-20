use std::str::FromStr;

use nom::digit;

named!(pub int32<&str, i32>,
    map!(
        recognize!(
            tuple!(
                opt!(tag!("-")),
                call!(digit)
            )
        ),
        |s| FromStr::from_str(s).unwrap()
    )
);

named!(pub int_usize<&str, usize>,
    map!(
        recognize!(
            tuple!(
                opt!(tag!("-")),
                call!(digit)
            )
        ),
        |s| FromStr::from_str(s).unwrap()
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! int32res {
        ($p:expr) => (
            int32($p).to_result().unwrap();
        )
    }

    #[test]
    fn test_int32() {
        assert_eq!(int32res!("5"), 5);
        assert_eq!(int32res!("10"), 10);
        assert_eq!(int32res!("-10"), -10);
    }
}
