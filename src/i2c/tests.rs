/*
MIT License

Copyright (c) 2021 Crispin Tschirky <ct@fhr.ch>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 */

use crate::i2c::{calculate_crc8, prepare_cmd, prepare_cmd_with_args, prepare_cmd_with_buf};

#[test]
fn test_build_crc() {
    let bytes: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(0xd0, calculate_crc8(&bytes));
}

#[test]
fn test_build_crc_1() {
    let bytes: Vec<u8> = vec![0x17, 0x10, 0x19, 0x73];
    assert_eq!(0xdf, calculate_crc8(&bytes));
}

#[test]
fn test_build_crc_1_check() {
    let bytes: Vec<u8> = vec![0x17, 0x10, 0x19, 0x73, 0xdf];
    assert_eq!(0x00, calculate_crc8(&bytes));
}

#[test]
fn test_build_crc_2() {
    let bytes: Vec<u8> = vec![
        0x17, 0x10, 0x19, 0x73, 0x16, 0x03, 0x20, 0x00, 0x23, 0x06, 0x20, 0x02,
    ];
    assert_eq!(0x6, calculate_crc8(&bytes));
}

#[test]
fn test_build_crc_2_check() {
    let bytes: Vec<u8> = vec![
        0x17, 0x10, 0x19, 0x73, 0x16, 0x03, 0x20, 0x00, 0x23, 0x06, 0x20, 0x02, 0x6,
    ];
    assert_eq!(0, calculate_crc8(&bytes));
}

#[test]
fn test_build_crc_3() {
    let bytes: Vec<u8> = vec![0xbe, 0xef];
    assert_eq!(0x92, calculate_crc8(&bytes));
}

#[test]
fn test_build_crc_4() {
    let bytes: Vec<u8> = vec![17, 10, 19, 73];
    assert_eq!(0x5c, calculate_crc8(&bytes));
}

#[test]
fn test_create_cmd_with_args() {
    let args: u16 = 0x1234;
    let buf = prepare_cmd_with_args(0x100a, args);
    assert_eq!(0x10, buf[0]);
    assert_eq!(0x0a, buf[1]);
    assert_eq!(0x12, buf[2]);
    assert_eq!(0x34, buf[3]);
    assert_eq!(0x37, buf[4]);
    assert_eq!(5, buf.len());
}

#[test]
fn test_create_cmd_with_args_set_intervall_2s() {
    let args: u16 = 2;
    let buf = prepare_cmd_with_args(0x4600, args);
    assert_eq!(0x46, buf[0]);
    assert_eq!(0x00, buf[1]);
    assert_eq!(0x00, buf[2]);
    assert_eq!(0x02, buf[3]);
    assert_eq!(0xE3, buf[4]);
    assert_eq!(5, buf.len());
}

#[test]
fn test_create_cmd_with_data_and_crc() {
    let in_buf = [0x17, 0x10, 0x19, 0x73];
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
    let in_buf = [0x17, 0x10, 0x19, 0x73];
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
