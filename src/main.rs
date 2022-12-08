//! これはhello_worldモジュールのmain関数に標準出力を渡して実行するプログラムです。

extern crate hello_world; //ローカルのhello_worldを読みこむ。

use std::io::{self}; //標準入出力のライブラリをインポートする。

/// hello_worldモジュールのmain関数に標準出力を引数として与え実行する。
fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    hello_world::main(&mut stdout).expect("I/O failed in hello_world module.");
}
