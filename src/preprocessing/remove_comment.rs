use std::iter;

enum CommentState {
    NotComment,
    Slash,
    BlockInner,
    BlockInnerStar,
    BlockEnd,
    LineInner,
    LineEnd,
}

pub(crate) fn remove_comment(input: &str) -> anyhow::Result<String> {
    use CommentState::*;
    let mut output = String::new();
    let mut state = CommentState::NotComment;
    let mut it = input.chars();
    let mut buffer = String::new();
    loop {
        if let Some(ch) = it.next() {
            state = match (state, ch) {
                (NotComment, '/') => Slash,
                (NotComment, _) => NotComment,
                (Slash, '/') => LineInner,
                (Slash, '*') => BlockInner,
                (Slash, _) => NotComment,
                (LineInner, '\n') => LineEnd,
                (LineInner, _) => LineInner,
                (LineEnd, '/') => Slash,
                (LineEnd, _) => NotComment,
                (BlockInner, '*') => BlockInnerStar,
                (BlockInner, _) => BlockInner,
                (BlockInnerStar, '/') => BlockEnd,
                (BlockInnerStar, _) => BlockInner,
                (BlockEnd, '/') => Slash,
                (BlockEnd, _) => NotComment,
            };
            match state {
                Slash => buffer.push(ch),
                NotComment | LineEnd => output.extend(buffer.drain(..).chain(iter::once(ch))),
                _ => buffer.clear(),
            }
        } else {
            break;
        }
    }
    output.extend(buffer.drain(..));
    match state {
        BlockInnerStar | BlockInner => {
            anyhow::bail!("Unfinished block comment")
        }
        _ => Ok(output),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserve_normal() {
        let test_codes = vec!["\n\n\n", "int a;", "a * b", "a / b", "/ *", "/ *", "/ /", "*/", "**"];
        for code in test_codes {
            assert_eq!(code, remove_comment(code).unwrap())
        }
    }

    #[test]
    fn remove_line_comment() {
        assert_eq!("", remove_comment("//").unwrap());
        assert_eq!("\n", remove_comment("//\n").unwrap());
        assert_eq!("", remove_comment("// //").unwrap());
        assert_eq!("", remove_comment("// /*").unwrap());
        assert_eq!("", remove_comment("// */").unwrap());
        assert_eq!("\n", remove_comment("// /*\n// */").unwrap());
    }

    #[test]
    fn remove_block_comment() {
        assert!(remove_comment("/*").is_err());
        assert!(remove_comment("/* \ntest\n").is_err());
        assert_eq!("", remove_comment("/**/").unwrap());
        assert_eq!("", remove_comment("/* test */").unwrap());
        assert_eq!("", remove_comment("/* \n\n\n */").unwrap());
        assert_eq!("", remove_comment("/* \n// \n// */").unwrap());
    }
}
