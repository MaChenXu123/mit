use std::collections::BTreeMap;
use std::borrow::Cow;

struct TempComparisonStruct<'a, 'b> {
    old_file: Option<Cow<'a, str>>,
    new_file: Option<Cow<'a, str>>,
    old_char: Option<&'b str>,
    new_char: Option<&'b str>,
}

fn file_comparison(old_file: String, new_file: String) -> Option<String> {
    let mut temp_com_struct = TempComparisonStruct {
        old_file: None,
        new_file: None,
        old_char: None,
        new_char: None,
    };
    let mut comparison_line: Vec<i64> = Vec::new();
    let long_str: Cow<str>;
    let small_str: Cow<str>;

    if old_file.lines().count() > new_file.lines().count() {
        long_str = Cow::Borrowed(&old_file);
        small_str = Cow::Borrowed(&new_file);
        temp_com_struct.old_file = Some(long_str.clone());
        temp_com_struct.new_file = Some(small_str.clone());
    } else {
        long_str = Cow::Borrowed(&new_file);
        small_str = Cow::Borrowed(&old_file);
        temp_com_struct.old_file = Some(small_str.clone());
        temp_com_struct.new_file = Some(long_str.clone());
    };

    for line in 0..long_str.lines().count() {
        if let Some(small_line) = small_str.lines().nth(line) {
            if small_line.is_empty() {
                comparison_line.push(line as i64);
                println!("第{}行不相同s1为{},s2为{}", line, long_str.lines().nth(line).unwrap(), "");
                continue;
            }
        } else {
            comparison_line.push(line as i64);
            println!("第{}行不相同s1为{},s2为{}", line, long_str.lines().nth(line).unwrap(), "");
            continue;
        }

        if long_str.lines().nth(line) != Some(small_str.lines().nth(line).unwrap()) {
            comparison_line.push(line as i64);
            let mut old_char_and_new_char: BTreeMap<i64, (char, char)> = BTreeMap::new();
            let long_chars = long_str.lines().nth(line).unwrap().chars();
            let small_chars = small_str.lines().nth(line).unwrap().chars();

            temp_com_struct.old_char = Some(long_str.lines().nth(line).unwrap());
            temp_com_struct.new_char = Some(small_str.lines().nth(line).unwrap());

            for (cline, (old_char, new_char)) in long_chars.zip(small_chars).enumerate() {
                if old_char == new_char {
                    continue;
                } else {
                    println!(
                        "第{}行第{}个字符不相同old is {},small is {}",
                        line, cline, old_char, new_char
                    );
                }
            }
        }
    }
    println!("{:?}", comparison_line);
    None
}

#[test]
fn test() {
    let _result = file_comparison("Hello\nWorld".to_string(), "a\nRust".to_string());
}
