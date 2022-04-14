#![allow(dead_code)]

macro_rules! impl_vec_field {
    ($(#[doc = $docs:expr])* $name:ident => $api_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name  >]<I, S>(mut self, $name: I)-> Self
            where
                I: IntoIterator<Item = S>,
                S: AsRef<str> + serde::Serialize
            {
                self.params.insert($api_name, serde_json::json!($name.into_iter().collect::<Vec<_>>()));
                self
            }
        }
    };
    ($(#[doc = $docs:expr])* $name:ident: $ty:ty => $api_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name  >]<I>(mut self, $name: I)-> Self
            where
                I: IntoIterator<Item = $ty>,
            {
                self.params.insert($api_name, serde_json::json!($name.into_iter().collect::<Vec<_>>()));
                self
            }
        }
    };
}

macro_rules! impl_field {
    ($(#[doc = $docs:expr])* $name:ident: $ty:ty => $api_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name >](mut self, $name: $ty)-> Self
            {
                self.params.insert($api_name, serde_json::json!($name));
                self
            }
        }
    };
}

macro_rules! impl_str_field {
    ($(#[doc = $docs:expr])* $name:ident => $api_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name >](mut self, $name: impl AsRef<str> + serde::Serialize)-> Self
            {
                self.params.insert($api_name, serde_json::json!($name.as_ref()));
                self
            }
        }
    };
}

macro_rules! impl_url_str_field {
    ($(#[doc = $docs:expr])* $name:ident => $api_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name >](mut self, $name: impl Into<String>)-> Self
            {
                self.params.insert($api_name, $name.into());
                self
            }
        }
    };
}

macro_rules! impl_url_field {
    ($(#[doc = $docs:expr])* $name:ident : $ty:tt => $api_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name >](mut self, $name: $ty)-> Self {
                self.params.insert($api_name, $name.to_string());
                self
            }
        }
    };
}

macro_rules! impl_url_vec_field {
    ($(#[doc = $docs:expr])* $name:ident => $api_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name >]<I, S>(mut self, $name: I)-> Self
            where
                I: IntoIterator<Item = S>,
                S: Into<String>
            {
                let joined = $name.into_iter().map(|it| it.into()).collect::<Vec<_>>().join(",");
                self.params.insert($api_name, format!("{}",joined));
                self
            }
        }
    };
}

macro_rules! impl_url_bool_field {
    ($(#[doc = $docs:expr])* $name:ident => $api_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name >](mut self, $name: bool)-> Self {
                self.params.insert($api_name, $name.to_string());
                self
            }
        }
    };
}

macro_rules! impl_url_enum_field {
    ($(#[doc = $docs:expr])* $name:ident: $ty:tt => $api_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name >](mut self, $name: $ty)-> Self
            {
                self.params.insert($api_name, $name.to_string());
                self
            }
        }
    };
}

macro_rules! impl_str_enum_field {
    ($(#[doc = $docs:expr])* $name:ident: $ty:tt => $api_name:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name >](mut self, $name: $ty)-> Self
            {
                self.params.insert($api_name, serde_json::json!($name.to_string()));
                self
            }
        }
    };
}

macro_rules! impl_map_field {
    (url $(#[doc = $docs:expr])* $name:ident => $api_name:literal) => {
        impl_map_field! { $(#[doc = $docs])* $name => $api_name => serde_json::to_string(&$name.into_iter().collect::<std::collections::HashMap<_, _>>()).unwrap_or_default() }
    };
    (json $(#[doc = $docs:expr])* $name:ident => $api_name:literal) => {
        impl_map_field! { $(#[doc = $docs])* $name => $api_name => serde_json::json!($name.into_iter().collect::<std::collections::HashMap<_, _>>()) }
    };
    ($(#[doc = $docs:expr])* $name:ident => $api_name:literal => $ret:expr) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            pub fn [< $name  >]<I, K, V>(mut self, $name: I)-> Self
            where
                I: IntoIterator<Item = (K, V)>,
                K: AsRef<str> + serde::Serialize + Eq + std::hash::Hash,
                V: AsRef<str> + serde::Serialize
            {
                self.params.insert($api_name, $ret);
                self
            }
        }
    };
}

macro_rules! impl_filter_func {
    ($(#[doc = $doc:expr])* $filter_ty:ident) => {
        $(
            #[doc = $doc]
        )*
        pub fn filter<F>(mut self, filters: F) -> Self
        where
            F: IntoIterator<Item = $filter_ty>,
        {
            let mut param = std::collections::HashMap::new();
            for (key, val) in filters.into_iter().map(|f| f.query_key_val()) {
                param.insert(key, vec![val]);
            }
            // structure is a a json encoded object mapping string keys to a list
            // of string values
            self.params
                .insert("filters", serde_json::to_string(&param).unwrap_or_default());
            self
        }
    };
}

macro_rules! impl_opts_builder {
    ($(#[doc = $docs:expr])* $name:ident $ty:expr) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            #[derive(serde::Serialize, Debug, Default, Clone)]
            pub struct [< $name Opts >] {
                params: std::collections::HashMap<&'static str, $ty>,
            }
            impl [< $name Opts >] {
                #[doc = concat!("Returns a new instance of a builder for ", stringify!($name), "Opts.")]
                pub fn builder() -> [< $name OptsBuilder >] {
                    [< $name OptsBuilder >]::default()
                }
            }

            #[doc = concat!("A builder struct for ", stringify!($name), "Opts.")]
            #[derive(Default, Debug, Clone)]
            pub struct [< $name OptsBuilder >] {
                params: std::collections::HashMap<&'static str, $ty>,
            }

            impl [< $name OptsBuilder >] {
                #[doc = concat!("Finish building ", stringify!($name), "Opts.")]
                pub fn build(self) -> [< $name Opts >] {
                    [< $name Opts >] {
                        params: self.params,
                    }
                }
            }
       }
    };
    (json => $(#[doc = $docs:expr])* $name:ident) => {
        paste::item! {
            impl_opts_builder!($(#[doc = $docs])* $name serde_json::Value);

            impl [< $name Opts >] {
                /// Serialize options as a JSON String. Returns an error if the options will fail
                /// to serialize.
                pub fn serialize(&self) -> crate::Result<String> {
                    serde_json::to_string(&self.params).map_err(crate::Error::from)
                }
            }
        }
    };
    (url => $(#[doc = $docs:expr])* $name:ident) => {
        paste::item! {
            impl_opts_builder!($(#[doc = $docs])* $name String);

            impl [< $name  Opts >] {
                /// Serialize options as a URL query String. Returns None if no options are defined.
                pub fn serialize(&self) -> Option<String> {
                    if self.params.is_empty() {
                        None
                    } else {
                        Some(
                            crate::util::url::encoded_pairs(&self.params)
                        )
                    }
                }
            }
        }
    };
}

macro_rules! impl_opts_required_builder {
    ($(#[doc = $docs:expr])* $name:ident $ty:expr, $(#[doc = $param_docs:expr])* $param:ident => $param_key:literal) => {
        paste::item! {
            $(
                #[doc= $docs]
            )*
            #[derive(serde::Serialize, Debug, Default, Clone)]
            pub struct [< $name Opts >] {
                params: std::collections::HashMap<&'static str, $ty>,
            }
            impl [< $name Opts >] {
                #[doc = concat!("Returns a new instance of a builder for ", stringify!($name), "Opts.")]
                $(
                    #[doc= $param_docs]
                )*
                pub fn builder($param: impl Into<$ty>) -> [< $name OptsBuilder >] {
                    [< $name OptsBuilder >]::new($param)
                }
            }

            #[doc = concat!("A builder struct for ", stringify!($name), "Opts.")]
            #[derive(Debug, Clone)]
            pub struct [< $name OptsBuilder >] {
                params: std::collections::HashMap<&'static str, $ty>,
            }

            impl [< $name OptsBuilder >] {
                #[doc = concat!("A builder struct for ", stringify!($name), "Opts.")]
                $(
                    #[doc= $param_docs]
                )*
                pub fn new($param: impl Into<$ty>) -> Self {
                    Self {
                        params: [($param_key, $param.into())].into()
                    }
                }

                #[doc = concat!("Finish building ", stringify!($name), "Opts.")]
                pub fn build(self) -> [< $name Opts >] {
                    [< $name Opts >] {
                        params: self.params,
                    }
                }
            }
       }
    };
    (json => $(#[doc = $docs:expr])* $name:ident, $(#[doc = $param_docs:expr])* $param:ident => $param_key:literal) => {
        impl_opts_required_builder!($(#[doc = $docs])* $name serde_json::Value, $(#[doc = $param_docs])* $param => $param_key);

        paste::item! {
            impl [< $name Opts >] {
                /// Serialize options as a JSON String. Returns an error if the options will fail
                /// to serialize.
                pub fn serialize(&self) -> crate::Result<String> {
                    serde_json::to_string(&self.params).map_err(crate::Error::from)
                }
            }
        }
    };
    (url => $(#[doc = $docs:expr])* $name:ident, $(#[doc = $param_docs:expr])* $param:ident => $param_key:literal) => {
        impl_opts_required_builder!($(#[doc = $docs])* $name String, $(#[doc = $param_docs])* $param => $param_key);

        paste::item! {
            impl [< $name  Opts >] {
                /// Serialize options as a URL query String. Returns None if no options are defined.
                pub fn serialize(&self) -> Option<String> {
                    if self.params.is_empty() {
                        None
                    } else {
                        Some(
                            crate::util::url::encoded_pairs(&self.params)
                        )
                    }
                }
            }
        }
    };
}

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
            pub struct [< $name >]<'podman> {
                podman: &'podman crate::Podman,
                $name_field: crate::Id,
            }

            impl<'podman> [< $name >]<'podman> {
                #[doc = concat!("Exports an interface exposing operations against a ", stringify!($name), " instance.")]
                pub fn new(podman: &'podman crate::Podman, $name_field: impl Into<crate::Id>) -> Self
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
            pub struct [< $name s >]<'podman> {
                podman: &'podman crate::Podman,
            }

            impl<'podman> [< $name s >]<'podman> {
                #[doc = concat!("Exports an interface for interacting with Podman ", stringify!($name), "s.")]
                pub fn new(podman: &'podman crate::Podman) -> Self {
                    [< $name s >] { podman }
                }

                #[doc = concat!("Returns a reference to a set of operations available to a specific ", stringify!($name), ".")]
                pub fn get(&self, $name_field: impl Into<crate::Id>) -> [< $name >]<'podman>
                {
                    [< $name >]::new(self.podman, $name_field)
                }
            }

        }
    }
}
