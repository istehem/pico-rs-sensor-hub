#[cfg(test)]
mod tests {
    use rstest::rstest;
    use tracing::info;

    #[rstest]
    #[test_log::test]
    fn test_join_strings() -> () {
        info!("Hello World!");
    }
}
