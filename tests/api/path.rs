use crate::prelude::*;

#[test]
fn path_leader_field() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    // length
    let path = Path::new("ldr.length")?;
    let values = record.path(&path, &options);
    assert_eq!(values, vec!["3612"]);

    // status
    let path = Path::new("ldr.status")?;
    let values = record.path(&path, &options);
    assert_eq!(values, vec!["n"]);

    // status
    let path = Path::new("ldr.encoding")?;
    let values = record.path(&path, &options);
    assert_eq!(values, vec!["a"]);

    // type
    let path = Path::new("ldr.type")?;
    let values = record.path(&path, &options);
    assert_eq!(values, vec!["z"]);

    // base_address
    let path = Path::new("ldr.base_address")?;
    let values = record.path(&path, &options);
    assert_eq!(values, vec!["589"]);

    Ok(())
}

#[test]
fn path_control_field() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    // simple
    let path = Path::new("001")?;
    let values = record.path(&path, &options);
    assert_eq!(values, vec!["119232022"]);

    // range
    let path = Path::new("005[4:8]")?;
    let values = record.path(&path, &options);
    assert_eq!(values, vec!["0720"]);

    // missing control field
    let path = Path::new("009")?;
    let values = record.path(&path, &options);
    assert!(values.is_empty());

    Ok(())
}

#[test]
fn path_data_field() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    // simple
    let path = Path::new("075.b")?;
    let values = record.path(&path, &options);
    assert_eq!(values, vec!["p", "piz"]);

    // predicate
    let path = Path::new("075{ b | 2 == 'gndspec' }")?;
    let values = record.path(&path, &options);
    assert_eq!(values, vec!["piz"]);

    // indicator
    let path = Path::new("100/1#.a")?;
    let values = record.path(&path, &options);
    assert_eq!(values, vec!["Lovelace, Ada"]);

    let path = Path::new("100/*.a")?;
    let values = record.path(&path, &options);
    assert_eq!(values, vec!["Lovelace, Ada"]);

    let path = Path::new("100/2#.a")?;
    let values = record.path(&path, &options);
    assert!(values.is_empty());

    let path = Path::new("100.a")?;
    let values = record.path(&path, &options);
    assert!(values.is_empty());

    // codes classes
    let path = Path::new("065.[a2]")?;
    let values = record.path(&path, &options);
    assert_eq!(values, vec!["28p", "sswd", "9.5p", "sswd"]);

    let path = Path::new("065{ [a2] | 2 == 'sswd' }")?;
    let values = record.path(&path, &options);
    assert_eq!(values, vec!["28p", "sswd", "9.5p", "sswd"]);

    let path = Path::new("065{ [abc] | 2 == 'sswd' }")?;
    let values = record.path(&path, &options);
    assert_eq!(values, vec!["28p", "9.5p"]);

    // multiple codes
    let path = Path::new("065{ a, 2 }")?;
    let values = record.path(&path, &options);
    assert_eq!(values, vec!["28p", "sswd", "9.5p", "sswd"]);

    // empty path
    let path = Path::new("100{ _ | 2 == 'gndspec'  }")?;
    let values = record.path(&path, &options);
    assert!(values.is_empty());

    let path = Path::new("100{ _ }")?;
    let values = record.path(&path, &options);
    assert!(values.is_empty());

    Ok(())
}
