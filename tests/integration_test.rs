use std::{
    io::{Error, ErrorKind, Result},
    process::Command,
};

#[test]
fn case_insensitive() -> Result<()> {
    Ok({
        // Arrange
        let query = "to";
        let file_path = "poem.txt";
        let case_sensitive = "0";

        // Action
        let mini_grep = Command::new("cargo")
            .args(["run", "--", query, file_path])
            .env("CASE_SENSITIVE", case_sensitive)
            .output()?;
        let stdout = match String::from_utf8(mini_grep.stdout) {
            Err(x) => return Err(Error::new(ErrorKind::InvalidData, x)),
            Ok(s) => s,
        };
        let exit_status = mini_grep.status;

        // Assert
        let actual: Vec<&str> = stdout.lines().collect();
        let expect = vec![
            "Are you nobody, too?",
            "How dreary to be somebody!",
            "To tell your name the livelong day",
            "To an admiring bog!",
        ];
        assert_eq!(actual, expect);
        assert!(exit_status.success());
    })
}

#[test]
fn case_sensitive() -> Result<()> {
    Ok({
        // Arrange
        let query = "to";
        let file_path = "poem.txt";
        let case_sensitive = "1";

        // Action
        let mini_grep = Command::new("cargo")
            .args(["run", "--", query, file_path])
            .env("CASE_SENSITIVE", case_sensitive)
            .output()?;
        let stdout = match String::from_utf8(mini_grep.stdout) {
            Err(x) => return Err(Error::new(ErrorKind::InvalidData, x)),
            Ok(s) => s,
        };
        let exit_status = mini_grep.status;

        // Assert
        let actual: Vec<&str> = stdout.lines().collect();
        let expect = vec!["Are you nobody, too?", "How dreary to be somebody!"];
        assert_eq!(actual, expect);
        assert!(exit_status.success());
    })
}

#[test]
fn without_environment_variable() -> Result<()> {
    Ok({
        // Arrange
        let query = "to";
        let file_path = "poem.txt";

        // Action
        let mini_grep = Command::new("cargo")
            .args(["run", "--", query, file_path])
            .output()?;
        let stderr = match String::from_utf8(mini_grep.stderr) {
            Err(x) => return Err(Error::new(ErrorKind::InvalidData, x)),
            Ok(s) => s,
        };
        let exit_status = mini_grep.status;

        // Assert
        assert!(stderr.contains(
            r#"CASE_SENSITIVE environment variable not found, defaulting to CASE_INSENSITIVE
run with `CASE_SENSITIVE=1` to search with case sensitivity"#
        ));
        assert!(exit_status.success());
    })
}

#[test]
fn invalid_arguments() -> Result<()> {
    Ok({
        let mini_grep = Command::new("cargo").arg("run").output()?;
        let stderr = match String::from_utf8(mini_grep.stderr) {
            Err(x) => return Err(Error::new(ErrorKind::InvalidData, x)),
            Ok(s) => s,
        };
        let exit_status = mini_grep.status;

        assert!(stderr.contains(r#"Valid arguments are "Query" "File Path""#));
        assert!(!exit_status.success());
    })
}
