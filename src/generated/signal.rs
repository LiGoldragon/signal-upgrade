#![allow(dead_code, non_camel_case_types, non_snake_case)]
pub type ComponentName = String;
pub type Component = ComponentName;
pub type MigrationIdentifier = String;
pub type RecordKind = String;
pub type RawByte = i64;
pub type RawBytes = std::vec::Vec<RawByte>;
pub type HandoverMarkerPayload = HandoverMarkerData;
pub type Source = Version;
pub type Target = Version;
pub type Identifier = MigrationIdentifier;
pub type Migrations = std::vec::Vec<SupportedMigration>;
pub type Migration = MigrationIdentifier;
pub type ChangedRecords = i64;
pub type RejectionReasonSelection = RejectionReason;
pub type Completions = std::vec::Vec<Completion>;
pub type Rejections = std::vec::Vec<Rejection>;
pub type Year = i64;
pub type Month = i64;
pub type Day = i64;
pub type Major = i64;
pub type Minor = i64;
pub type Patch = i64;
pub type Hour = i64;
pub type Minute = i64;
pub type Second = i64;
pub type SchemaHash = ContractVersion;
pub type StateSequence = i64;
pub type MirroredWriteCount = i64;
pub type RecordFrontier = std::option::Option<i64>;
pub type RecordedAtDate = Date;
pub type RecordedAtTime = Time;
pub type SourceMarker = HandoverMarkerData;
pub type AcceptedMarker = HandoverMarkerData;
pub type SourceVersion = ContractVersion;
pub type TargetVersion = ContractVersion;
pub type Kind = RecordKind;
pub type Payload = RawBytes;
pub type DivergenceReasonSelection = DivergenceReason;
pub type FailureIdentifier = i64;
pub type FinalizedMarker = HandoverMarkerData;
pub type DivergenceIdentifier = i64;
pub type Recovered = bool;
pub type HandoverRejectionReasonSelection = HandoverRejectionReason;
pub type ContractVersion = RawBytes;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct Version {
    pub major: Major,
    pub minor: Minor,
    pub patch: Patch,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SupportedMigration {
    pub component: Component,
    pub source: Source,
    pub target: Target,
    pub identifier: Identifier,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Inspection {
    All,
    Component(ComponentName),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct Attempt {
    pub component: Component,
    pub source: Source,
    pub target: Target,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ReportQuery {
    All,
    Component(ComponentName),
}
pub type InspectionReportedPayload = Migrations;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct Completion {
    pub component: Component,
    pub source: Source,
    pub target: Target,
    pub migration: Migration,
    pub changed_records: ChangedRecords,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum RejectionReason {
    UnsupportedMigration,
    ComponentMismatch,
    MigrationFailed,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct Rejection {
    pub component: Component,
    pub source: Source,
    pub target: Target,
    pub rejection_reason_selection: RejectionReasonSelection,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ReportedPayload {
    pub completions: Completions,
    pub rejections: Rejections,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum UnimplementedReason {
    NotBuiltYet,
    IntegrationNotLanded,
}
pub type RequestUnimplementedPayload = UnimplementedReason;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct Date {
    pub year: Year,
    pub month: Month,
    pub day: Day,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct Time {
    pub hour: Hour,
    pub minute: Minute,
    pub second: Second,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct HandoverMarkerData {
    pub component: Component,
    pub schema_hash: SchemaHash,
    pub state_sequence: StateSequence,
    pub mirrored_write_count: MirroredWriteCount,
    pub record_frontier: RecordFrontier,
    pub recorded_at_date: RecordedAtDate,
    pub recorded_at_time: RecordedAtTime,
}
pub type MarkerRequest = Component;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ReadinessReport {
    pub component: Component,
    pub source_marker: SourceMarker,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct CompletionReport {
    pub component: Component,
    pub accepted_marker: AcceptedMarker,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct MirrorPayload {
    pub component: Component,
    pub source_version: SourceVersion,
    pub target_version: TargetVersion,
    pub kind: Kind,
    pub payload: Payload,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct DivergencePayload {
    pub component: Component,
    pub source_version: SourceVersion,
    pub target_version: TargetVersion,
    pub divergence_reason_selection: DivergenceReasonSelection,
    pub kind: Kind,
    pub payload: Payload,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RecoveryRequest {
    pub component: Component,
    pub failure_identifier: FailureIdentifier,
}
pub type HandoverAcceptance = AcceptedMarker;
pub type HandoverFinalization = FinalizedMarker;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct MirrorAcknowledgement {
    pub component: Component,
    pub mirrored_write_count: MirroredWriteCount,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct DivergenceAcknowledgement {
    pub component: Component,
    pub divergence_identifier: DivergenceIdentifier,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RecoveryResult {
    pub component: Component,
    pub recovered: Recovered,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct HandoverRejection {
    pub component: Component,
    pub handover_rejection_reason_selection: HandoverRejectionReasonSelection,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum HandoverRejectionReason {
    SchemaMismatch,
    StateSequenceAdvanced,
    AlreadyInHandover,
    NotReady,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum DivergenceReason {
    NotRepresentable,
    TargetUnavailable,
    TargetRejected,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
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
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
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
