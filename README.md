![test](https://github.com/yoshd/pokebell/workflows/test/badge.svg)

# pokebell

ポケベルの2タッチ入力の相互変換ライブラリ

## example

```rs
let c = Converter::new();
c.convert_to_two_touch_string("ごくろうさん".to_string()).unwrap(); // ["5963", "25042395133103"]
c.convert_from_two_touch_string("81225223".to_string()).unwrap(); // "やきにく"
```
