pub fn get_padding(string:&str) -> String {
    let mut output = String::new();

    for c in string.chars() {
        if c != ' ' && c != '\t'{
            break;
        } else {
            output.push(c);
        }
    }

    output
}

#[cfg(test)]
mod test {
    use super::get_padding;

    #[test]
    fn get_padding_test_1() {
        assert_eq!(
            get_padding("".into()),
            "".to_string()   
        );
    }

    #[test]
    fn get_padding_test_2() {
        assert_eq!(
            get_padding(" ".into()),
            " ".to_string()   
        );
    }

    #[test]
    fn get_padding_test_3() {
        assert_eq!(
            get_padding("\t".into()),
            "\t".to_string()   
        );
    }

    #[test]
    fn get_padding_test_4() {
        assert_eq!(
            get_padding("\t ".into()),
            "\t ".to_string()   
        );
    }

    #[test]
    fn get_padding_test_5() {
        assert_eq!(
            get_padding("\t   \t\t \t \t  x\t \t ".into()),
            "\t   \t\t \t \t  ".to_string()   
        );
    }
}