pub mod macros;

pub fn contains<T: PartialEq + Eq>(list:&[T], elem:T) -> Result<usize, ()> {
    #[allow(clippy::needless_range_loop)]
    for i in 0..list.len() {
        if list[i] == elem {
            return Ok(i)
        }
    }

    Err(())
}

pub fn identical_elems<T: PartialEq + Eq>(list:&[T]) -> bool {
    #[allow(clippy::needless_range_loop)]

    if list.len() < 2 {
        return false
    }

    for i in 0..list.len() {
        for j in i+1..list.len() {
            if list[i] == list[j] {
                return true
            }
        }
    }
    false
}

pub fn get_loc(buf:&[u8]) -> usize {
    let mut loc:usize = 1;
    for byte in buf {
        if *byte == b'\n' {
            loc += 1;
        }
    }
    loc
}