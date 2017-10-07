fn test<'a>(slice: &'a [i32]) -> impl Iterator<Item=&'a i32> {
    slice.iter()
}