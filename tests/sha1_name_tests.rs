use acton_ern::prelude::*;

#[test]
fn test_sha1name_in_ern() -> anyhow::Result<()> {
    // Create an ERN using SHA1Name
    let ern: Result<Ern, ErnError> = ErnBuilder::new()
        .with::<Domain>("acton-internal")?
        .with::<Category>("hr")?
        .with::<Account>("company123")?
        .with::<SHA1Name>("document-content")?
        .with::<Part>("departmentA")?
        .with::<Part>("team1")?
        .build();

    // Verify the constructed ERN matches the expected value
    assert!(
        ern.is_ok(),
        "ern:acton-internal:hr:company123:<sha1-id>/departmentA/team1"
    );

    let ern = ern?;
    assert_eq!(ern.domain().to_string(), "acton-internal");
    assert_eq!(ern.category().to_string(), "hr");
    assert_eq!(ern.account().to_string(), "company123");
    assert_eq!(ern.parts().to_string(), "departmentA/team1");

    // The root should not be empty
    assert!(!ern.root().to_string().is_empty());

    Ok(())
}

#[test]
fn test_sha1name_root_is_deterministic_through_builder() -> anyhow::Result<()> {
    // The type parameter must actually select the identifier algorithm. Building the same
    // ERN twice with SHA1Name has to produce the same root, not two v7 roots.
    let build = || -> Result<Ern, ErnError> {
        ErnBuilder::new()
            .with::<Domain>("acton-internal")?
            .with::<Category>("hr")?
            .with::<Account>("company123")?
            .with::<SHA1Name>("worker")?
            .with::<Part>("inbox")?
            .build()
    };

    let left = build()?;
    let right = build()?;

    assert_eq!(left.root(), right.root());
    assert_eq!(left, right);
    assert_eq!(
        left.root().as_str(),
        SHA1Name::new("worker".to_string())?.as_str()
    );
    Ok(())
}

#[test]
fn test_sha1name_root_survives_parsing() -> anyhow::Result<()> {
    let ern = ErnBuilder::new()
        .with::<Domain>("acton-internal")?
        .with::<Category>("hr")?
        .with::<Account>("company123")?
        .with::<SHA1Name>("worker")?
        .with::<Part>("inbox")?
        .build()?;

    let reparsed = ErnParser::new(ern.to_string()).parse()?;

    assert_eq!(ern, reparsed);
    Ok(())
}

#[test]
fn test_entityroot_root_is_not_deterministic_through_builder() -> anyhow::Result<()> {
    // The counterpart to the SHA1Name case: EntityRoot stays time-ordered and unique.
    let build = || -> Result<Ern, ErnError> {
        ErnBuilder::new()
            .with::<Domain>("acton-internal")?
            .with::<Category>("hr")?
            .with::<Account>("company123")?
            .with::<EntityRoot>("worker")?
            .with::<Part>("inbox")?
            .build()
    };

    assert_ne!(build()?.root(), build()?.root());
    Ok(())
}

#[test]
fn test_sha1name_creation() -> anyhow::Result<()> {
    // Create a SHA1Name directly
    let name1 = SHA1Name::new("test-content".to_string())?;
    let name2 = SHA1Name::new("test-content".to_string())?;

    // SHA1Name should be deterministic for the same input
    assert_eq!(name1.to_string(), name2.to_string());

    Ok(())
}

#[test]
fn test_sha1name_vs_entityroot() -> anyhow::Result<()> {
    // Create a SHA1Name
    let sha1_name1 = SHA1Name::new("test-content".to_string())?;
    let sha1_name2 = SHA1Name::new("test-content".to_string())?;

    // Create an EntityRoot
    let entity_root1 = EntityRoot::new("test-content".to_string())?;
    let entity_root2 = EntityRoot::new("test-content".to_string())?;

    // SHA1Name should be deterministic (same content always produces same ID)
    assert_eq!(sha1_name1.to_string(), sha1_name2.to_string());

    // EntityRoot should be non-deterministic (same content produces different IDs)
    assert_ne!(entity_root1.to_string(), entity_root2.to_string());

    Ok(())
}
