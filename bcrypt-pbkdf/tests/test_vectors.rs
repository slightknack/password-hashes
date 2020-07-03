extern crate bcrypt_pbkdf;

use bcrypt_pbkdf::bcrypt_pbkdf;

#[test]
fn test_openbsd_vectors() {
    struct Test {
        password: &'static str,
        salt: Vec<u8>,
        rounds: u32,
        out: Vec<u8>,
    }

    let tests = vec!(
            Test{
                password: "password",
                salt: b"salt".to_vec(),
                rounds: 4,
                out: vec![
                    0x5b, 0xbf, 0x0c, 0xc2, 0x93, 0x58, 0x7f, 0x1c, 0x36, 0x35, 0x55, 0x5c, 0x27, 0x79, 0x65, 0x98,
                    0xd4, 0x7e, 0x57, 0x90, 0x71, 0xbf, 0x42, 0x7e, 0x9d, 0x8f, 0xbe, 0x84, 0x2a, 0xba, 0x34, 0xd9],
            }, Test{
                password: "password",
                salt: vec![0],
                rounds: 4,
                out: vec![0xc1, 0x2b, 0x56, 0x62, 0x35, 0xee, 0xe0, 0x4c, 0x21, 0x25, 0x98, 0x97, 0x0a, 0x57, 0x9a, 0x67],
            }, Test{
                password: "\x00",
                salt: b"salt".to_vec(),
                rounds: 4,
                out: vec![0x60, 0x51, 0xbe, 0x18, 0xc2, 0xf4, 0xf8, 0x2c, 0xbf, 0x0e, 0xfe, 0xe5, 0x47, 0x1b, 0x4b, 0xb9],
            }, Test{
                password: "password\x00",
                salt: b"salt\x00".to_vec(),
                rounds: 4,
                out: vec![
                    0x74, 0x10, 0xe4, 0x4c, 0xf4, 0xfa, 0x07, 0xbf, 0xaa, 0xc8, 0xa9, 0x28, 0xb1, 0x72, 0x7f, 0xac,
                    0x00, 0x13, 0x75, 0xe7, 0xbf, 0x73, 0x84, 0x37, 0x0f, 0x48, 0xef, 0xd1, 0x21, 0x74, 0x30, 0x50],
            }, Test{
                password: "pass\x00wor",
                salt: b"sa\x00l".to_vec(),
                rounds: 4,
                out: vec![0xc2, 0xbf, 0xfd, 0x9d, 0xb3, 0x8f, 0x65, 0x69, 0xef, 0xef, 0x43, 0x72, 0xf4, 0xde, 0x83, 0xc0],
            }, Test{
                password: "pass\x00word",
                salt: b"sa\x00lt".to_vec(),
                rounds: 4,
                out: vec![0x4b, 0xa4, 0xac, 0x39, 0x25, 0xc0, 0xe8, 0xd7, 0xf0, 0xcd, 0xb6, 0xbb, 0x16, 0x84, 0xa5, 0x6f],
            }, Test{
                password: "password",
                salt: b"salt".to_vec(),
                rounds: 8,
                out: vec![
                    0xe1, 0x36, 0x7e, 0xc5, 0x15, 0x1a, 0x33, 0xfa, 0xac, 0x4c, 0xc1, 0xc1, 0x44, 0xcd, 0x23, 0xfa,
                    0x15, 0xd5, 0x54, 0x84, 0x93, 0xec, 0xc9, 0x9b, 0x9b, 0x5d, 0x9c, 0x0d, 0x3b, 0x27, 0xbe, 0xc7,
                    0x62, 0x27, 0xea, 0x66, 0x08, 0x8b, 0x84, 0x9b, 0x20, 0xab, 0x7a, 0xa4, 0x78, 0x01, 0x02, 0x46,
                    0xe7, 0x4b, 0xba, 0x51, 0x72, 0x3f, 0xef, 0xa9, 0xf9, 0x47, 0x4d, 0x65, 0x08, 0x84, 0x5e, 0x8d],
            }, Test{
                password: "password",
                salt: b"salt".to_vec(),
                rounds: 42,
                out: vec![0x83, 0x3c, 0xf0, 0xdc, 0xf5, 0x6d, 0xb6, 0x56, 0x08, 0xe8, 0xf0, 0xdc, 0x0c, 0xe8, 0x82, 0xbd],
            }, Test{
                password: "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
                salt: b"salis\x00".to_vec(),
                rounds: 8,
                out: vec![0x10, 0x97, 0x8b, 0x07, 0x25, 0x3d, 0xf5, 0x7f, 0x71, 0xa1, 0x62, 0xeb, 0x0e, 0x8a, 0xd3, 0x0a],
            },
        );

    for t in tests.iter() {
        let mut out = vec![0; t.out.len()];
        bcrypt_pbkdf(&t.password[..], &t.salt[..], t.rounds, &mut out).unwrap();
        assert_eq!(out, t.out);
    }
}