#[macro_use]
extern crate serde_json;

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use serde_pickle as pickle;
    use std::fs::File;
    use toml;

    #[test]
    fn test_dynamic_json() {
        let j = r#"{
            "userid": 103609,
            "verified": true,
            "friendly_name": "Jason",
            "access_privileges": [
              "user",
              "admin"
            ]
        }"#;

        let parsed: Value = serde_json::from_str(j).unwrap();
        let expected = json!({
          "userid": 103609,
          "verified": true,
          "friendly_name": "Jason",
          "access_privileges": [
            "user",
            "admin"
          ]
        });
        assert_eq!(parsed, expected);

        assert_eq!(parsed["userid"], 103609);
        assert_eq!(parsed["verified"], true);
        assert_eq!(parsed["friendly_name"], "Jason");
        assert_eq!(parsed["access_privileges"][0], "user");
        assert_eq!(parsed["access_privileges"][1], "admin");
        assert_eq!(parsed["access_privileges"][2], Value::Null);
        assert_eq!(parsed["not-available"], Value::Null);
    }

    #[test]
    fn test_dynamic_pickle() {
        let parsed: Value =  { 
            let data = File::open("user.pkl").expect("Did you run create_pickle.py?");
            pickle::from_reader(&data).unwrap()
        };

        let expected = json!({
          "userid": 103609,
          "verified": true,
          "friendly_name": "Jason",
          "access_privileges": [
            "user",
            "admin"
          ]
        });
        assert_eq!(parsed, expected);

        assert_eq!(parsed["userid"], 103609);
        assert_eq!(parsed["verified"], true);
        assert_eq!(parsed["friendly_name"], "Jason");
        assert_eq!(parsed["access_privileges"][0], "user");
        assert_eq!(parsed["access_privileges"][1], "admin");
        assert_eq!(parsed["access_privileges"][2], Value::Null);
        assert_eq!(parsed["not-available"], Value::Null);
    }

    #[test]
    fn test_dynamic_toml() {
        let t = r#"
            [[user]]
            userid = 103609
            verified = true
            friendly_name = "Jason"
            access_privileges = [ "user", "admin" ]
        "#;

        let parsed: Value = toml::de::from_str(t).unwrap();

        let expected = json!({
            "user": [
                {
                    "userid": 103609,
                    "verified": true,
                    "friendly_name": "Jason",
                    "access_privileges": [
                        "user",
                        "admin"
                    ]
                }

            ]
        });
        assert_eq!(parsed, expected);

        let first_user = &parsed["user"][0];
        assert_eq!(first_user["userid"], 103609);
        assert_eq!(first_user["verified"], true);
        assert_eq!(first_user["friendly_name"], "Jason");
        assert_eq!(first_user["access_privileges"][0], "user");
        assert_eq!(first_user["access_privileges"][1], "admin");
        assert_eq!(first_user["access_privileges"][2], Value::Null);
        assert_eq!(first_user["not-available"], Value::Null);
    }

    
}
