//! これはwritelnメソッドを利用して「Hello world!」を出力するプログラムです。

use std::io::{self, Write}; //標準入出力のライブラリとWriteをインポートする。

/// Hello world!が出力されます。また、返り値としてioのResultが返却されます。
/// * `write_place` - writelnでの出力先を指定します。標準出力を引数に与えれば標準出力を行います。
pub fn main<W: Write>(write_place: &mut W) -> io::Result<()> {
    writeln!(write_place, "Hello world!") //writelnを利用しwrite_placeへ「Hello world!」を書き込む。
}

// 単体テストを行います。cargo testコマンドでのみ実行されます。
#[cfg(test)]
mod tests {
    // testの外のものをインポートします。
    use super::*;

    #[test]
    fn test_main() {
        let mut buf = Vec::<u8>::new(); 
        let result = main(&mut buf);
        let output_text = String::from_utf8(buf).unwrap();
        assert!(result.is_ok());
        assert_eq!(output_text, "Hello world!\n");
    }
}
