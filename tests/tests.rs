use acton_ern::EntityRoot;
use acton_ern::prelude::*;

//
// /// Tests for the Acton Ern implementation
#[test]
fn test() -> anyhow::Result<()> {
    // Create an ERN (Entity Resource Name) using the ErnBuilder with specified components
    let ern: Result<Ern, ErnError> = ErnBuilder::new()
        .with::<Domain>("acton-internal")?
        .with::<Category>("hr")?
        .with::<Account>("company123")?
        .with::<EntityRoot>("root")?
        .with::<Part>("departmentA")?
        .with::<Part>("team1")?
        .build();

    // Verify the constructed ERN (Entity Resource Name) matches the expected value
    assert!(
        ern.is_ok(),
        "ern:acton-internal:hr:company123:root/departmentA/team1"
    );
    let ern = ern?;
    assert_eq!(ern.domain().to_string(), "acton-internal");
    assert_eq!(ern.category().to_string(), "hr");
    assert_eq!(ern.account().to_string(), "company123");
    assert_eq!(ern.parts().to_string(), "departmentA/team1");
    assert!(ern.root().to_string().starts_with("root_"));
    Ok(())
}
//
#[test]
fn test_v7() -> anyhow::Result<()> {
    // Create an ERN (Entity Resource Name) using the ErnBuilder with specified components
    let ern_left: Result<Ern, ErnError> = ErnBuilder::new()
        .with::<Domain>("acton-internal".to_string())?
        .with::<Category>("hr".to_string())?
        .with::<Account>("company123".to_string())?
        .with::<EntityRoot>("root".to_string())?
        .with::<Part>("departmentA".to_string())?
        .with::<Part>("team1".to_string())?
        .build();

    let ern_right: Result<Ern, ErnError> = ErnBuilder::new()
        .with::<Domain>("acton-internal".to_string())?
        .with::<Category>("hr".to_string())?
        .with::<Account>("company123".to_string())?
        .with::<EntityRoot>("root".to_string())?
        .with::<Part>("departmentA".to_string())?
        .with::<Part>("team1".to_string())?
        .build();

    // Verify the constructed ERN (Entity Resource Name) matches the expected value
    assert!(ern_left.is_ok());
    assert!(ern_right.is_ok());
    assert_ne!(ern_left?, ern_right?);
    Ok(())
}

#[test]
fn test_v5() -> anyhow::Result<()> {
    // Create an ERN (Entity Resource Name) using the ErnBuilder with specified components
    let ern_left: Result<Ern, ErnError> = ErnBuilder::new()
        .with::<Domain>("acton-internal".to_string())?
        .with::<Category>("hr".to_string())?
        .with::<Account>("company123".to_string())?
        .with::<EntityRoot>("same".to_string())?
        .with::<Part>("departmentA".to_string())?
        .with::<Part>("team1".to_string())?
        .build();

    let ern_right: Result<Ern, ErnError> = ErnBuilder::new()
        .with::<Domain>("acton-internal".to_string())?
        .with::<Category>("hr".to_string())?
        .with::<Account>("company123".to_string())?
        .with::<EntityRoot>("same".to_string())?
        .with::<Part>("departmentA".to_string())?
        .with::<Part>("team1".to_string())?
        .build();

    // Verify the constructed ERN (Entity Resource Name) matches the expected value
    assert!(ern_left.is_ok());
    assert!(ern_right.is_ok());
    // Compare individual components instead of the full string
    let left = ern_left?;
    let right = ern_right?;
    assert_eq!(left.domain(), right.domain());
    assert_eq!(left.category(), right.category());
    assert_eq!(left.account(), right.account());
    assert_eq!(left.parts(), right.parts());
    // Don't compare roots as they'll have different IDs
    Ok(())
}

/// Regression test for https://github.com/govcraft/acton-ern/issues/5.
///
/// This is the call sequence `acton-reactive`'s `ActorConfig::new` uses to derive a
/// supervised child's identity from its parent.
#[test]
fn test_ern_round_trip_and_add() -> anyhow::Result<()> {
    let parent = Ern::with_root("pool")?;
    let reparsed = ErnParser::new(parent.to_string()).parse()?;
    assert_eq!(parent, reparsed, "parser should round-trip Display output");

    let child = (parent.clone() + Ern::with_root("worker")?)?;
    assert!(
        child.to_string().contains("worker"),
        "child should carry its own name"
    );
    Ok(())
}

/// Deriving a child from a parent must go through the parent's real root, so the child
/// is recognizably descended from it rather than from a corrupted copy.
#[test]
fn test_derived_child_keeps_parent_identity() -> anyhow::Result<()> {
    let parent = Ern::with_root("pool")?;
    let parent_id = ErnParser::new(parent.to_string()).parse()?;
    let child = (parent_id + Ern::with_root("worker")?)?;

    assert_eq!(child.root(), parent.root());
    assert!(child.is_child_of(&parent));
    assert_eq!(child.parent().as_ref(), Some(&parent));
    Ok(())
}

#[test]
fn test_parser() -> anyhow::Result<()> {
    // Create an ErnParser with a specific ERN (Entity Resource Name) string
    let parser: ErnParser =
        ErnParser::new("ern:acton-internal:hr:company123:root/departmentA/team1".to_string());

    // Parse the ERN (Entity Resource Name) string into its components
    let result = parser.parse();

    // Verify the parser returns a successful result
    assert!(
        result.is_ok(),
        "Parser should return Ok, but returned Err with message: {:?}",
        result.err()
    );

    // Extract the components from the result
    let ern = result.unwrap();

    // Verify each component matches the expected value
    assert_eq!(
        ern.domain().to_string(),
        "acton-internal",
        "Domain should be 'acton-internal'"
    );
    assert_eq!(ern.category().to_string(), "hr", "Category should be 'hr'");
    assert_eq!(
        ern.account().to_string(),
        "company123",
        "Account should be 'company123'"
    );
    assert_eq!(
        ern.parts().to_string(),
        "departmentA/team1",
        "Parts should match expected values"
    );
    Ok(())
}
