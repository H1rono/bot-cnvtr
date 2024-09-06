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

    (string $i:ident) => {
        ::paste::paste! {
            #[must_use]
            #[derive(
                Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
                ::serde::Serialize, ::serde::Deserialize
            )]
            #[serde(transparent)]
            pub struct [<$i:camel>](pub ::std::string::String);

            impl ::std::convert::From<::std::string::String> for [<$i:camel>] {
                fn from(value: ::std::string::String) -> Self {
                    Self(value)
                }
            }

            impl ::std::convert::From<[<$i:camel>]> for ::std::string::String {
                fn from(value: [<$i:camel>]) -> ::std::string::String {
                    value.0
                }
            }

            impl ::std::convert::AsRef<::std::string::String> for [<$i:camel>] {
                fn as_ref(&self) -> &::std::string::String {
                    &self.0
                }
            }

            impl ::std::convert::AsRef<::std::primitive::str> for [<$i:camel>] {
                fn as_ref(&self) -> &::std::primitive::str {
                    self.0.as_ref()
                }
            }

            impl ::std::fmt::Display for [<$i:camel>] {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result
                {
                    <::std::string::String as ::std::fmt::Display>::fmt(&self.0, f)
                }
            }
        }
    };
}

pub(crate) use newtype;
