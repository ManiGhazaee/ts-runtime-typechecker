#[macro_export]
macro_rules! next {
    ($src_vec:ident, $i:ident, $default:expr, $($pattern:literal => $result:expr),+ $(,)?) => {
        $i += 1;
        if let Some(next_char) = $src_vec.get($i) {
            match *next_char as char {
                $($pattern => $result,)*
                _ => {
                    $i -= 1;
                    $default
                }
            }
        } else {
            $i -= 1;
            $default
        }
    };
}
