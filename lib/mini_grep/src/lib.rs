use file_access::as_file::*;
use std::io::Result;

type Lines = Vec<String>;
pub fn mini_grep<Query: AsRef<str>, Path: AsRef<str>>(
    query: &Query,
    file_path: &Path,
    case_sensitive: bool,
) -> Result<Lines> {
    let query = if case_sensitive {
        query.as_ref().to_string()
    } else {
        query.as_ref().to_uppercase()
    };

    return Ok(file_path
        .as_file()
        .read_lines()?
        .into_iter()
        .filter(|line| {
            if case_sensitive {
                line.to_string()
            } else {
                line.to_uppercase()
            }
            .contains(&query)
        })
        .collect());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_insensitive() -> Result<()> {
        Ok({
            // Arrange
            let query = "to";
            let file_path = "../../poem.txt";

            // Action
            let search_result = mini_grep(&query, &file_path, false)?;

            // Assert
            let expect = vec![
                "Are you nobody, too?",
                "How dreary to be somebody!",
                "To tell your name the livelong day",
                "To an admiring bog!",
            ];
            assert_eq!(search_result, expect);
        })
    }

    #[test]
    fn case_sensitive() -> Result<()> {
        Ok({
            // Arrange
            let query = "to";
            let file_path = "../../poem.txt";

            // Action
            let search_result = mini_grep(&query, &file_path, true)?;

            // Assert
            let expect = vec!["Are you nobody, too?", "How dreary to be somebody!"];
            assert_eq!(search_result, expect);
        })
    }
}
