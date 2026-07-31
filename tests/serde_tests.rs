#[cfg(feature = "serde")]
mod serde_tests {
    use acton_ern::{Account, Category, Domain, EntityRoot, Ern, Part, Parts, SHA1Name};

    #[test]
    fn test_domain_serialization() {
        let domain = Domain::new("test-domain").unwrap();

        // Test JSON serialization
        let json = serde_json::to_string(&domain).unwrap();
        assert_eq!(json, "\"test-domain\"");

        // Test JSON deserialization
        let deserialized: Domain = serde_json::from_str(&json).unwrap();
        assert_eq!(domain, deserialized);

        // Test YAML serialization
        let yaml = serde_yaml::to_string(&domain).unwrap();
        assert_eq!(yaml.trim(), "test-domain");

        // Test YAML deserialization
        let deserialized: Domain = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(domain, deserialized);
    }

    #[test]
    fn test_category_serialization() {
        let category = Category::new("test-category").unwrap();

        // Test JSON serialization
        let json = serde_json::to_string(&category).unwrap();
        assert_eq!(json, "\"test-category\"");

        // Test JSON deserialization
        let deserialized: Category = serde_json::from_str(&json).unwrap();
        assert_eq!(category, deserialized);

        // Test YAML serialization
        let yaml = serde_yaml::to_string(&category).unwrap();
        assert_eq!(yaml.trim(), "test-category");

        // Test YAML deserialization
        let deserialized: Category = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(category, deserialized);
    }

    #[test]
    fn test_account_serialization() {
        let account = Account::new("test-account").unwrap();

        // Test JSON serialization
        let json = serde_json::to_string(&account).unwrap();
        assert_eq!(json, "\"test-account\"");

        // Test JSON deserialization
        let deserialized: Account = serde_json::from_str(&json).unwrap();
        assert_eq!(account, deserialized);

        // Test YAML serialization
        let yaml = serde_yaml::to_string(&account).unwrap();
        assert_eq!(yaml.trim(), "test-account");

        // Test YAML deserialization
        let deserialized: Account = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(account, deserialized);
    }

    #[test]
    fn test_entity_root_serialization() {
        let root = EntityRoot::new("test-root".to_string()).unwrap();

        // Test JSON serialization
        let json = serde_json::to_string(&root).unwrap();
        let deserialized: EntityRoot = serde_json::from_str(&json).unwrap();
        assert_eq!(root, deserialized);

        // Test YAML serialization
        let yaml = serde_yaml::to_string(&root).unwrap();
        let deserialized: EntityRoot = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(root, deserialized);
    }

    #[test]
    fn test_part_serialization() {
        let part = Part::new("test-part").unwrap();

        // Test JSON serialization
        let json = serde_json::to_string(&part).unwrap();
        assert_eq!(json, "\"test-part\"");

        // Test JSON deserialization
        let deserialized: Part = serde_json::from_str(&json).unwrap();
        assert_eq!(part, deserialized);

        // Test YAML serialization
        let yaml = serde_yaml::to_string(&part).unwrap();
        assert_eq!(yaml.trim(), "test-part");

        // Test YAML deserialization
        let deserialized: Part = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(part, deserialized);
    }

    #[test]
    fn test_parts_serialization() {
        let parts = Parts::new(vec![
            Part::new("part1").unwrap(),
            Part::new("part2").unwrap(),
            Part::new("part3").unwrap(),
        ]);

        // Test JSON serialization
        let json = serde_json::to_string(&parts).unwrap();

        // Test JSON deserialization
        let deserialized: Parts = serde_json::from_str(&json).unwrap();
        assert_eq!(parts, deserialized);

        // Test YAML serialization
        let yaml = serde_yaml::to_string(&parts).unwrap();

        // Test YAML deserialization
        let deserialized: Parts = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(parts, deserialized);
    }

    #[test]
    fn test_sha1name_serialization() {
        let sha1name = SHA1Name::new("test-content".to_string()).unwrap();

        // Test JSON serialization
        let json = serde_json::to_string(&sha1name).unwrap();
        let deserialized: SHA1Name = serde_json::from_str(&json).unwrap();
        assert_eq!(sha1name, deserialized);

        // Test YAML serialization
        let yaml = serde_yaml::to_string(&sha1name).unwrap();
        let deserialized: SHA1Name = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(sha1name, deserialized);
    }

    #[test]
    fn test_ern_serialization() {
        let ern = Ern::new(
            Domain::new("test-domain").unwrap(),
            Category::new("test-category").unwrap(),
            Account::new("test-account").unwrap(),
            EntityRoot::new("test-root".to_string()).unwrap(),
            Parts::new(vec![
                Part::new("part1").unwrap(),
                Part::new("part2").unwrap(),
            ]),
        );

        // Test JSON serialization
        let json = serde_json::to_string(&ern).unwrap();

        // Test JSON deserialization
        let deserialized: Ern = serde_json::from_str(&json).unwrap();
        assert_eq!(ern, deserialized);

        // Test YAML serialization
        let yaml = serde_yaml::to_string(&ern).unwrap();

        // Test YAML deserialization
        let deserialized: Ern = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(ern, deserialized);
    }

    #[test]
    fn test_ern_pretty_json_format() {
        let ern = Ern::new(
            Domain::new("test-domain").unwrap(),
            Category::new("test-category").unwrap(),
            Account::new("test-account").unwrap(),
            EntityRoot::new("test-root".to_string()).unwrap(),
            Parts::new(vec![
                Part::new("part1").unwrap(),
                Part::new("part2").unwrap(),
            ]),
        );

        // Test pretty JSON serialization
        let pretty_json = serde_json::to_string_pretty(&ern).unwrap();

        // Verify the pretty JSON contains expected fields
        assert!(pretty_json.contains("\"domain\""));
        assert!(pretty_json.contains("\"category\""));
        assert!(pretty_json.contains("\"account\""));
        assert!(pretty_json.contains("\"root\""));
        assert!(pretty_json.contains("\"parts\""));

        // Test deserialization from pretty JSON
        let _deserialized: Ern = serde_json::from_str(&pretty_json).unwrap();
    }

    #[test]
    fn test_ern_round_trip_json() {
        let original_ern = Ern::new(
            Domain::new("test-domain").unwrap(),
            Category::new("test-category").unwrap(),
            Account::new("test-account").unwrap(),
            EntityRoot::new("test-root".to_string()).unwrap(),
            Parts::new(vec![
                Part::new("part1").unwrap(),
                Part::new("part2").unwrap(),
            ]),
        );

        // Serialize to JSON
        let json = serde_json::to_string(&original_ern).unwrap();

        // Deserialize from JSON
        let deserialized: Ern = serde_json::from_str(&json).unwrap();

        // Every component survives the round trip, including the root
        assert_eq!(original_ern.domain(), deserialized.domain());
        assert_eq!(original_ern.category(), deserialized.category());
        assert_eq!(original_ern.account(), deserialized.account());
        assert_eq!(original_ern.root(), deserialized.root());
        assert_eq!(original_ern.parts(), deserialized.parts());
        assert_eq!(original_ern, deserialized);
    }

    #[test]
    fn test_ern_round_trip_yaml() {
        let original_ern = Ern::new(
            Domain::new("test-domain").unwrap(),
            Category::new("test-category").unwrap(),
            Account::new("test-account").unwrap(),
            EntityRoot::new("test-root".to_string()).unwrap(),
            Parts::new(vec![
                Part::new("part1").unwrap(),
                Part::new("part2").unwrap(),
            ]),
        );

        // Serialize to YAML
        let yaml = serde_yaml::to_string(&original_ern).unwrap();

        // Deserialize from YAML
        let deserialized: Ern = serde_yaml::from_str(&yaml).unwrap();

        // Every component survives the round trip, including the root
        assert_eq!(original_ern.domain(), deserialized.domain());
        assert_eq!(original_ern.category(), deserialized.category());
        assert_eq!(original_ern.account(), deserialized.account());
        assert_eq!(original_ern.root(), deserialized.root());
        assert_eq!(original_ern.parts(), deserialized.parts());
        assert_eq!(original_ern, deserialized);
    }

    #[test]
    fn test_invalid_json_deserialization() {
        // Test with invalid JSON
        let invalid_json =
            r#"{"domain": "test-domain", "category": "test-category", "invalid": true}"#;
        let result: Result<Ern, _> = serde_json::from_str(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_yaml_deserialization() {
        // Test with invalid YAML
        let invalid_yaml = r#"
        domain: test-domain
        category: test-category
        invalid: true
        "#;
        let result: Result<Ern, _> = serde_yaml::from_str(invalid_yaml);
        assert!(result.is_err());
    }
}
