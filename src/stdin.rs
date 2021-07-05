use std::fmt::Debug;
use std::io::{BufRead, stderr, stdin, stdout, Write};
use std::str::FromStr;

/// 標準入力ストリームから値を読み込みます。
///
/// # Examples
///
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to `rustdoc`, it will even test it for you!
/// use scantool::stdin::scan;
/// let n: i32 = scan();
/// ```
pub fn scan<T: FromStr>() -> T
    where T::Err: Debug {
    let stdout: Option<_> = stdout()
        .flush()
        .ok();
    let stderr: Option<_> = stderr()
        .flush()
        .ok();

    let parse: fn(String) -> Option<T> = |x| x.trim()
        .parse()
        .ok();
    let stdin: Option<T> = stdin()
        .lock()
        .lines()
        .next()
        .and_then(|r| r.ok())
        .and_then(parse);

    stdout.and(stderr)
        .and(stdin)
        .unwrap()
}


#[test]
fn scan_test() {
    assert_eq!(2 + 2, 4);
}
