use file_access::as_file::*;
use std::io::Result;

type Lines = Vec<String>;
pub fn mini_grep<Query: AsRef<str>, Path: AsRef<str>>(
    query: &Query,
    file_path: &Path,
    case_sensitive: bool,
) -> Result<Lines> {
    let query = query.as_ref();
    let file_path = file_path.as_ref();

    let mut lines = vec![];
    for line in file_path.as_file().read_lines()? {
        let normalized_line = if case_sensitive {
            format!("{line}") // borrow {line}
        } else {
            format!("{}", line.to_uppercase())
        };
        let normalized_query = if case_sensitive {
            query.to_string()
        } else {
            query.to_uppercase()
        };

        if normalized_line.contains(&normalized_query) {
            lines.push(line);
        }
    }

    return Ok(lines);
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
            assert!(
                search_result
                    .iter()
                    .all(|hit| hit.to_uppercase().contains(&query.to_uppercase())),
                "all must contain query"
            );
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
            assert!(
                search_result.iter().all(|hit| hit.contains(&query)),
                "all must contain query"
            );
        })
    }
}
