pub trait SmartContractInteractions {
    fn check_if_hash_valid(&self, file_hash: &str, account_id: &str) -> bool {
        let client = reqwest::blocking::Client::new();
        let body = base64_url::encode(&format!(
            "{{
            'hash': '{}'
        }}",
            file_hash
        ));
        let res = client
            .post("https://rpc.testnet.near.org")
            .body(format!(
                "{{
                'jsonrpc': '2.0',
                'id': 'dontcare',
                'method': 'query',
                'params': {{
                  'request_type': 'call_function',
                  'finality': 'final',
                  'account_id': '{}',
                  'method_name': 'check_if_hash_exists',
                  'args_base64': '{}'
                }}
              }}",
                account_id, body
            ))
            .send()
            .unwrap()
            .text()
            .unwrap();

        if res.contains("true") {
            return true;
        } else {
            return false;
        }
    }
}
