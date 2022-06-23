use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct Cw4626ConfigOptions {
    pub withdrawals_allowed: Option<bool>,
    pub deposits_allowed: Option<bool>,
}
