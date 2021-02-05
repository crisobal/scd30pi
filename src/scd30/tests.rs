use crate::scd30::{calculate_crc8,  prepare_cmd_with_args, prepare_cmd_with_buf, prepare_cmd};

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


#[test]
fn test_create_cmd_with_args() {
    let args : u16 = 0x1234;
    let buf = prepare_cmd_with_args(0x100a,args);
    assert_eq!(0x10, buf[0]);
    assert_eq!(0x0a, buf[1]);
    assert_eq!(0x12, buf[2]);
    assert_eq!(0x34, buf[3]);
    assert_eq!(0x37, buf[4]);
    assert_eq!(5, buf.len());
}


#[test]
fn test_create_cmd_with_data_and_crc() {
    let in_buf = [ 0x17,0x10,0x19,0x73];
    let buf = prepare_cmd_with_buf(0x100a, &in_buf, true);
    assert_eq!(0x10, buf[0]);
    assert_eq!(0x0a, buf[1]);
    assert_eq!(0x17, buf[2]);
    assert_eq!(0x10, buf[3]);
    assert_eq!(0x19, buf[4]);
    assert_eq!(0x73, buf[5]);
    assert_eq!(0xdf, buf[6]);
    assert_eq!(7, buf.len());
}

#[test]
fn test_create_cmd_with_data() {
    let in_buf = [ 0x17,0x10,0x19,0x73];
    let buf = prepare_cmd_with_buf(0x100a, &in_buf, false);
    assert_eq!(0x10, buf[0]);
    assert_eq!(0x0a, buf[1]);
    assert_eq!(0x17, buf[2]);
    assert_eq!(0x10, buf[3]);
    assert_eq!(0x19, buf[4]);
    assert_eq!(0x73, buf[5]);
    assert_eq!(6, buf.len());
}


#[test]
fn test_create_cmd_cmd_only_1() {
    let buf = prepare_cmd(0x100a);
    assert_eq!(0x10, buf[0]);
    assert_eq!(0x0a, buf[1]);
    assert_eq!(2, buf.len())
}