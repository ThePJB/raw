pub trait ExpectWith<T, E> {
    #[track_caller]
    fn expect_with<F>(self, f: F) -> T
    where
        F: FnOnce() -> E;
}

impl<T, E: std::fmt::Display> ExpectWith<T, E> for Option<T> {
    fn expect_with<F>(self, f: F) -> T
    where
        F: FnOnce() -> E,
    {
        match self {
            Some(val) => val,
            None => panic!("{}", f()),
            // None => {
            //     // println!("{}", f());
            //     // panic!("panic");
            // },
        }
    }
}

impl<T, E: std::fmt::Display, E2> ExpectWith<T, E> for Result<T, E2> {
    fn expect_with<F>(self, f: F) -> T
    where
        F: FnOnce() -> E,
    {
        match self {
            Ok(val) => val,
            Err(_) => panic!("{}", f()),
        }
    }
}

pub trait UnwrapMut<T> {
    #[track_caller]
    fn unwrap_mut(&mut self) -> &mut T;
}

impl<T> UnwrapMut<T> for Option<T> {
    #[track_caller]
    fn unwrap_mut(&mut self) -> &mut T {
        self.as_mut()
            .expect("called `UnwrapMut::unwrap_mut` on a `None` value")
    }
}

impl<T, E> UnwrapMut<T> for Result<T, E> {
    #[track_caller]
    fn unwrap_mut(&mut self) -> &mut T {
        match self {
            Ok(value) => value,
            Err(_) => panic!("called `UnwrapMut::unwrap_mut` on an `Err` value"),
        }
    }
}

pub trait UnwrapRef<'a, T> {
    #[track_caller]
    fn unwrap_ref(&'a self) -> &'a T;
}

impl<'a, T> UnwrapRef<'a, T> for Option<T> {
    #[track_caller]
    fn unwrap_ref(&'a self) -> &'a T {
        self.as_ref()
            .expect("called `UnwrapRef::unwrap_ref` on a `None` value")
    }
}

impl<'a, T, E> UnwrapRef<'a, T> for Result<T, E> {
    fn unwrap_ref(&'a self) -> &'a T {
        match self {
            Ok(value) => value,
            Err(_) => panic!("called `UnwrapRef::unwrap_ref` on an `Err` value"),
        }
    }
}

// fuck can it take like &'static str as well? guess it depends on the error... idk
//shouldnt rly matter should just be debug
// yea y would it need to be an owned string
// oh yeh its dumb doesnt need to have type to E for Result<T,E>

// as for whether theres any slow question mark anywhere, idk.
