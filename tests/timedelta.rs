use datetime::timedelta::Timedelta;

#[test]
fn test_timedelta() {
    let default_timedelta = Timedelta::default();

    // assert_eq!(default_timedelta.years, 0);
}

#[test]
fn print_default_timedelta() {
    let default_timedelta = Timedelta::default();
    println!("{:?}", default_timedelta);
    println!("{}", default_timedelta);
}

#[test]
fn print_timedelta() {
    let default_timedelta = Timedelta::new(
        Some(61),
        Some(6),
        Some(5),
        Some(2),
        Some(7),
        Some(1),
        Some(true),
    );
    println!("{:?}", default_timedelta);
    println!("{}", default_timedelta);
}
