use super::{Like, PostStatus};
use crate::str_serializers::*;
use crate::{AttestationId, CommentId, SolutionId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, Timestamp};
use std::collections::HashSet;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Attestation {
    // Common fields
    pub id: AttestationId,
    pub name: String,
    pub description: String,
    pub author_id: AccountId,
    #[serde(with = "u64_dec_format")]
    pub timestamp: Timestamp,
    pub status: PostStatus,
    pub likes: HashSet<Like>,
    pub comments: Vec<CommentId>,

    //Specific fields
    #[serde(with = "u64_dec_format")]
    pub submission_id: SolutionId,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AttestationV1 {
    pub name: String,
    pub description: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "attestation_version")]
pub enum VersionedAttestation {
    V0(Attestation),
    V1(AttestationV1),
}

impl VersionedAttestation {
    pub fn latest_version(self) -> AttestationV1 {
        self.into()
    }
}

impl From<VersionedAttestation> for Attestation {
    fn from(va: VersionedAttestation) -> Self {
        match va {
            VersionedAttestation::V0(v0) => v0,
            VersionedAttestation::V1(_) => unimplemented!(),
        }
    }
}

impl From<VersionedAttestation> for AttestationV1 {
    fn from(va: VersionedAttestation) -> Self {
        match va {
            VersionedAttestation::V0(_) => unimplemented!(),
            VersionedAttestation::V1(v1) => v1,
        }
    }
}

impl From<Attestation> for VersionedAttestation {
    fn from(a: Attestation) -> Self {
        VersionedAttestation::V0(a)
    }
}
