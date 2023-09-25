#[cfg(test)]
mod tests {

    use fuels::prelude::{setup_program_test, *};

    #[tokio::test]
    async fn revert_issue() {
        setup_program_test!(
            Wallets("wallet"),
            Abigen(Script(name = "MyScript", project = "script")),
            LoadScript(
                name = "script_instance",
                script = "MyScript",
                wallet = "wallet"
            )
        );

        let Error::RevertTransactionError {
            reason, receipts, ..
        } = script_instance
            .main()
            .call()
            .await
            .expect_err("Should have reverted")
        else {
            panic!("Expected RevertTransactionError!");
        };

        dbg!(&reason);
        dbg!(&receipts);

        let last_log_data = receipts
            .iter()
            .filter_map(|r| match r {
                fuels::tx::Receipt::Log { ra, .. } => Some(ra.to_be_bytes().to_vec()),
                fuels::tx::Receipt::LogData { data, .. } => Some(data.as_ref().unwrap().clone()),
                _ => None,
            })
            .last()
            .unwrap();

        let expected_str_data = vec![
            0x53, 0x6f, 0x6d, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
        ];
        assert_eq!(
            expected_str_data,
            "Some message".to_string().bytes().collect::<Vec<u8>>()
        );
        assert_eq!(last_log_data, expected_str_data);
    }
}
