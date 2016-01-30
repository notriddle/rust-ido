// This library is released under the same terms as Rust itself.

//! "Immediate do-notation": error handling for Haskell wannabes.
//!
//! Usage:
//!
//!     #[macro_use] extern crate ido;
//!     # fn main() {
//!     let result = ido!{
//!         x =<< Ok(1);
//!         _ =<< Err::<usize, usize>(x);
//!         Ok(unreachable!())
//!     };
//!     assert_eq!(result, Err(1));
//!     let val = ido!{
//!         a =<< Some(2);
//!         Some(a + 2)
//!     };
//!     assert_eq!(val, Some(4));
//!     # }

pub trait Bindable<T>: Sized {
    /// When binding, this is the type that is exposed to the user.
    type Value;
    /// Auxillary state that this pseudo-monad may want to track.
    type State;
    /// Given an instance of Self, decide how to proceed.
    fn bind(self) -> Binding<T, Self::Value, Self::State>;
}

pub trait Mergeable<T: Sized> {
    /// Add auxillary state from the previous instance to this one.
    /// (for example, to create a log in temporal order, `state` must be prepended)
    fn merge(&mut self, state: T);
}

impl<T> Mergeable<()> for T {
    fn merge(&mut self, _: ()) {}
}

impl<T, U> Bindable<Option<U>> for Option<T> {
    type Value = T;
    type State = ();
    fn bind(self) -> Binding<Option<U>, Self::Value, Self::State> {
        match self {
            Some(x) => Binding::Value(x, ()),
            None => Binding::Empty(None),
        }
    }
}

impl<T, U, E> Bindable<Result<U, E>> for Result<T, E> {
    type Value = T;
    type State = ();
    fn bind(self) -> Binding<Result<U, E>, Self::Value, Self::State> {
        match self {
            Ok(x) => Binding::Value(x, ()),
            Err(e) => Binding::Empty(Err(e)),
        }
    }
}

#[macro_export]
macro_rules! ido {
    { mut $var: ident =<< $val: expr; $($rest: tt)* } => {{
        let v = $val;
        match $crate::Bindable::bind(v) {
            $crate::Binding::Value(mut $var, state) => {
                let mut v = ido!{ $($rest)* };
                $crate::Mergeable::merge(&mut v, state);
                v
            },
            $crate::Binding::Empty(v) => {
                From::from(v)
            },
        }
    }};
    { _ =<< $val: expr; $($rest: tt)* } => {{
        let v = $val;
        match $crate::Bindable::bind(v) {
            $crate::Binding::Value(_, state) => {
                let mut v = ido!{ $($rest)* };
                $crate::Mergeable::merge(&mut v, state);
                v
            },
            $crate::Binding::Empty(v) => {
                From::from(v)
            },
        }
    }};
    { $var: ident =<< $val: expr; $($rest: tt)* } => {{
        let v = $val;
        match $crate::Bindable::bind(v) {
            $crate::Binding::Value($var, state) => {
                let mut v = ido!{ $($rest)* };
                $crate::Mergeable::merge(&mut v, state);
                v
            },
            $crate::Binding::Empty(v) => {
                From::from(v)
            },
        }
    }};
    { $val: stmt; $($rest: tt)* } => {{ $val; ido!{ $($rest)* } }};
    { $val: expr } => { $val };
    { } => { () };
}

/// The result of a pseudo-monadic bind. If `Empty` is returned, the `ido!`
/// will evaluate to its contents, short-circuiting the rest of the statements.
/// Otherwise, Value.0 is exposed to the user, and Value.1 will be merged into
/// the instance of the bindable that the next statement returns.
pub enum Binding<T, U, V> {
    Value(U, V),
    Empty(T),
}

#[cfg(test)]
mod test {
    use super::*;
    use std::borrow::Cow;
    use std::mem;
    #[test]
    pub fn complete_option() {
        assert_eq!(ido!{
            x =<< Some(1);
            Some(x+1)
        }, Some(2));
    }
    #[allow(unused)]
    #[test]
    pub fn early_return_option() {
        assert_eq!(ido!{
            x =<< Some(1);
            y =<< None as Option<usize>;
            Some(x+1)
        }, None);
    }
    #[test]
    pub fn complete() {
        let r: Result<usize, ()> = Ok(1);
        assert_eq!(ido!{
            x =<< r;
            Ok(x+1)
        }, Ok(2));
    }
    #[allow(unused)]
    #[test]
    pub fn early_return() {
        assert_eq!(ido!{
            x =<< Ok(1);
            y =<< Err::<usize, usize>(x);
            Ok(2)
        }, Err(1));
    }
    #[test]
    pub fn state_threading() {
        struct Logger<T>(T, Vec<Cow<'static, str>>);
        impl<T> Bindable<Logger<T>> for Logger<T> {
            type Value = T;
            type State = Vec<Cow<'static, str>>;
            fn bind(self) -> Binding<Self, Self::Value, Self::State> {
                Binding::Value(self.0, self.1)
            }
        };
        impl<T> Mergeable<Vec<Cow<'static, str>>> for Logger<T> {
            fn merge(&mut self, mut state: Vec<Cow<'static, str>> ) {
                mem::swap(&mut self.1, &mut state);
                self.1.append(&mut state);
            }
        }
        let log = ido!{
            a =<< Logger(1, vec!["This".into()]);
            a =<< Logger(a+1, vec!["is".into()]);
            a =<< Logger(a+1, vec!["overly".into()]);
            Logger(a+1, vec!["complicated".into()])
        };
        assert_eq!(log.1, vec![Cow::Borrowed("This"), "is".into(), "overly".into(), "complicated".into()]);
    }
}
