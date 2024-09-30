pub trait IntoWithParam<T, P>: Sized {
    fn into_with_param(self, param: P) -> T;
}
