use std::collections::HashMap;

pub struct Converter {
    base_map: HashMap<char, String>,
    inversed_base_map: HashMap<String, char>,
    normalization_map: HashMap<char, char>,
    reserved_word_map: HashMap<String, Vec<String>>,
}

impl Converter {
    /// 入力された文字列を2タッチ入力の数字に変換
    /// 入力可能な文字列は [2タッチ入力](https://ja.wikipedia.org/wiki/2%E3%82%BF%E3%83%83%E3%83%81%E5%85%A5%E5%8A%9B) , [ポケベル解読！数字の意味が分かる早見表！](https://koma-yome.com/archives/724) 参照
    ///
    /// ## Example
    /// ```
    /// let c = Converter::new();
    /// c.convert_to_two_touch_string("ごくろうさん".to_string()).unwrap(); // ["5963", "25042395133103"]
    /// ```
    pub fn convert_to_two_touch_string(&self, val: String) -> Result<Vec<String>, Error> {
        if val.is_empty() {
            return Err(Error::from(ErrorKind::ParseError));
        }
        let mut ret = Vec::new();
        if let Some(reserved) = self.reserved_word_map.get(&val) {
            ret.append(&mut reserved.clone());
        }
        let chars = val.chars();
        let mut normal = String::new();
        for mut ch in chars {
            if ch.is_ascii_alphabetic() {
                ch = ch.to_ascii_uppercase();
            }
            let ch = self.normalize(&ch);
            match self.base_map.get(&ch) {
                Some(s) => normal += s,
                None => {
                    if ret.is_empty() {
                        return Err(Error::from(ErrorKind::ParseError));
                    }
                    return Ok(ret);
                }
            };
        }
        ret.push(normal);
        Ok(ret)
    }

    /// 2タッチ入力から日本語に変換する。
    /// 濁点等は別の文字になる。
    /// 例: 2104 -> か゛
    ///
    /// ## Example
    /// ```
    /// let c = Converter::new();
    /// c.convert_from_two_touch_string("81225223".to_string()).unwrap(); // "やきにく"
    /// ```
    pub fn convert_from_two_touch_string(&self, val: String) -> Result<String, Error> {
        if val.len() % 2 != 0 || val.is_empty() || !val.is_ascii() {
            return Err(Error::from(ErrorKind::ParseError));
        }
        let mut ret = String::new();
        for i in 0..(val.len() / 2) {
            let idx = i * 2;
            match self.inversed_base_map.get(&val[idx..(idx + 2)]) {
                Some(ch) => ret.push(*ch),
                None => return Err(Error::from(ErrorKind::ParseError)),
            };
        }
        Ok(ret)
    }

    fn normalize(&self, ch: &char) -> char {
        match self.normalization_map.get(ch) {
            Some(nc) => *nc,
            None => *ch,
        }
    }

    /// Converterの初期化
    /// (もっといい方法があるかもしれない)
    pub fn new() -> Self {
        let mut base_map = HashMap::new();
        // see https://ja.wikipedia.org/wiki/2%E3%82%BF%E3%83%83%E3%83%81%E5%85%A5%E5%8A%9B
        base_map.insert('あ', "11".to_string());
        base_map.insert('い', "12".to_string());
        base_map.insert('う', "13".to_string());
        base_map.insert('え', "14".to_string());
        base_map.insert('お', "15".to_string());
        base_map.insert('か', "21".to_string());
        base_map.insert('き', "22".to_string());
        base_map.insert('く', "23".to_string());
        base_map.insert('け', "24".to_string());
        base_map.insert('こ', "25".to_string());
        base_map.insert('さ', "31".to_string());
        base_map.insert('し', "32".to_string());
        base_map.insert('す', "33".to_string());
        base_map.insert('せ', "34".to_string());
        base_map.insert('そ', "35".to_string());
        base_map.insert('た', "41".to_string());
        base_map.insert('ち', "42".to_string());
        base_map.insert('つ', "43".to_string());
        base_map.insert('て', "44".to_string());
        base_map.insert('と', "45".to_string());
        base_map.insert('な', "51".to_string());
        base_map.insert('に', "52".to_string());
        base_map.insert('ぬ', "53".to_string());
        base_map.insert('ね', "54".to_string());
        base_map.insert('の', "55".to_string());
        base_map.insert('は', "61".to_string());
        base_map.insert('ひ', "62".to_string());
        base_map.insert('ふ', "63".to_string());
        base_map.insert('へ', "64".to_string());
        base_map.insert('ほ', "65".to_string());
        base_map.insert('ま', "71".to_string());
        base_map.insert('み', "72".to_string());
        base_map.insert('む', "73".to_string());
        base_map.insert('め', "74".to_string());
        base_map.insert('も', "75".to_string());
        base_map.insert('や', "81".to_string());
        base_map.insert('(', "82".to_string());
        base_map.insert('ゆ', "83".to_string());
        base_map.insert(')', "84".to_string());
        base_map.insert('よ', "85".to_string());
        base_map.insert('ら', "91".to_string());
        base_map.insert('り', "92".to_string());
        base_map.insert('る', "93".to_string());
        base_map.insert('れ', "94".to_string());
        base_map.insert('ろ', "95".to_string());
        base_map.insert('わ', "01".to_string());
        base_map.insert('を', "02".to_string());
        base_map.insert('ん', "03".to_string());
        base_map.insert('゛', "04".to_string());
        base_map.insert('゜', "05".to_string());

        base_map.insert('A', "16".to_string());
        base_map.insert('B', "17".to_string());
        base_map.insert('C', "18".to_string());
        base_map.insert('D', "19".to_string());
        base_map.insert('E', "10".to_string());
        base_map.insert('F', "26".to_string());
        base_map.insert('G', "27".to_string());
        base_map.insert('H', "28".to_string());
        base_map.insert('I', "29".to_string());
        base_map.insert('J', "20".to_string());
        base_map.insert('K', "36".to_string());
        base_map.insert('L', "37".to_string());
        base_map.insert('M', "38".to_string());
        base_map.insert('N', "39".to_string());
        base_map.insert('O', "30".to_string());
        base_map.insert('P', "46".to_string());
        base_map.insert('Q', "47".to_string());
        base_map.insert('R', "48".to_string());
        base_map.insert('S', "49".to_string());
        base_map.insert('T', "40".to_string());
        base_map.insert('U', "56".to_string());
        base_map.insert('V', "57".to_string());
        base_map.insert('W', "58".to_string());
        base_map.insert('X', "59".to_string());
        base_map.insert('Y', "50".to_string());
        base_map.insert('Z', "66".to_string());
        base_map.insert('?', "67".to_string());
        base_map.insert('!', "68".to_string());
        base_map.insert('-', "69".to_string());
        base_map.insert('/', "60".to_string());
        base_map.insert('\\', "76".to_string());
        base_map.insert('&', "77".to_string());
        base_map.insert('*', "86".to_string());
        base_map.insert('#', "87".to_string());
        base_map.insert(' ', "88".to_string());
        base_map.insert('1', "96".to_string());
        base_map.insert('2', "97".to_string());
        base_map.insert('3', "98".to_string());
        base_map.insert('4', "99".to_string());
        base_map.insert('5', "90".to_string());
        base_map.insert('6', "06".to_string());
        base_map.insert('7', "07".to_string());
        base_map.insert('8', "08".to_string());
        base_map.insert('9', "09".to_string());
        base_map.insert('0', "00".to_string());

        base_map.insert('が', "2104".to_string());
        base_map.insert('ぎ', "2204".to_string());
        base_map.insert('ぐ', "2304".to_string());
        base_map.insert('げ', "2404".to_string());
        base_map.insert('ご', "2504".to_string());
        base_map.insert('ざ', "3104".to_string());
        base_map.insert('じ', "3204".to_string());
        base_map.insert('ず', "3304".to_string());
        base_map.insert('ぜ', "3404".to_string());
        base_map.insert('ぞ', "3504".to_string());
        base_map.insert('だ', "4104".to_string());
        base_map.insert('ぢ', "4204".to_string());
        base_map.insert('づ', "4304".to_string());
        base_map.insert('で', "4404".to_string());
        base_map.insert('ど', "4504".to_string());
        base_map.insert('ば', "6104".to_string());
        base_map.insert('び', "6204".to_string());
        base_map.insert('ぶ', "6304".to_string());
        base_map.insert('べ', "6404".to_string());
        base_map.insert('ぼ', "6504".to_string());

        base_map.insert('ぱ', "6105".to_string());
        base_map.insert('ぴ', "6205".to_string());
        base_map.insert('ぷ', "6305".to_string());
        base_map.insert('ぺ', "6405".to_string());
        base_map.insert('ぽ', "6505".to_string());

        let mut inversed_base_map = HashMap::with_capacity(base_map.len());
        for (key, value) in &base_map {
            inversed_base_map.insert(value.clone(), key.clone());
        }

        let mut normalization_map = HashMap::new();
        normalization_map.insert('ぁ', 'あ');
        normalization_map.insert('ぃ', 'い');
        normalization_map.insert('ぅ', 'う');
        normalization_map.insert('ぇ', 'え');
        normalization_map.insert('ぉ', 'お');
        normalization_map.insert('っ', 'つ');
        normalization_map.insert('ゃ', 'や');
        normalization_map.insert('ゅ', 'ゆ');
        normalization_map.insert('ょ', 'よ');
        normalization_map.insert('（', '(');
        normalization_map.insert('）', ')');
        normalization_map.insert('Ａ', 'A');
        normalization_map.insert('Ｂ', 'B');
        normalization_map.insert('Ｃ', 'C');
        normalization_map.insert('Ｄ', 'D');
        normalization_map.insert('Ｅ', 'E');
        normalization_map.insert('Ｆ', 'F');
        normalization_map.insert('Ｇ', 'G');
        normalization_map.insert('Ｈ', 'H');
        normalization_map.insert('Ｉ', 'I');
        normalization_map.insert('Ｊ', 'J');
        normalization_map.insert('Ｋ', 'J');
        normalization_map.insert('Ｌ', 'L');
        normalization_map.insert('Ｍ', 'M');
        normalization_map.insert('Ｎ', 'N');
        normalization_map.insert('Ｏ', 'O');
        normalization_map.insert('Ｐ', 'P');
        normalization_map.insert('Ｑ', 'Q');
        normalization_map.insert('Ｒ', 'R');
        normalization_map.insert('Ｓ', 'S');
        normalization_map.insert('Ｔ', 'T');
        normalization_map.insert('Ｕ', 'U');
        normalization_map.insert('Ｖ', 'V');
        normalization_map.insert('Ｗ', 'W');
        normalization_map.insert('Ｘ', 'X');
        normalization_map.insert('Ｙ', 'Y');
        normalization_map.insert('Ｚ', 'Z');
        normalization_map.insert('？', '?');
        normalization_map.insert('！', '!');
        normalization_map.insert('－', '-');
        normalization_map.insert('／', '/');
        normalization_map.insert('￥', '\\');
        normalization_map.insert('＆', '&');
        normalization_map.insert('＊', '*');
        normalization_map.insert('＃', '#');
        normalization_map.insert('　', ' ');
        normalization_map.insert('１', '1');
        normalization_map.insert('２', '2');
        normalization_map.insert('３', '3');
        normalization_map.insert('４', '4');
        normalization_map.insert('５', '5');
        normalization_map.insert('６', '6');
        normalization_map.insert('７', '7');
        normalization_map.insert('８', '8');
        normalization_map.insert('９', '9');
        normalization_map.insert('０', '0');
        normalization_map.insert('ー', '-');

        // see https://koma-yome.com/archives/724
        let mut reserved_word_map = HashMap::new();
        reserved_word_map.insert("今".to_string(), vec!["10".to_string()]);
        reserved_word_map.insert("いま".to_string(), vec!["10".to_string()]);
        reserved_word_map.insert("海".to_string(), vec!["41".to_string()]);
        reserved_word_map.insert("うみ".to_string(), vec!["41".to_string()]);
        reserved_word_map.insert("シー".to_string(), vec!["41".to_string()]);
        reserved_word_map.insert("しー".to_string(), vec!["41".to_string()]);
        reserved_word_map.insert("至急".to_string(), vec!["49".to_string()]);
        reserved_word_map.insert("しきゅう".to_string(), vec!["49".to_string()]);
        reserved_word_map.insert("待ってる".to_string(), vec!["106".to_string()]);
        reserved_word_map.insert("まってる".to_string(), vec!["106".to_string()]);
        reserved_word_map.insert("TEL".to_string(), vec!["106".to_string()]);
        reserved_word_map.insert("ＴＥＬ".to_string(), vec!["106".to_string()]);
        reserved_word_map.insert("テル".to_string(), vec!["106".to_string()]);
        reserved_word_map.insert("遅れてる".to_string(), vec!["9106".to_string()]);
        reserved_word_map.insert("おくれてる".to_string(), vec!["9106".to_string()]);
        reserved_word_map.insert(
            "愛してる".to_string(),
            vec![
                "14106".to_string(),
                "114106".to_string(),
                "1410".to_string(),
            ],
        );
        reserved_word_map.insert(
            "あいしてる".to_string(),
            vec![
                "14106".to_string(),
                "114106".to_string(),
                "1410".to_string(),
            ],
        );
        reserved_word_map.insert("何してる".to_string(), vec!["724106".to_string()]);
        reserved_word_map.insert("なにしてる".to_string(), vec!["724106".to_string()]);
        reserved_word_map.insert(
            "起きてる".to_string(),
            vec!["09106".to_string(), "9106".to_string()],
        );
        reserved_word_map.insert(
            "おきてる".to_string(),
            vec!["09106".to_string(), "9106".to_string()],
        );
        reserved_word_map.insert("行くよ".to_string(), vec!["194".to_string()]);
        reserved_word_map.insert("いくよ".to_string(), vec!["194".to_string()]);
        reserved_word_map.insert("池袋".to_string(), vec!["269".to_string()]);
        reserved_word_map.insert("いけぶくろ".to_string(), vec!["269".to_string()]);
        reserved_word_map.insert("渋谷".to_string(), vec!["428".to_string()]);
        reserved_word_map.insert("しぶや".to_string(), vec!["428".to_string()]);
        reserved_word_map.insert("おやすみ".to_string(), vec!["833".to_string()]);
        reserved_word_map.insert(
            "おはよう".to_string(),
            vec!["840".to_string(), "0840".to_string()],
        );
        reserved_word_map.insert("ハロー".to_string(), vec!["860".to_string()]);
        reserved_word_map.insert("はやく".to_string(), vec!["889".to_string()]);
        reserved_word_map.insert("早く".to_string(), vec!["889".to_string()]);
        reserved_word_map.insert(
            "サンキュー".to_string(),
            vec!["39".to_string(), "999".to_string()],
        );
        reserved_word_map.insert(
            "Thank you".to_string(),
            vec!["39".to_string(), "999".to_string()],
        );
        reserved_word_map.insert(
            "thank you".to_string(),
            vec!["39".to_string(), "999".to_string()],
        );
        reserved_word_map.insert("会えない".to_string(), vec!["1871".to_string()]);
        reserved_word_map.insert("あえない".to_string(), vec!["1871".to_string()]);
        reserved_word_map.insert("さよなら".to_string(), vec!["3470".to_string()]);
        reserved_word_map.insert("寒いよ".to_string(), vec!["3614".to_string()]);
        reserved_word_map.insert("さむいよ".to_string(), vec!["3614".to_string()]);
        reserved_word_map.insert("仕事".to_string(), vec!["4510".to_string()]);
        reserved_word_map.insert("しごと".to_string(), vec!["4510".to_string()]);
        reserved_word_map.insert("横浜".to_string(), vec!["4580".to_string()]);
        reserved_word_map.insert("よこはま".to_string(), vec!["4580".to_string()]);
        reserved_word_map.insert("よろしく".to_string(), vec!["4649".to_string()]);
        reserved_word_map.insert("ファイト".to_string(), vec!["5110".to_string()]);
        reserved_word_map.insert("お仕事ファイト".to_string(), vec!["045105110".to_string()]);
        reserved_word_map.insert("おしごとふぁいと".to_string(), vec!["045105110".to_string()]);
        reserved_word_map.insert("ふぁいと".to_string(), vec!["5110".to_string()]);
        reserved_word_map.insert("ご苦労さん".to_string(), vec!["5963".to_string()]);
        reserved_word_map.insert("ごくろうさん".to_string(), vec!["5963".to_string()]);
        reserved_word_map.insert("バイト".to_string(), vec!["8110".to_string()]);
        reserved_word_map.insert("ばいと".to_string(), vec!["8110".to_string()]);
        reserved_word_map.insert("バイバイ".to_string(), vec!["8181".to_string()]);
        reserved_word_map.insert("ばいばい".to_string(), vec!["8181".to_string()]);
        reserved_word_map.insert("今どこ".to_string(), vec!["10105".to_string()]);
        reserved_word_map.insert("いまどこ".to_string(), vec!["10105".to_string()]);
        reserved_word_map.insert("会いたいよ".to_string(), vec!["110149".to_string()]);
        reserved_word_map.insert("あいたいよ".to_string(), vec!["11014".to_string()]);
        reserved_word_map.insert("着いたよ".to_string(), vec!["21104".to_string()]);
        reserved_word_map.insert("ついたよ".to_string(), vec!["21104".to_string()]);
        reserved_word_map.insert("寂しいよ".to_string(), vec!["33414".to_string()]);
        reserved_word_map.insert("さびしいよ".to_string(), vec!["33414".to_string()]);
        reserved_word_map.insert("デートしよ".to_string(), vec!["101044".to_string()]);
        reserved_word_map.insert("でーとしよ".to_string(), vec!["101044".to_string()]);
        reserved_word_map.insert("TEL欲しい".to_string(), vec!["106841".to_string()]);
        reserved_word_map.insert("TELほしい".to_string(), vec!["106841".to_string()]);
        reserved_word_map.insert("ごめんなさい".to_string(), vec!["500731".to_string()]);
        reserved_word_map.insert("早くして".to_string(), vec!["889410".to_string()]);
        reserved_word_map.insert("はやくして".to_string(), vec!["889410".to_string()]);
        reserved_word_map.insert("どこにいるの".to_string(), vec!["1052167".to_string()]);
        reserved_word_map.insert("今から行くよ".to_string(), vec!["1056194".to_string()]);
        reserved_word_map.insert("いまからいくよ".to_string(), vec!["1056194".to_string()]);
        reserved_word_map.insert("ボウリング行こ".to_string(), vec!["015".to_string()]);
        reserved_word_map.insert("ボウリングいこ".to_string(), vec!["015".to_string()]);
        reserved_word_map.insert("遅れる".to_string(), vec!["090".to_string()]);
        reserved_word_map.insert("おくれる".to_string(), vec!["090".to_string()]);
        reserved_word_map.insert(
            "ずっと一緒にいようね".to_string(),
            vec!["2101442147".to_string()],
        );
        reserved_word_map.insert(
            "ずっと一緒にいよーね".to_string(),
            vec!["21014421479".to_string()],
        );
        reserved_word_map.insert(
            "ずっといっしょにいようね".to_string(),
            vec!["2101442147".to_string()],
        );
        reserved_word_map.insert(
            "ずっといっしょにいよーね".to_string(),
            vec!["21014421479".to_string()],
        );

        Converter {
            base_map: base_map,
            inversed_base_map: inversed_base_map,
            normalization_map: normalization_map,
            reserved_word_map: reserved_word_map,
        }
    }
}

#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "parse error")]
    ParseError,
}

/* ----------- failure boilerplate ----------- */

use failure::{Backtrace, Context, Fail};
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn new(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }

    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert_to_two_touch_string_normal() {
        let c = Converter::new();
        let result = c
            .convert_to_two_touch_string("こんにちは".to_string())
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "2503524261");

        let result = c
            .convert_to_two_touch_string("ごくろうさん".to_string())
            .unwrap();
        let expected = vec!["5963".to_string(), "25042395133103".to_string()];
        assert_eq!(result, expected);

        let result = c
            .convert_to_two_touch_string("ご苦労さん".to_string())
            .unwrap();
        let expected = vec!["5963".to_string()];
        assert_eq!(result, expected);

        let result = c
            .convert_to_two_touch_string("だA*（￥ぽＧ".to_string())
            .unwrap();
        let expected = vec!["410416868276650527".to_string()];
        assert_eq!(result, expected);
    }
    #[test]
    fn test_convert_to_two_touch_string_error() {
        let c = Converter::new();
        let result = c.convert_to_two_touch_string("筋肉".to_string());
        assert!(result.is_err());
        let result = c.convert_to_two_touch_string("".to_string());
        assert!(result.is_err());
        let result = c.convert_to_two_touch_string("@".to_string());
        assert!(result.is_err());
    }
    #[test]
    fn test_convert_from_two_touch_string_normal() {
        let c = Converter::new();
        let result = c
            .convert_from_two_touch_string("48564940".to_string())
            .unwrap();
        assert_eq!(result, "RUST");
        let result = c
            .convert_from_two_touch_string("81225223".to_string())
            .unwrap();
        assert_eq!(result, "やきにく");
        let result = c
            .convert_from_two_touch_string("250459868884".to_string())
            .unwrap();
        assert_eq!(result, "こ゛X* )");
    }

    #[test]
    fn test_convert_from_two_touch_string_error() {
        let c = Converter::new();
        let result = c.convert_from_two_touch_string("8080".to_string());
        assert!(result.is_err());
        let c = Converter::new();
        let result = c.convert_from_two_touch_string("111".to_string());
        assert!(result.is_err());
        let c = Converter::new();
        let result = c.convert_from_two_touch_string("".to_string());
        assert!(result.is_err());
        let c = Converter::new();
        let result = c.convert_from_two_touch_string("筋肉".to_string());
        assert!(result.is_err());
    }
}
