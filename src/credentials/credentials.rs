use crate::helpers::Class;
use serde::{Deserialize, Serialize};

/// Trait implemented by specialization of credentials
pub trait Credentials {}

macro_rules! credentials_with_common_fields_and_impl {
    (
        $(#[$attr:meta])*
        pub struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                pub $field:ident: $field_type:ty,
            )*
            $(private_fields {
                $(
                    $(#[$private_field_attr:meta])*
                    $private_field:ident: $private_field_type:ty
                ),* $(,)*
            })*
        }
    ) => {
        $(#[$attr])*
        pub struct $name {
            /// ID of the credentials
            pub id: String,
            /// Description
            pub description: String,
            /// Name displayed in the Jenkins UI
            pub display_name: String,
            /// Full name containing the domain as well as the ID
            pub full_name: String,
            /// Description of the credentials' type
            pub type_name: String,

            $(
                $(#[$field_attr])*
                pub $field: $field_type,
            )*
            $($(
                $(#[$private_field_attr])*
                $private_field: $private_field_type,
            )*)*
        }
        impl Credentials for $name {}
    };
}

credentials_with_common_fields_and_impl!(/// A Jenkins `Computer`
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommonCredentials {
    /// _class provided by Jenkins
    #[serde(rename = "_class")]
    pub class: Option<String>,
    private_fields {
        #[serde(flatten)]
        other_fields: serde_json::Value,
    }
});
specialize!(CommonCredentials => Credentials);

impl CommonCredentials {}
