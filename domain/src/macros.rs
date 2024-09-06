macro_rules! newtype {
    (id $i:ident) => {
        ::paste::paste! {
            #[must_use]
            #[derive(
                Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd,
                ::serde::Serialize, ::serde::Deserialize
            )]
            #[serde(transparent)]
            pub struct [<$i:camel Id>](pub ::uuid::Uuid);

            impl ::std::convert::From<::uuid::Uuid> for [<$i:camel Id>] {
                fn from(value: ::uuid::Uuid) -> Self {
                    Self(value)
                }
            }

            impl ::std::convert::From<[<$i:camel Id>]> for ::uuid::Uuid {
                fn from(value: [<$i:camel Id>]) -> Self {
                    value.0
                }
            }

            impl ::std::fmt::Display for [<$i:camel Id>] {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result
                {
                    <::uuid::Uuid as ::std::fmt::Display>::fmt(&self.0, f)
                }
            }
        }
    };
}

pub(crate) use newtype;
