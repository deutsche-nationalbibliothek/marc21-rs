use crate::prelude::*;

#[test]
fn query_multiple_constituents() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let query = Query::new("001,500/1#{ a, 9 | 4 == 'bezf' }")?;
    let values = record.query(&query, &options);
    assert_eq!(
        values,
        vec![
            vec!["119232022", "Byron, George Gordon Byron", "v:Vater"],
            vec![
                "119232022",
                "Byron, Anne Isabella Milbanke Byron",
                "v:Mutter"
            ],
            vec!["119232022", "Blunt, Anne Isabella", "v:Tochter"],
        ]
    );

    Ok(())
}

#[test]
fn query_leader_field() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    // length
    let query = Query::new("ldr.length")?;
    let values = record.query(&query, &options);
    assert_eq!(values, vec![vec!["3612"]]);

    // status
    let query = Query::new("ldr.status")?;
    let values = record.query(&query, &options);
    assert_eq!(values, vec![vec!["n"]]);

    // status
    let query = Query::new("ldr.encoding")?;
    let values = record.query(&query, &options);
    assert_eq!(values, vec![vec!["a"]]);

    // type
    let query = Query::new("ldr.type")?;
    let values = record.query(&query, &options);
    assert_eq!(values, vec![vec!["z"]]);

    // base_address
    let query = Query::new("ldr.base_address")?;
    let values = record.query(&query, &options);
    assert_eq!(values, vec![vec!["589"]]);

    Ok(())
}

#[test]
fn query_control_field() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    // simple
    let query = Query::new("001")?;
    let values = record.query(&query, &options);
    assert_eq!(values, vec![vec!["119232022"]]);

    // range
    let query = Query::new("005[4:8]")?;
    let values = record.query(&query, &options);
    assert_eq!(values, vec![vec!["0720"]]);

    // missing control field
    let query = Query::new("009")?;
    let values = record.query(&query, &options);
    assert!(values.is_empty());

    let query = Query::new("001,009")?;
    let values = record.query(&query, &options);
    assert_eq!(values, vec![vec!["119232022", ""]]);

    Ok(())
}

#[test]
fn query_data_field() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    // simple
    let query = Query::new("075.b")?;
    let values = record.query(&query, &options);
    assert_eq!(values, vec![vec!["p"], vec!["piz"]]);

    // predicate
    let query = Query::new("075{ b | 2 == 'gndspec' }")?;
    let values = record.query(&query, &options);
    assert_eq!(values, vec![vec!["piz"]]);

    // indicator
    let query = Query::new("100/1#.a")?;
    let values = record.query(&query, &options);
    assert_eq!(values, vec![vec!["Lovelace, Ada"]]);

    let query = Query::new("100/*.a")?;
    let values = record.query(&query, &options);
    assert_eq!(values, vec![vec!["Lovelace, Ada"]]);

    let query = Query::new("100/2#.a")?;
    let values = record.query(&query, &options);
    assert!(values.is_empty());

    let query = Query::new("100.a")?;
    let values = record.query(&query, &options);
    assert!(values.is_empty());

    // codes classes
    let query = Query::new("065.[a2]")?;
    let values = record.query(&query, &options);
    assert_eq!(
        values,
        vec![vec!["28p"], vec!["sswd"], vec!["9.5p"], vec!["sswd"]]
    );

    let query = Query::new("065{ [a2] | 2 == 'sswd' }")?;
    let values = record.query(&query, &options);
    assert_eq!(
        values,
        vec![vec!["28p"], vec!["sswd"], vec!["9.5p"], vec!["sswd"]]
    );

    let query = Query::new("065{ [abc] | 2 == 'sswd' }")?;
    let values = record.query(&query, &options);
    assert_eq!(values, vec![vec!["28p"], vec!["9.5p"]]);

    // multiple codes
    let query = Query::new("065{ a, 2 }")?;
    let values = record.query(&query, &options);
    assert_eq!(values, vec![vec!["28p", "sswd"], vec!["9.5p", "sswd"]]);

    // empty query
    let query = Query::new("100{ _ | 2 == 'gndspec'  }")?;
    let values = record.query(&query, &options);
    assert!(values.is_empty());

    let query = Query::new("100{ _ }")?;
    let values = record.query(&query, &options);
    assert!(values.is_empty());

    Ok(())
}

#[test]
fn query_missing_field() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let query = Query::new("001,101/1#{ a, b }")?;
    let values = record.query(&query, &options);
    assert_eq!(values, vec![vec!["119232022", "", ""]]);

    Ok(())
}
