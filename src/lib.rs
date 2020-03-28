//! ポケベルの2タッチ入力の相互変換ライブラリ
//!
//! 機種依存の入力方法はサポートしません。

#[macro_use]
extern crate failure;

pub mod c_interface;
pub mod two_touch_input;
