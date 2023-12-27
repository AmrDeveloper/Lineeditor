/// Input filter used to ignore any input character that not matching the rules
/// You can Mix one or more rules to make your own custom rules
///
/// ## Examples of mixed rules
///
/// ```
/// // To Make rules that accept any alphabetic character or whitespace
/// InputFilter::Options(vec![Box::new(InputFilter::Alphabetic), Box::new(InputFilter::Whitespace)])
///
/// // To Make rules that accept any Text but not accept Punctuation
/// InputFilter::Options(vec![Box::new(InputFilter::Alphabetic), Box::new(Not(Box::new(InputFilter::Punctuation)))])
/// ```
///
pub enum InputFilter {
    /// A-Z and a-z
    Alphabetic,
    /// A-Z, a-z and 0-9
    AlphaNumeric,
    /// 0-9
    Digit,
    /// Any character
    Text,
    /// a-f, A-F and 0-9
    HexDigit,
    /// Whitespace
    Whitespace,
    /// Punctuation
    Punctuation,
    /// Allow everything except One InputFilter
    Not(Box<InputFilter>),
    /// Valid if one of the char is matching at least one of the InputFilters
    Options(Vec<Box<InputFilter>>),
    /// User defined input filter function
    Custom(fn(char) -> bool),
}

/// Input Filter function that returns true if character is matching the rules of the given InputFilter
pub fn filter_input(ch: char, input_filter: &InputFilter) -> bool {
    match input_filter {
        InputFilter::Alphabetic => ch.is_alphabetic(),
        InputFilter::AlphaNumeric => ch.is_alphanumeric(),
        InputFilter::Text => true,
        InputFilter::Digit => ch.is_numeric(),
        InputFilter::HexDigit => ch.is_ascii_hexdigit(),
        InputFilter::Whitespace => ch.is_whitespace(),
        InputFilter::Punctuation => ch.is_ascii_punctuation(),
        InputFilter::Not(filter) => !filter_input(ch, filter),
        InputFilter::Options(input_filters) => {
            for filter in input_filters {
                if filter_input(ch, &filter) {
                    return true;
                }
            }
            false
        }
        InputFilter::Custom(function) => function(ch),
    }
}
