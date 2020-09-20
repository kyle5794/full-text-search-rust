#[macro_export]
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[macro_export]
macro_rules! assert_vec {
    ($a:expr, $b:expr) => {
        let matching = $a.iter().zip($b.iter()).filter(|&(x, y)| x == y).count();
        assert_eq!(matching, $a.len());
        assert_eq!(matching, $b.len());
    };
}
