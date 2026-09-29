pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DeletePermanentlyWorkspaceRequestPermanent {
    #[serde(rename = "true")]
    True,
}
impl fmt::Display for DeletePermanentlyWorkspaceRequestPermanent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::True => "true",
        };
        write!(f, "{}", s)
    }
}
