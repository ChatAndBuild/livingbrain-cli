pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MovePageBucketRequest {
    /// Top-level grouping the page belongs to.
    pub bucket: MovePageBucketRequestBucket,
}

impl MovePageBucketRequest {
    pub fn builder() -> MovePageBucketRequestBuilder {
        <MovePageBucketRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MovePageBucketRequestBuilder {
    bucket: Option<MovePageBucketRequestBucket>,
}

impl MovePageBucketRequestBuilder {
    pub fn bucket(mut self, value: MovePageBucketRequestBucket) -> Self {
        self.bucket = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MovePageBucketRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bucket`](MovePageBucketRequestBuilder::bucket)
    pub fn build(self) -> Result<MovePageBucketRequest, BuildError> {
        Ok(MovePageBucketRequest {
            bucket: self.bucket.ok_or_else(|| BuildError::missing_field("bucket"))?,
        })
    }
}

