#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type ComponentName = String;
#[rustfmt::skip]
pub type Component = ComponentName;
#[rustfmt::skip]
pub type MigrationIdentifier = String;
#[rustfmt::skip]
pub type RecordKind = String;
#[rustfmt::skip]
pub type RawByte = i64;
#[rustfmt::skip]
pub type RawBytes = std::vec::Vec<RawByte>;
#[rustfmt::skip]
pub type HandoverMarkerPayload = HandoverMarkerData;
#[rustfmt::skip]
pub type Source = Version;
#[rustfmt::skip]
pub type Target = Version;
#[rustfmt::skip]
pub type Identifier = MigrationIdentifier;
#[rustfmt::skip]
pub type Migrations = std::vec::Vec<SupportedMigration>;
#[rustfmt::skip]
pub type Migration = MigrationIdentifier;
#[rustfmt::skip]
pub type ChangedRecords = i64;
#[rustfmt::skip]
pub type RejectionReasonSelection = RejectionReason;
#[rustfmt::skip]
pub type Completions = std::vec::Vec<Completion>;
#[rustfmt::skip]
pub type Rejections = std::vec::Vec<Rejection>;
#[rustfmt::skip]
pub type Year = i64;
#[rustfmt::skip]
pub type Month = i64;
#[rustfmt::skip]
pub type Day = i64;
#[rustfmt::skip]
pub type Major = i64;
#[rustfmt::skip]
pub type Minor = i64;
#[rustfmt::skip]
pub type Patch = i64;
#[rustfmt::skip]
pub type Hour = i64;
#[rustfmt::skip]
pub type Minute = i64;
#[rustfmt::skip]
pub type Second = i64;
#[rustfmt::skip]
pub type SchemaHash = ContractVersion;
#[rustfmt::skip]
pub type StateSequence = i64;
#[rustfmt::skip]
pub type MirroredWriteCount = i64;
#[rustfmt::skip]
pub type RecordFrontier = std::option::Option<i64>;
#[rustfmt::skip]
pub type RecordedAtDate = Date;
#[rustfmt::skip]
pub type RecordedAtTime = Time;
#[rustfmt::skip]
pub type SourceMarker = HandoverMarkerData;
#[rustfmt::skip]
pub type AcceptedMarker = HandoverMarkerData;
#[rustfmt::skip]
pub type SourceVersion = ContractVersion;
#[rustfmt::skip]
pub type TargetVersion = ContractVersion;
#[rustfmt::skip]
pub type Kind = RecordKind;
#[rustfmt::skip]
pub type Payload = RawBytes;
#[rustfmt::skip]
pub type DivergenceReasonSelection = DivergenceReason;
#[rustfmt::skip]
pub type FailureIdentifier = i64;
#[rustfmt::skip]
pub type FinalizedMarker = HandoverMarkerData;
#[rustfmt::skip]
pub type DivergenceIdentifier = i64;
#[rustfmt::skip]
pub type Recovered = bool;
#[rustfmt::skip]
pub type HandoverRejectionReasonSelection = HandoverRejectionReason;
#[rustfmt::skip]
pub type ContractVersion = RawBytes;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Version {
    pub major: Major,
    pub minor: Minor,
    pub patch: Patch,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SupportedMigration {
    pub component: Component,
    pub source: Source,
    pub target: Target,
    pub identifier: Identifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Inspection {
    All,
    Component(ComponentName),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Attempt {
    pub component: Component,
    pub source: Source,
    pub target: Target,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ReportQuery {
    All,
    Component(ComponentName),
}
#[rustfmt::skip]
pub type InspectionReportedPayload = Migrations;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Completion {
    pub component: Component,
    pub source: Source,
    pub target: Target,
    pub migration: Migration,
    pub changed_records: ChangedRecords,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RejectionReason {
    UnsupportedMigration,
    ComponentMismatch,
    MigrationFailed,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Rejection {
    pub component: Component,
    pub source: Source,
    pub target: Target,
    pub rejection_reason_selection: RejectionReasonSelection,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ReportedPayload {
    pub completions: Completions,
    pub rejections: Rejections,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum UnimplementedReason {
    NotBuiltYet,
    IntegrationNotLanded,
}
#[rustfmt::skip]
pub type RequestUnimplementedPayload = UnimplementedReason;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Date {
    pub year: Year,
    pub month: Month,
    pub day: Day,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Time {
    pub hour: Hour,
    pub minute: Minute,
    pub second: Second,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HandoverMarkerData {
    pub component: Component,
    pub schema_hash: SchemaHash,
    pub state_sequence: StateSequence,
    pub mirrored_write_count: MirroredWriteCount,
    pub record_frontier: RecordFrontier,
    pub recorded_at_date: RecordedAtDate,
    pub recorded_at_time: RecordedAtTime,
}
#[rustfmt::skip]
pub type MarkerRequest = Component;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ReadinessReport {
    pub component: Component,
    pub source_marker: SourceMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CompletionReport {
    pub component: Component,
    pub accepted_marker: AcceptedMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MirrorPayload {
    pub component: Component,
    pub source_version: SourceVersion,
    pub target_version: TargetVersion,
    pub kind: Kind,
    pub payload: Payload,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DivergencePayload {
    pub component: Component,
    pub source_version: SourceVersion,
    pub target_version: TargetVersion,
    pub divergence_reason_selection: DivergenceReasonSelection,
    pub kind: Kind,
    pub payload: Payload,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RecoveryRequest {
    pub component: Component,
    pub failure_identifier: FailureIdentifier,
}
#[rustfmt::skip]
pub type HandoverAcceptance = AcceptedMarker;
#[rustfmt::skip]
pub type HandoverFinalization = FinalizedMarker;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MirrorAcknowledgement {
    pub component: Component,
    pub mirrored_write_count: MirroredWriteCount,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DivergenceAcknowledgement {
    pub component: Component,
    pub divergence_identifier: DivergenceIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RecoveryResult {
    pub component: Component,
    pub recovered: Recovered,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HandoverRejection {
    pub component: Component,
    pub handover_rejection_reason_selection: HandoverRejectionReasonSelection,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum HandoverRejectionReason {
    SchemaMismatch,
    StateSequenceAdvanced,
    AlreadyInHandover,
    NotReady,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DivergenceReason {
    NotRepresentable,
    TargetUnavailable,
    TargetRejected,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Inspect(Inspection),
    AttemptUpgrade(Attempt),
    Report(ReportQuery),
    AskHandoverMarker(MarkerRequest),
    ReadyToHandover(ReadinessReport),
    HandoverCompleted(CompletionReport),
    Mirror(MirrorPayload),
    Divergence(DivergencePayload),
    RecoverFromFailure(RecoveryRequest),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    InspectionReported(InspectionReportedPayload),
    UpgradeCompleted(Completion),
    UpgradeRejected(Rejection),
    Reported(ReportedPayload),
    HandoverMarker(HandoverMarkerPayload),
    HandoverAccepted(HandoverAcceptance),
    HandoverFinalized(HandoverFinalization),
    MirrorAcknowledged(MirrorAcknowledgement),
    DivergenceAcknowledged(DivergenceAcknowledgement),
    RecoveryCompleted(RecoveryResult),
    HandoverRejected(HandoverRejection),
    RequestUnimplemented(RequestUnimplementedPayload),
}
