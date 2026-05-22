//! OwnerSignal contract for privileged `persona-orchestrate`
//! administration.
//!
//! Ordinary claim/release/handoff/activity traffic lives in
//! `signal-persona-orchestrate`. This crate carries owner-only
//! orders that mutate the orchestration substrate itself.

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode, NotaEnum, NotaRecord};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
pub use signal_persona_orchestrate::{
    ApplicationFailure, ApplicationFailureReason, ApplicationSuccess, DownstreamComponent,
    HarnessKind, LaneAuthority, LaneIdentifier, LaneRegistration, PartialApplied, Role,
    RoleIdentifier, RoleName, RoleToken, ScopeReason, WirePath,
};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct CreateRoleOrder {
    pub role: RoleIdentifier,
    pub harness: HarnessKind,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RetireRoleOrder {
    pub role: RoleIdentifier,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum Retirement {
    Role(RetireRoleOrder),
    Lane(LaneIdentifier),
}

impl NotaEncode for Retirement {
    fn encode(&self, encoder: &mut Encoder) -> nota_codec::Result<()> {
        match self {
            Self::Role(order) => {
                encoder.start_record("Role")?;
                order.encode(encoder)?;
                encoder.end_record()
            }
            Self::Lane(lane) => {
                encoder.start_record("Lane")?;
                lane.encode(encoder)?;
                encoder.end_record()
            }
        }
    }
}

impl NotaDecode for Retirement {
    fn decode(decoder: &mut Decoder<'_>) -> nota_codec::Result<Self> {
        let head = decoder.peek_record_head()?;
        match head.as_str() {
            "Role" => {
                decoder.expect_record_head("Role")?;
                let order = RetireRoleOrder::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::Role(order))
            }
            "Lane" => {
                decoder.expect_record_head("Lane")?;
                let lane = LaneIdentifier::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::Lane(lane))
            }
            other => Err(nota_codec::Error::UnknownVariant {
                enum_name: "Retirement",
                got: other.to_string(),
            }),
        }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RefreshRepositoryIndexOrder {}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct LaneRegistrationRequest {
    pub role: Role,
    pub authority: LaneAuthority,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct LaneAuthorityChange {
    pub lane: LaneIdentifier,
    pub authority: LaneAuthority,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RoleCreated {
    pub role: RoleIdentifier,
    pub harness: HarnessKind,
    pub report_repository_path: WirePath,
    pub report_lane_path: WirePath,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RoleRetired {
    pub role: RoleIdentifier,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RoleCreationRejected {
    pub role: RoleIdentifier,
    pub reason: RoleCreationRejectionReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RoleCreationRejectionReason {
    RoleAlreadyExists,
    ReportRepositoryAlreadyExists,
    ReportLaneAlreadyExists,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, Copy, PartialEq, Eq,
)]
pub struct RepositoryIndexRefreshed {
    pub repositories: u32,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct LaneRegistered {
    pub registration: LaneRegistration,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct LaneRetired {
    pub lane: LaneIdentifier,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct LaneAuthoritySet {
    pub lane: LaneIdentifier,
    pub authority: LaneAuthority,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum OwnerOrchestrateUnimplementedReason {
    NotBuiltYet,
    DependencyNotReady,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct OwnerOrchestrateRequestUnimplemented {
    pub operation: OwnerOperationKind,
    pub reason: OwnerOrchestrateUnimplementedReason,
}

signal_channel! {
    channel OwnerOrchestrate {
        operation Create(CreateRoleOrder),
        operation Retire(Retirement),
        operation Refresh(RefreshRepositoryIndexOrder),
        operation Register(LaneRegistrationRequest),
        operation SetAuthority(LaneAuthorityChange),
    }
    reply OwnerOrchestrateReply {
        RoleCreated(RoleCreated),
        RoleRetired(RoleRetired),
        RoleCreationRejected(RoleCreationRejected),
        RepositoryIndexRefreshed(RepositoryIndexRefreshed),
        LaneRegistered(LaneRegistered),
        LaneRetired(LaneRetired),
        LaneAuthoritySet(LaneAuthoritySet),
        PartialApplied(PartialApplied),
        OwnerOrchestrateRequestUnimplemented(OwnerOrchestrateRequestUnimplemented),
    }
}

pub type OwnerOrchestrateRequest = Operation;
pub type OwnerOperationKind = OperationKind;
pub type ChannelRequest = signal_frame::Request<Operation>;
pub type ChannelReply = signal_frame::Reply<OwnerOrchestrateReply>;

impl Operation {
    pub fn operation_kind(&self) -> OwnerOperationKind {
        self.kind()
    }
}

impl From<CreateRoleOrder> for OwnerOrchestrateRequest {
    fn from(payload: CreateRoleOrder) -> Self {
        Self::Create(payload)
    }
}

impl From<RetireRoleOrder> for OwnerOrchestrateRequest {
    fn from(payload: RetireRoleOrder) -> Self {
        Self::Retire(Retirement::Role(payload))
    }
}

impl From<LaneIdentifier> for OwnerOrchestrateRequest {
    fn from(payload: LaneIdentifier) -> Self {
        Self::Retire(Retirement::Lane(payload))
    }
}

impl From<RefreshRepositoryIndexOrder> for OwnerOrchestrateRequest {
    fn from(payload: RefreshRepositoryIndexOrder) -> Self {
        Self::Refresh(payload)
    }
}

impl From<LaneRegistrationRequest> for OwnerOrchestrateRequest {
    fn from(payload: LaneRegistrationRequest) -> Self {
        Self::Register(payload)
    }
}

impl From<LaneAuthorityChange> for OwnerOrchestrateRequest {
    fn from(payload: LaneAuthorityChange) -> Self {
        Self::SetAuthority(payload)
    }
}
