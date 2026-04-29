// - that `sploosh(splish(-1, 0), splish(1, 1), splish(3, 2))` returns the value `4`

#[cfg(test)]
#[test]
fn integration_testing(){

    use testing::*;
    assert_eq!( sploosh(splish(-1, 0),  splish(1, 1), splish(3, 2)), 4);
}