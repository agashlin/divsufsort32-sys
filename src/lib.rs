#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn sufsort() {
        let s = b"abcdabcd";
        let mut sa = [-1i32; 10];

        unsafe {
            assert_eq!(
                super::divsufsort(s.as_ptr(), sa.as_mut_ptr(), s.len() as i32),
                0
            );
        }
        assert_eq!(sa, [4, 0, 5, 1, 6, 2, 7, 3, -1, -1]);
    }

    #[test]
    fn sufsort_part() {
        let s = b"abcdabcd";
        let n = 6;
        let mut sa = [-1i32; 10];
        unsafe {
            assert_eq!(super::divsufsort(s.as_ptr(), sa.as_mut_ptr(), n), 0);
        }
        assert_eq!(sa, [4, 0, 5, 1, 2, 3, -1, -1, -1, -1]);
    }

    #[test]
    fn sufsort_0() {
        let s = [4u8, 0, 3, 1, 2];
        let mut sa = [-1i32; 5];
        unsafe {
            assert_eq!(
                super::divsufsort(s.as_ptr(), sa.as_mut_ptr(), s.len() as i32),
                0
            );
        }
        assert_eq!(sa, [1, 3, 4, 2, 0]);
    }
}
