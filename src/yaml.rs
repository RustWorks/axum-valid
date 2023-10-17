//! # Support for `Yaml<T>` from `axum-yaml`
//!
//! ## Feature
//!
//! Enable the `yaml` feature to use `Valid<Yaml<T>>`.
//!
//! ## Usage
//!
//! 1. Implement `Deserialize` and `Validate` for your data type `T`.
//! 2. In your handler function, use `Valid<Yaml<T>>` as some parameter's type.
//!
//! ## Example
//!
//! ```no_run
//! #[cfg(feature = "validator")]
//! mod validator_example {
//!     use axum::routing::post;
//!     use axum_yaml::Yaml;
//!     use axum::Router;
//!     use axum_valid::Valid;
//!     use serde::Deserialize;
//!     use validator::Validate;
//!
//!     pub fn router() -> Router {
//!         Router::new().route("/yaml", post(handler))
//!     }
//!
//!     async fn handler(Valid(Yaml(parameter)): Valid<Yaml<Parameter>>) {
//!         assert!(parameter.validate().is_ok());
//!         // Support automatic dereferencing
//!         println!("v0 = {}, v1 = {}", parameter.v0, parameter.v1);
//!     }
//!
//!     #[derive(Validate, Deserialize)]
//!     pub struct Parameter {
//!         #[validate(range(min = 5, max = 10))]
//!         pub v0: i32,
//!         #[validate(length(min = 1, max = 10))]
//!         pub v1: String,
//!     }
//! }
//!
//! #[cfg(feature = "garde")]
//! mod garde_example {
//!     use axum::routing::post;
//!     use axum_yaml::Yaml;
//!     use axum::Router;
//!     use axum_valid::Garde;
//!     use serde::Deserialize;
//!     use garde::Validate;
//!
//!     pub fn router() -> Router {
//!         Router::new().route("/yaml", post(handler))
//!     }
//!
//!     async fn handler(Garde(Yaml(parameter)): Garde<Yaml<Parameter>>) {
//!         assert!(parameter.validate(&()).is_ok());
//!         // Support automatic dereferencing
//!         println!("v0 = {}, v1 = {}", parameter.v0, parameter.v1);
//!     }
//!
//!     #[derive(Validate, Deserialize)]
//!     pub struct Parameter {
//!         #[garde(range(min = 5, max = 10))]
//!         pub v0: i32,
//!         #[garde(length(min = 1, max = 10))]
//!         pub v1: String,
//!     }
//! }
//!
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! #     use axum::Router;
//! #     let router = Router::new();
//! #     #[cfg(feature = "validator")]
//! #     let router = router.nest("/validator", validator_example::router());
//! #     #[cfg(feature = "garde")]
//! #     let router = router.nest("/garde", garde_example::router());
//! #     axum::Server::bind(&([0u8, 0, 0, 0], 8080).into())
//! #         .serve(router.into_make_service())
//! #         .await?;
//! #     Ok(())
//! # }
//! ```

use crate::HasValidate;
#[cfg(feature = "validator")]
use crate::HasValidateArgs;
use axum_yaml::Yaml;
#[cfg(feature = "validator")]
use validator::ValidateArgs;

impl<T> HasValidate for Yaml<T> {
    type Validate = T;
    fn get_validate(&self) -> &T {
        &self.0
    }
}

#[cfg(feature = "validator")]
impl<'v, T: ValidateArgs<'v>> HasValidateArgs<'v> for Yaml<T> {
    type ValidateArgs = T;
    fn get_validate_args(&self) -> &Self::ValidateArgs {
        &self.0
    }
}

#[cfg(feature = "validify")]
impl<T: validify::Modify> crate::HasModify for Yaml<T> {
    type Modify = T;

    fn get_modify(&mut self) -> &mut Self::Modify {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{ValidTest, ValidTestParameter};
    use axum::http::StatusCode;
    use axum_yaml::Yaml;
    use reqwest::RequestBuilder;
    use serde::Serialize;

    impl<T: ValidTestParameter + Serialize> ValidTest for Yaml<T> {
        const ERROR_STATUS_CODE: StatusCode = StatusCode::UNSUPPORTED_MEDIA_TYPE;

        fn set_valid_request(builder: RequestBuilder) -> RequestBuilder {
            builder
                .header(reqwest::header::CONTENT_TYPE, "application/yaml")
                .body(
                    serde_yaml::to_string(&T::valid())
                        .expect("Failed to serialize parameters to yaml"),
                )
        }

        fn set_error_request(builder: RequestBuilder) -> RequestBuilder {
            // `Content-Type` not set, `Yaml` should return `415 Unsupported Media Type`
            builder
        }

        fn set_invalid_request(builder: RequestBuilder) -> RequestBuilder {
            builder
                .header(reqwest::header::CONTENT_TYPE, "application/yaml")
                .body(
                    serde_yaml::to_string(&T::invalid())
                        .expect("Failed to serialize parameters to yaml"),
                )
        }
    }
}
