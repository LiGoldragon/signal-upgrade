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

#[cfg(feature = "nota-text")]
use nota_next::{Block, Delimiter, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
use version_projection::{ComponentName as ProjectionComponentName, ContractVersion, RecordKind};

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentName(String);

impl ComponentName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MigrationIdentifier(String);

impl MigrationIdentifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl Version {
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct SupportedMigration {
    pub component: ComponentName,
    pub source: Version,
    pub target: Version,
    pub identifier: MigrationIdentifier,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum Inspection {
    All,
    Component(ComponentName),
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Attempt {
    pub component: ComponentName,
    pub source: Version,
    pub target: Version,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum ReportQuery {
    All,
    Component(ComponentName),
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct InspectionReported {
    pub migrations: Vec<SupportedMigration>,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Completion {
    pub component: ComponentName,
    pub source: Version,
    pub target: Version,
    pub migration: MigrationIdentifier,
    pub changed_records: u64,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RejectionReason {
    UnsupportedMigration,
    ComponentMismatch,
    MigrationFailed,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Rejection {
    pub component: ComponentName,
    pub source: Version,
    pub target: Version,
    pub reason: RejectionReason,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Reported {
    pub completions: Vec<Completion>,
    pub rejections: Vec<Rejection>,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnimplementedReason {
    NotBuiltYet,
    IntegrationNotLanded,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct RequestUnimplemented {
    pub reason: UnimplementedReason,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Date {
    pub year: u64,
    pub month: u64,
    pub day: u64,
}

impl Date {
    pub const fn new(year: u64, month: u64, day: u64) -> Self {
        Self { year, month, day }
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Time {
    pub hour: u64,
    pub minute: u64,
    pub second: u64,
}

impl Time {
    pub const fn new(hour: u64, minute: u64, second: u64) -> Self {
        Self {
            hour,
            minute,
            second,
        }
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HandoverMarker {
    pub component: ProjectionComponentName,
    pub schema_hash: ContractVersion,
    pub state_sequence: u64,
    pub mirrored_write_count: u64,
    pub record_frontier: Option<u64>,
    pub recorded_at_date: Date,
    pub recorded_at_time: Time,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct MarkerRequest {
    pub component: ProjectionComponentName,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct ReadinessReport {
    pub component: ProjectionComponentName,
    pub source_marker: HandoverMarker,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct CompletionReport {
    pub component: ProjectionComponentName,
    pub accepted_marker: HandoverMarker,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct MirrorPayload {
    pub component: ProjectionComponentName,
    pub source_version: ContractVersion,
    pub target_version: ContractVersion,
    pub kind: RecordKind,
    pub payload: Vec<u8>,
}

#[cfg(feature = "nota-text")]
impl NotaEncode for MirrorPayload {
    fn to_nota(&self) -> String {
        Delimiter::Parenthesis.wrap([
            self.component.to_nota(),
            self.source_version.to_nota(),
            self.target_version.to_nota(),
            self.kind.to_nota(),
            BytePayload::new(self.payload.clone()).to_nota(),
        ])
    }
}

#[cfg(feature = "nota-text")]
impl NotaDecode for MirrorPayload {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let fields =
            NotaBlock::new(block).expect_children(Delimiter::Parenthesis, "MirrorPayload", 5)?;
        Ok(Self {
            component: ProjectionComponentName::from_nota_block(&fields[0])?,
            source_version: ContractVersion::from_nota_block(&fields[1])?,
            target_version: ContractVersion::from_nota_block(&fields[2])?,
            kind: RecordKind::from_nota_block(&fields[3])?,
            payload: BytePayload::from_nota_block(&fields[4])?.into_vec(),
        })
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct DivergencePayload {
    pub component: ProjectionComponentName,
    pub source_version: ContractVersion,
    pub target_version: ContractVersion,
    pub reason: DivergenceReason,
    pub kind: RecordKind,
    pub payload: Vec<u8>,
}

#[cfg(feature = "nota-text")]
impl NotaEncode for DivergencePayload {
    fn to_nota(&self) -> String {
        Delimiter::Parenthesis.wrap([
            self.component.to_nota(),
            self.source_version.to_nota(),
            self.target_version.to_nota(),
            self.reason.to_nota(),
            self.kind.to_nota(),
            BytePayload::new(self.payload.clone()).to_nota(),
        ])
    }
}

#[cfg(feature = "nota-text")]
impl NotaDecode for DivergencePayload {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let fields = NotaBlock::new(block).expect_children(
            Delimiter::Parenthesis,
            "DivergencePayload",
            6,
        )?;
        Ok(Self {
            component: ProjectionComponentName::from_nota_block(&fields[0])?,
            source_version: ContractVersion::from_nota_block(&fields[1])?,
            target_version: ContractVersion::from_nota_block(&fields[2])?,
            reason: DivergenceReason::from_nota_block(&fields[3])?,
            kind: RecordKind::from_nota_block(&fields[4])?,
            payload: BytePayload::from_nota_block(&fields[5])?.into_vec(),
        })
    }
}

#[cfg(feature = "nota-text")]
#[derive(Debug, Clone, PartialEq, Eq)]
struct BytePayload {
    bytes: Vec<u8>,
}

#[cfg(feature = "nota-text")]
impl BytePayload {
    fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    fn into_vec(self) -> Vec<u8> {
        self.bytes
    }
}

#[cfg(feature = "nota-text")]
impl NotaEncode for BytePayload {
    fn to_nota(&self) -> String {
        Delimiter::SquareBracket.wrap(self.bytes.iter().map(u8::to_string))
    }
}

#[cfg(feature = "nota-text")]
impl NotaDecode for BytePayload {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let values = Vec::<u64>::from_nota_block(block)?;
        let mut bytes = Vec::with_capacity(values.len());
        for value in values {
            let byte = u8::try_from(value).map_err(|_| {
                NotaDecodeError::Parse(format!("byte payload item out of range: {value}"))
            })?;
            bytes.push(byte);
        }
        Ok(Self::new(bytes))
    }
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct RecoveryRequest {
    pub component: ProjectionComponentName,
    pub failure_identifier: u64,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HandoverAcceptance {
    pub accepted_marker: HandoverMarker,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HandoverFinalization {
    pub finalized_marker: HandoverMarker,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct MirrorAcknowledgement {
    pub component: ProjectionComponentName,
    pub mirrored_write_count: u64,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct DivergenceAcknowledgement {
    pub component: ProjectionComponentName,
    pub divergence_identifier: u64,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct RecoveryResult {
    pub component: ProjectionComponentName,
    pub recovered: bool,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HandoverRejection {
    pub component: ProjectionComponentName,
    pub reason: HandoverRejectionReason,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HandoverRejectionReason {
    SchemaMismatch,
    StateSequenceAdvanced,
    AlreadyInHandover,
    NotReady,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct OperationReceived {
    pub operation: OperationKind,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct EffectEmitted {
    pub operation: OperationKind,
    pub outcome: EffectOutcome,
}

#[cfg_attr(
    feature = "nota-text",
    derive(nota_next::NotaEncode, nota_next::NotaDecode)
)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectOutcome {
    InspectionReported,
    UpgradeCompleted,
    UpgradeRejected,
    Reported,
    HandoverMarkerReturned,
    HandoverAccepted,
    HandoverFinalized,
    MirrorAcknowledged,
    DivergenceAcknowledged,
    RecoveryCompleted,
    HandoverRejected,
    RequestUnimplemented,
    NoChange,
}
