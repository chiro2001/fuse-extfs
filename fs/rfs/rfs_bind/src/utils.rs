#[macro_export]
macro_rules! ret {
    ($n:expr, $e:expr, $expected:expr) => {
        {
            let ret = $e;
            match ret {
                $expected => Ok(()),
                _ => Err(anyhow!("{} returns error! value = {}", $n, ret))
            }
        }
    };
    ($n:expr, $e:expr) => {
        ret!($n, $e, 0)
    }
}

#[macro_export]
macro_rules! ret_ne {
    ($n:expr, $e:expr) => {
        {
            let ret = $e;
            if ret != 0 { Ok(()) } else { Err(anyhow!("{} returns error! value = {}", $n, ret)) }
        }
    }
}
