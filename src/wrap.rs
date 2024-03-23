// pub trait Wrapper<Inner> {
//     //type Inner;
//     fn new(value: Inner) -> Self;
//     fn into_inner(self) -> Inner;
//     fn inner(&self) -> &Inner;
//     fn inner_mut(&mut self) -> &mut Inner;
// }

use std::ops::{Deref, DerefMut};

macro_rules! implicit {
    // allow empty
    () => {};

    // allow multiple items
    (
        $(#[$attrs:meta])? // attributes, possibly empty
        $vis:vis // visibility, possibly empty
        type // trait keyword
        $new_type:ident = $type:ty; // trait signature
        $($rest:tt)*
    ) => {
        $(#[$attrs])?
        #[repr(transparent)]
        $vis // visibility, possibly empty
        struct // trait keyword
        $new_type($type); // trait signature

        impl Deref for $new_type {
            type Target = $type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $new_type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl From<$type> for $new_type  {
            fn from(value: $type) -> $new_type {
                Self(value)
            }
        }

        impl From<$new_type> for $type {
            fn from(value: $new_type) -> Self {
                value.0
            }
        }

        implicit!($($rest)*);
    };
    (
        $vis:vis // visibility, possibly empty
        impl // trait keyword
        $ext_trait_id:ident for $type:ty // trait signature
        {
            // functions
            $(
                $fn_vis:vis // visibility, possibly empty
                fn // fn keyword
                $fn_name:ident // fn name
                ( $($fn_arg:tt)* ) // fn args
                $(-> $fn_ret:ty)? // fn return type
                $fn_body:block // fn body
            )*
        }
        $($rest:tt)*
    ) => {
        impl // trait keyword
        $ext_trait_id
        for
        $type // with next line forms the Typepath of Trait
        {
            // functions
            $(
                fn // fn keyword
                $fn_name // fn name
                ( $($fn_arg)* ) // fn args
                $(-> $fn_ret)? // fn return type
                $fn_body // fn body
            )*
        }

        $vis // visibility, possibly empty
        trait // trait keyword
        $ext_trait_id // trait signature
        {
            // functions
            $(
                fn // fn keyword
                $fn_name // fn name
                ( $($fn_arg)* ) // fn args
                $(-> $fn_ret)? // fn return type
                ;
            )*
        }
        implicit!($($rest)*);
    };

    // https://doc.rust-lang.org/reference/items/traits.html
    // https://doc.rust-lang.org/reference/paths.html?highlight=path#paths
    (
        $vis:vis // visibility, possibly empty
        trait // trait keyword
        $ti:ident // trait signature
        $(<$($gen:ident),+>)? // GenericParams
        : $c0:ident $(<$($c0g:ty),+>)? $(+ $cn:path)* // TypeParamBounds. for some reason, the first one can't
        // where clause should go here
        { $($body:tt)* } // body, while block matches, it doesn't work for some reason
        $($rest:tt)*
    ) => {
        $vis // visibility, possibly empty
        trait // trait keyword
        $ti // with next line forms the Typepath of Trait
        <$($($gen ,)+)?> //  GenericParams
        : $c0 $(<$($c0g),+>)? $(+ $cn)* // TypeParamBounds
        {
            $($body)* // body
        }
        // SelfType should be unique type parameter
        impl <$($($gen ,)+)? SelfType>
            $ti <$($($gen ,)+)?> // Typepath of Trait
            for SelfType // type of match
            where SelfType: $c0 $(<$($c0g),+>)? $(+ $cn)*
        {

        }
        implicit!($($rest)*);
    };
}


implicit!(
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, ::serde::Serialize, ::serde::Deserialize)]
    pub type WrapperString = String;

    pub type WrapperF64 = f64;

    pub trait WrapperGeneric1: Deref + DerefMut {
        fn uh_oh1(&self) {}
    }

    pub trait WrapperGenericImplicit<Inner>:
        From<Inner> + Into<Inner> + Deref<Target = Inner> + DerefMut<Target = Inner>
    {
        fn uh_oh2(&self) {}
    }

    pub impl StringExt for str {
        fn uh_oh3(&self) -> bool {
            true
        }
    }
    pub impl FloatExt for f64 {
        fn uh_oh4(&self) -> bool {
            true
        }
    }
);

// implicit!(pub trait WrapperGeneric1 :  [ Deref + DerefMut ] {
// fn uh_oh(&self) {
// }
// });

#[test]
fn test() {
    let x = WrapperF64::from(1.1);
    x.uh_oh1();
    x.uh_oh2();

    let hello = WrapperString::from("hello".to_string());
    let world = WrapperString::from("world".to_string());

    let json = serde_json::to_string_pretty(&hello).unwrap();

    println!("{}", json);

    assert_ne!(hello, world);
    "hello".uh_oh3();
}

//implicit!(pub trait WrapperGenericImplicit<Inner> : From<Inner> + Into<Inner> + Deref<Target = Inner> + DerefMut<Target = Inner>{
// can have extension methods but don't need to
// fn from_inner(value: Inner) -> Self {
//     Self::from(value)
// }

// fn into_inner(self) -> Inner {
//     self.into()
// }

// fn inner(&self) -> &Inner {
//     self.deref()
// }

// fn inner_mut(&mut self) -> &mut Inner {
//     self.deref_mut()
// }
// });

// implicit!(pub trait WrapperGenericImplicit<Inner> : From<Inner> + Into<Inner> + Deref<Target = Inner> + DerefMut<Target = Inner>);

// impl<T> WrapperAssociatedImplicit for Wrapper
// where
//     T: $tt,
// {
//     type Inner = Inner;

// }
// pub trait WrapperGenericImplicit<Inner>:
//     From<Inner> + Into<Inner> + Deref<Target = Inner> + DerefMut<Target = Inner>
// {

// }

// can do like an extension trait

// implicit
// pub trait WrapperAssociatedImplicit:
//     From<Self::Inner> + Into<Self::Inner> + Deref<Target = Self::Inner> + DerefMut<Target = Self::Inner>
// {
//     type Inner;
//     // fn from_inner(value: Self::Inner) -> Self {
//     //     Self::from(value)
//     // }
//     // fn into_inner(self) -> Self::Inner {
//     //     self.into()
//     // }
//     // fn inner(&self) -> &Self::Inner {
//     //     self.deref()
//     // }
//     // fn inner_mut(&mut self) -> &mut Self::Inner {
//     //     self.deref_mut()
//     // }
// }

// impl<Wrapper, Inner> WrapperAssociatedImplicit for Wrapper
// where
//     Wrapper: From<Inner> + Into<Inner> + Deref<Target = Inner> + DerefMut<Target = Inner>,
// {
//     type Inner = Inner;

// }

// impl<Wrapper, Inner> WrapperGenericImplicit<Inner> for Wrapper
// where
//     Wrapper: From<Inner> + Into<Inner> + Deref<Target = Inner> + DerefMut<Target = Inner>,
// {

// }

// backwards is not possible
// seems generalyl impossible to implement forign traits by local trait
// pub trait WrapperAssociated {
//     type Inner;

//     fn from_inner(value: Self::Inner);
//     fn into_inner(self) -> Self::Inner;
//     fn inner(&self) -> &Self::Inner;
//     fn inner_mut(&mut self) -> &mut Self::Inner;
// }

// pub trait WrapperGeneric<Inner> {
//     fn from_inner(value: Inner);
//     fn into_inner(self) -> Inner;
//     fn inner(&self) -> &Inner;
//     fn inner_mut(&mut self) -> &mut Inner;
// }

// pub trait Wrapper1Ext<Inner>: WrapperGenericImplicit<Inner> {
//     fn from_inner(value: Inner) -> Inner {
//         Inner::from(value)
//     }
//     fn into_inner(self) -> Inner {
//         self.into()
//     }
//     fn inner(&self) -> &Inner {
//         self.deref()
//     }
//     fn inner_mut(&mut self) -> &mut Inner {
//         self.deref_mut()
//     }
// }

// impl<I, W: WrapperGenericImplicit<I>> Wrapper1Ext<I> for W {}

// #[test]
// fn test_wrap1() {
//     let mut wrap = FloatWrap::from(1.0);
//     assert_eq!(<FloatWrap as Wrapper1Ext<f64>>::inner(&wrap), &1.0);
//     *Wrapper1Ext::inner_mut(&mut wrap) = 2.0;
//     assert_eq!(Wrapper1Ext::inner(&wrap), &2.0);
//     assert_eq!(Wrapper1Ext::into_inner(wrap), 2.0);
// }

// // pub trait Wrapper:
// //     From<Self::Inner>
// //     + Into<Self::Inner>
// //     + std::ops::Deref<Target = Self::Inner>
// //     + std::ops::DerefMut<Target = Self::Inner>
// // {
// //     type Inner;
// //     fn from_inner(value: Self::Inner) -> Self {
// //         Self::Inner::from(value)
// //     }
// //     fn into_inner(self) -> Self::Inner {
// //         self.into()
// //     }
// //     fn inner(&self) -> &Self::Inner {
// //         self.deref()
// //     }
// //     fn inner_mut(&mut self) -> &mut Self::Inner {
// //         self.deref_mut()
// //     }
// // }



// // impl Wrapper for FloatWrap {
// //     type Inner = f64;
// // }

// #[derive(
//     Debug,
//     derive_more::Add,
//     ::derive_more::Display,
//     Default,
//     Clone,
//     Hash,
//     PartialEq,
//     Eq,
//     PartialOrd,
//     Ord,
//     ::serde::Serialize,
//     ::serde::Deserialize,
// )]
// #[repr(transparent)]
// pub struct Wrap<T> {
//     inner: T,
// }

// impl ::std::hash::Hash for Wrap<f64> {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.inner.hash(state);
//     }
// }

// can't implement because don't know T doesn't already
// impl<T> Into<T> for Wrap<T> where T : ! From<Wrap<<T>> {
//     fn into(self) -> T {
//         self.inner
//     }
// }

// impl<T> Wrapper<T> for Wrap<T> {
//     fn new(value: T) -> Self {
//         Self { inner: value }
//     }

//     fn into_inner(self) -> T {
//         self.inner
//     }

//     fn inner(&self) -> &T {
//         &self.inner
//     }

//     fn inner_mut(&mut self) -> &mut T {
//         &mut self.inner
//     }
// }

// // compare if wraped type has compare
// impl<T: PartialEq> PartialEq<T> for Wrap<T> {
//     fn eq(&self, other: &T) -> bool {
//         other.eq(self)
//     }
// }

// impl<T: PartialOrd> PartialOrd<T> for Wrap<T> {
//     fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
//         self.inner.partial_cmp(other)
//     }
// }

// #[test]
// fn test_wrap() {
//     let a = Wrap::new(1.0);
//     let b = Wrap::new(0.0);
//     let c = Wrap::new("haha");

//     assert_ne!(a, b);
//     assert_eq!(c, "haha");
//     assert!(a > b);
//     assert!(a == 1.0);
//     assert!(a > 0.0);
//     assert_eq!(a, 1.0);
//     assert_eq!(a.into_inner(), 1.0);
// }

// #[macro_export]
// macro_rules! wrap {
//     ($ident:ident = $type:ty) => {
//         #[derive(::schemars::JsonSchema)]
//         #[derive(
//             Serialize,
//             Deserialize,
//             Deref,
//             Debug,
//             Into,
//             From,
//             Constructor,
//             PartialEq,
//             Eq,
//             Clone,
//             Ord,
//             PartialOrd,
//             Hash,
//         )]
//         // #[deref(forward)]
//         #[repr(transparent)]
//         pub struct $ident($type);

//         impl $ident {
//             pub fn inner(&self) -> &$type {
//                 &self.0
//             }
//         }
//     };
// }
