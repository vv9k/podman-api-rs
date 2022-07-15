#![allow(dead_code)]

macro_rules! api_url {
    () => {
        concat!("https://docs.podman.io/en/", crate::version!() , "/_static/api.html")
    };
    (operation $ep:expr) => {
        concat!("\n[Api Reference](", api_url!(), "#operation/", $ep, ")")
    };
    (tag $ep:expr) => {
        concat!("\n[Api Reference](", api_url!(), "#tag/", $ep, ")")
    };
    ($base:ident) => {
        api_url!(tag stringify!($base))
    };
    ($base:ident => $op:ident) => {
        api_url!(operation concat!(stringify!($base), stringify!($op)))
    };
}

macro_rules! api_doc {
    (
        $base:ident => $op:ident
        $(#[doc = $doc:expr])*
        |
        $it:item
    ) => {
        #[doc = concat!(api_url!($base => $op))]
        #[doc = "\n"]
        $(
            #[doc = $doc]
        )*
        $it
    };
    (
        $base:ident
        $(#[doc = $doc:expr])*
        |
        $it:item
    ) => {
        #[doc = concat!(api_url!($base))]
        #[doc = "\n"]
        $(
            #[doc = $doc]
        )*
        $it
    };
}

macro_rules! impl_api_ty {
    ($(#[doc = $docs:expr])* $name:ident => $name_field:ident) => {
        paste::item! {
            #[doc = concat!("Interface for accessing and manipulating Podman ", stringify!($name), ".\n", $($docs,)* "\n", api_url!($name))]
            #[derive(Debug)]
            pub struct [< $name >] {
                podman: crate::Podman,
                $name_field: crate::Id,
            }

            impl [< $name >] {
                #[doc = concat!("Exports an interface exposing operations against a ", stringify!($name), " instance.")]
                pub fn new(podman: crate::Podman, $name_field: impl Into<crate::Id>) -> Self
                {
                    [< $name >] {
                        podman,
                        $name_field: $name_field.into(),
                    }
                }

                #[doc = concat!("A getter for ", stringify!($name), " ", stringify!($name_field))]
                pub fn $name_field(&self) -> &crate::Id {
                    &self.$name_field
                }


            }

            #[doc = concat!("Handle for Podman ", stringify!($name), "s.")]
            #[derive(Debug)]
            pub struct [< $name s >] {
                podman: crate::Podman,
            }

            impl [< $name s >] {
                #[doc = concat!("Exports an interface for interacting with Podman ", stringify!($name), "s.")]
                pub fn new(podman: crate::Podman) -> Self {
                    [< $name s >] { podman }
                }

                #[doc = concat!("Returns a reference to a set of operations available to a specific ", stringify!($name), ".")]
                pub fn get(&self, $name_field: impl Into<crate::Id>) -> [< $name >]
                {
                    [< $name >]::new(self.podman.clone(), $name_field)
                }
            }

        }
    }
}
