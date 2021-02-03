use crate::scd30::calculate_crc8;

#[test]
fn test_build_crc() {
    let bytes: Vec<u8> = vec![1,2,3,4,5,6,7,8];
    assert_eq!(0xd0, calculate_crc8( &bytes));
}

#[test]
fn test_build_crc_1() {
    let bytes: Vec<u8> = vec![0x17,0x10,0x19,0x73];
    assert_eq!(0xdf, calculate_crc8( &bytes));
}

#[test]
fn test_build_crc_1_check() {
    let bytes: Vec<u8> = vec![0x17,0x10,0x19,0x73,0xdf];
    assert_eq!(0x00, calculate_crc8( &bytes));
}

#[test]
fn test_build_crc_2() {
    let bytes: Vec<u8> = vec![0x17,0x10,0x19,0x73,0x16,0x03,0x20,0x00,0x23,0x06,0x20,0x02];
    assert_eq!(0x6, calculate_crc8( &bytes));
}

#[test]
fn test_build_crc_2_check() {
    let bytes: Vec<u8> = vec![0x17,0x10,0x19,0x73,0x16,0x03,0x20,0x00,0x23,0x06,0x20,0x02,0x6];
    assert_eq!(0, calculate_crc8( &bytes));
}

#[test]
fn test_build_crc_3() {
    let bytes: Vec<u8> = vec![0xbe, 0xef];
    assert_eq!(0x92, calculate_crc8( &bytes));
}

#[test]
fn test_build_crc_4() {
    let bytes: Vec<u8> = vec![17, 10 ,19 , 73];
    assert_eq!(0x5c, calculate_crc8( &bytes));
}
