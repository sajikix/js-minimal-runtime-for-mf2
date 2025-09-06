// fn is_quoted_char(c: char) -> bool {
//     match c {
//         '\u{01}'..='\u{5B}' => true, // omit NULL (%x00) and \ (%x5C)
//         '\u{5D}'..='\u{7B}' => true, // omit | (%x7C)
//         '\u{7D}'..='\u{10FFFF}' => true,
//         _ => false,
//     }
// }

// pub fn is_simple_start_char(c: char) -> bool {
//     match c {
//         '\u{01}'..='\u{08}' => true, // mit NULL (%x00), HTAB (%x09) and LF (%x0A)
//         '\u{0B}'..='\u{0C}' => true, // omit CR (%x0D)
//         '\u{0E}'..='\u{1F}' => true, // omit SP (%x20)
//         '\u{21}'..='\u{2D}' => true, // omit . (%x2E)
//         '\u{2F}'..='\u{5B}' => true, // omit \ (%x5C)
//         '\u{5D}'..='\u{7A}' => true, // omit { (%x7B)
//         '\u{7c}' => true,            // omit } (%x7D)
//         '\u{7E}'..='\u{2FFF}' => true, // omit IDEOGRAPHIC SPACE (%x3000)
//         '\u{3001}'..='\u{10FFFF}' => true, // allowing surrogates is intentional
//         _ => false,
//     }
// }

// fn is_text_char(c: char) -> bool {
//     match c {
//         '\u{01}'..='\u{5B}' => true,     // omit NULL (%x00) and \ (%x5C)
//         '\u{5D}'..='\u{7A}' => true,     // omit { (%x7B)
//         '\u{7c}' => true,                // omit } (%x7D)
//         '\u{7E}'..='\u{10FFFF}' => true, // allowing surrogates is intentional
//         _ => false,
//     }
// }

fn is_alpha_char(c: char) -> bool {
    match c {
        '\u{41}'..='\u{5A}' => true, // A-Z
        '\u{61}'..='\u{7A}' => true, // a-z
        _ => false,
    }
}

fn is_digit_char(c: char) -> bool {
    match c {
        '\u{30}'..='\u{39}' => true, // 0-9
        _ => false,
    }
}

pub fn is_name_start_char(c: char) -> bool {
    match c {
        '+' => true,                         // %x2B
        '_' => true,                         // %x5F
        '\u{A1}'..='\u{61B}' => true,        // %xA1-61B
        '\u{61D}'..='\u{167F}' => true,      // %x61D-167F
        '\u{1681}'..='\u{1FFF}' => true,     // %x1681-1FFF
        '\u{200B}'..='\u{200D}' => true,     // %x200B-200D
        '\u{2010}'..='\u{2027}' => true,     // %x2010-2027
        '\u{2030}'..='\u{205E}' => true,     // %x2030-205E
        '\u{2060}'..='\u{2065}' => true,     // %x2060-2065
        '\u{206A}'..='\u{2FFF}' => true,     // %x206A-2FFF
        '\u{3001}'..='\u{D7FF}' => true,     // %x3001-D7FF
        '\u{E000}'..='\u{FDCF}' => true,     // %xE000-FDCF
        '\u{FDF0}'..='\u{FFFD}' => true,     // %xFDF0-FFFD
        '\u{10000}'..='\u{1FFFD}' => true,   // %x10000-1FFFD
        '\u{20000}'..='\u{2FFFD}' => true,   // %x20000-2FFFD
        '\u{30000}'..='\u{3FFFD}' => true,   // %x30000-3FFFD
        '\u{40000}'..='\u{4FFFD}' => true,   // %x40000-4FFFD
        '\u{50000}'..='\u{5FFFD}' => true,   // %x50000-5FFFD
        '\u{60000}'..='\u{6FFFD}' => true,   // %x60000-6FFFD
        '\u{70000}'..='\u{7FFFD}' => true,   // %x70000-7FFFD
        '\u{80000}'..='\u{8FFFD}' => true,   // %x80000-8FFFD
        '\u{90000}'..='\u{9FFFD}' => true,   // %x90000-9FFFD
        '\u{A0000}'..='\u{AFFFD}' => true,   // %xA0000-AFFFD
        '\u{B0000}'..='\u{BFFFD}' => true,   // %xB0000-BFFFD
        '\u{C0000}'..='\u{CFFFD}' => true,   // %xC0000-CFFFD
        '\u{D0000}'..='\u{DFFFD}' => true,   // %xD0000-DFFFD
        '\u{E0000}'..='\u{EFFFD}' => true,   // %xE0000-EFFFD
        '\u{F0000}'..='\u{FFFFD}' => true,   // %xF0000-FFFFD
        '\u{100000}'..='\u{10FFFD}' => true, // %x100000-10FFFD
        _ => is_alpha_char(c),               // ALPHA
    }
}

pub fn is_name_char(c: char) -> bool {
    match c {
        '-' | '.' => true,
        _ => is_name_start_char(c) | is_digit_char(c),
    }
}

pub fn is_ws_char(c: char) -> bool {
    match c {
        '\u{20}' | '\u{09}' | '\u{0D}' | '\u{0A}' | '\u{3000}' => true,
        _ => false,
    }
}

pub fn is_bidi_char(c: char) -> bool {
    match c {
        '\u{061c}' | '\u{200E}' | '\u{200F}' => true,
        '\u{2066}'..='\u{2069}' => true,
        _ => false,
    }
}

// TODO use this validation in parser
// pub fn is_valid_quoted_literal_string(s: &str) -> bool {
//     let mut escaped = false;
//     for c in s.chars() {
//         match c {
//             '|' => {
//                 if !escaped {
//                     return false;
//                 }
//                 escaped = false;
//                 continue;
//             }
//             '\u{5C}' => {
//                 escaped = !escaped;
//             }
//             _ => {
//                 if escaped {
//                     // '{' and is allowed for escaped char
//                     if c == '{' || c == '}' {
//                         escaped = false;
//                         continue;
//                     }
//                     return false;
//                 }
//                 if !is_quoted_char(c) {
//                     return false;
//                 }
//                 continue;
//             }
//         }
//     }
//     return !escaped;
// }

// TODO use this validation in parser
// pub fn is_valid_text_string(s: &str) -> bool {
//     let mut escaped = false;
//     for c in s.chars() {
//         match c {
//             '{' | '}' => {
//                 if !escaped {
//                     return false;
//                 }
//                 escaped = false;
//                 continue;
//             }
//             '\u{5C}' => {
//                 escaped = !escaped;
//             }
//             _ => {
//                 if escaped {
//                     // '|' is allowed for escaped char
//                     if c == '|' {
//                         escaped = false;
//                         continue;
//                     }
//                     return false;
//                 }
//                 if !is_text_char(c) {
//                     return false;
//                 }
//                 continue;
//             }
//         }
//     }
//     return !escaped;
// }

pub fn is_valid_name_string(s: &str) -> bool {
    let mut initial_bidi = true;
    let mut tail_bidi = false;
    let mut is_initital_name_char = true;
    for c in s.chars() {
        if is_bidi_char(c) {
            if !initial_bidi & !tail_bidi {
                return false;
            }
            if !initial_bidi {
                tail_bidi = true;
            }
        } else {
            initial_bidi = false;
            if tail_bidi {
                // bidi char is not allowed in the middle of name
                return false;
            }
            if is_initital_name_char {
                if !is_name_start_char(c) {
                    return false;
                }
                is_initital_name_char = false;
            } else {
                if !is_name_char(c) {
                    return false;
                }
            }
        }
    }
    return true;
}

// TODO use this validation in parser
// pub fn is_valid_unquoted_literal_string(s: &str) -> bool {
//     for c in s.chars() {
//         if !is_name_char(c) {
//             return false;
//         }
//     }
//     return true;
// }

pub fn trim_tail_ws_and_bidi(s: &str) -> String {
    let mut i = s.chars().count();
    loop {
        if i == 0 {
            break;
        }
        let c = s.chars().nth(i - 1).unwrap();
        if !is_ws_char(c) & !is_bidi_char(c) {
            break;
        }
        i -= 1;
    }
    s[..i].to_string()
}
