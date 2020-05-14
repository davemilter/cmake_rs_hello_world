extern "C" {
    fn f(_: i32, _: i32) -> i32;
}

pub fn my_f(a: i32, b: i32) -> i32 {
    unsafe { f(a, b) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, my_f(1, 1));
    }
}
