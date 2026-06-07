//! Signal contract for the ordinary `upgrade` surface.
//!
//! This crate carries the peer-callable vocabulary for inspecting
//! compiled migration support, attempting a schema upgrade, reading
//! reports, and coordinating adjacent-version handover. Runtime
//! migration logic lives in `upgrade`; cross-version projection helpers
//! live in `version-projection`.

pub mod schema {
    #[rustfmt::skip]
    pub mod lib;
}

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode, NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
use signal_sema::SemaObservation;
use version_projection::{ComponentName as ProjectionComponentName, ContractVersion, RecordKind};

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct ComponentName(String);

impl ComponentName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct MigrationIdentifier(String);

impl MigrationIdentifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl Version {
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct SupportedMigration {
    pub component: ComponentName,
    pub source: Version,
    pub target: Version,
    pub identifier: MigrationIdentifier,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum Inspection {
    All,
    Component(ComponentName),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Attempt {
    pub component: ComponentName,
    pub source: Version,
    pub target: Version,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum ReportQuery {
    All,
    Component(ComponentName),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct InspectionReported {
    pub migrations: Vec<SupportedMigration>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Completion {
    pub component: ComponentName,
    pub source: Version,
    pub target: Version,
    pub migration: MigrationIdentifier,
    pub changed_records: u64,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RejectionReason {
    UnsupportedMigration,
    ComponentMismatch,
    MigrationFailed,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Rejection {
    pub component: ComponentName,
    pub source: Version,
    pub target: Version,
    pub reason: RejectionReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Reported {
    pub completions: Vec<Completion>,
    pub rejections: Vec<Rejection>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum UnimplementedReason {
    NotBuiltYet,
    IntegrationNotLanded,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RequestUnimplemented {
    pub reason: UnimplementedReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl Date {
    pub const fn new(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }
}

impl NotaEncode for Date {
    fn encode(&self, encoder: &mut Encoder) -> nota_codec::Result<()> {
        encoder.write_date(self.year, self.month, self.day)
    }
}

impl NotaDecode for Date {
    fn decode(decoder: &mut Decoder<'_>) -> nota_codec::Result<Self> {
        let (year, month, day) = decoder.read_date()?;
        Ok(Self { year, month, day })
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Time {
    pub const fn new(hour: u8, minute: u8, second: u8) -> Self {
        Self {
            hour,
            minute,
            second,
        }
    }
}

impl NotaEncode for Time {
    fn encode(&self, encoder: &mut Encoder) -> nota_codec::Result<()> {
        encoder.write_time(self.hour, self.minute, self.second)
    }
}

impl NotaDecode for Time {
    fn decode(decoder: &mut Decoder<'_>) -> nota_codec::Result<Self> {
        let (hour, minute, second) = decoder.read_time()?;
        Ok(Self {
            hour,
            minute,
            second,
        })
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct HandoverMarker {
    pub component: ProjectionComponentName,
    pub schema_hash: ContractVersion,
    pub state_sequence: u64,
    pub mirrored_write_count: u64,
    pub record_frontier: Option<u64>,
    pub recorded_at_date: Date,
    pub recorded_at_time: Time,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct MarkerRequest {
    pub component: ProjectionComponentName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ReadinessReport {
    pub component: ProjectionComponentName,
    pub source_marker: HandoverMarker,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct CompletionReport {
    pub component: ProjectionComponentName,
    pub accepted_marker: HandoverMarker,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct MirrorPayload {
    pub component: ProjectionComponentName,
    pub source_version: ContractVersion,
    pub target_version: ContractVersion,
    pub kind: RecordKind,
    pub payload: Vec<u8>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DivergencePayload {
    pub component: ProjectionComponentName,
    pub source_version: ContractVersion,
    pub target_version: ContractVersion,
    pub reason: DivergenceReason,
    pub kind: RecordKind,
    pub payload: Vec<u8>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RecoveryRequest {
    pub component: ProjectionComponentName,
    pub failure_identifier: u64,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct HandoverAcceptance {
    pub accepted_marker: HandoverMarker,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct HandoverFinalization {
    pub finalized_marker: HandoverMarker,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct MirrorAcknowledgement {
    pub component: ProjectionComponentName,
    pub mirrored_write_count: u64,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DivergenceAcknowledgement {
    pub component: ProjectionComponentName,
    pub divergence_identifier: u64,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RecoveryResult {
    pub component: ProjectionComponentName,
    pub recovered: bool,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct HandoverRejection {
    pub component: ProjectionComponentName,
    pub reason: HandoverRejectionReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum HandoverRejectionReason {
    SchemaMismatch,
    StateSequenceAdvanced,
    AlreadyInHandover,
    NotReady,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum DivergenceReason {
    NotRepresentable,
    TargetUnavailable,
    TargetRejected,
}

signal_channel! {
    channel Upgrade {
        operation Inspect(Inspection),
        operation AttemptUpgrade(Attempt),
        operation Report(ReportQuery),
        operation AskHandoverMarker(MarkerRequest),
        operation ReadyToHandover(ReadinessReport),
        operation HandoverCompleted(CompletionReport),
        operation Mirror(MirrorPayload),
        operation Divergence(DivergencePayload),
        operation RecoverFromFailure(RecoveryRequest),
    }
    reply Reply {
        InspectionReported(InspectionReported),
        UpgradeCompleted(Completion),
        UpgradeRejected(Rejection),
        Reported(Reported),
        HandoverMarker(HandoverMarker),
        HandoverAccepted(HandoverAcceptance),
        HandoverFinalized(HandoverFinalization),
        MirrorAcknowledged(MirrorAcknowledgement),
        DivergenceAcknowledged(DivergenceAcknowledgement),
        RecoveryCompleted(RecoveryResult),
        HandoverRejected(HandoverRejection),
        RequestUnimplemented(RequestUnimplemented),
    }
    observable {
        filter default;
        operation_event OperationReceived;
        effect_event EffectEmitted;
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct OperationReceived {
    pub operation: OperationKind,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct EffectEmitted {
    pub observation: SemaObservation,
}
