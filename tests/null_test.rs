use k4rust::*;

#[test]
fn test_nulls_and_infinities() {
    assert_eq!(NH, i16::MIN);
    assert_eq!(WH, i16::MAX);
    assert_eq!(NI, i32::MIN);
    assert_eq!(WI, i32::MAX);
    assert_eq!(NJ, i64::MIN);
    assert_eq!(WJ, i64::MAX);
    assert!(NE.is_nan());
    assert!(NF.is_nan());
    assert!(WE.is_infinite());
    assert!(WF.is_infinite());
    assert_eq!(NC, b' ');

    assert_eq!(UU, 2);
    assert_eq!(KP, 12);
    assert_eq!(KM, 13);
    assert_eq!(KD, 14);
    assert_eq!(KZ, 15);
    assert_eq!(KN, 16);
    assert_eq!(KU, 17);
    assert_eq!(KV, 18);
    assert_eq!(KT, 19);
    assert_eq!(XT, 98);
    assert_eq!(XD, 99);
}
