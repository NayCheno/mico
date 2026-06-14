use std::collections::{HashMap, HashSet};
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Ident(pub String);

impl From<&str> for Ident {
    fn from(value: &str) -> Self {
        Self(value.trim().to_string())
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceSpan {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScalarType {
    Bool,
    UInt(u32),
    Named(Ident),
}

impl ScalarType {
    pub fn parse(s: &str) -> Self {
        let s = s.trim();
        if s == "bool" {
            return ScalarType::Bool;
        }
        if let Some(bits) = s.strip_prefix('u') {
            if let Ok(width) = bits.parse::<u32>() {
                return ScalarType::UInt(width);
            }
        }
        ScalarType::Named(Ident::from(s))
    }
}

impl Serialize for ScalarType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_source_string())
    }
}

impl<'de> Deserialize<'de> for ScalarType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(Self::parse(&value))
    }
}

impl ScalarType {
    fn to_source_string(&self) -> String {
        match self {
            ScalarType::Bool => "bool".to_string(),
            ScalarType::UInt(width) => format!("u{width}"),
            ScalarType::Named(name) => name.0.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClockDomain {
    pub name: Ident,
    pub clock: Ident,
    pub reset: Ident,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Producer,
    Consumer,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldDef {
    pub name: Ident,
    #[serde(rename = "type")]
    pub ty: ScalarType,
    pub role: Role,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractDef {
    pub name: Ident,
    pub expr: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractExpr {
    Ident(Ident),
    Stable(Ident),
    Fire { valid: Ident, ready: Ident },
    And(Box<ContractExpr>, Box<ContractExpr>),
    Or(Box<ContractExpr>, Box<ContractExpr>),
    Implication(Box<ContractExpr>, Box<ContractExpr>),
    Until(Box<ContractExpr>, Box<ContractExpr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContractRequirement {
    StablePayload,
    FireEvent,
    Order,
    NoDrop,
    NoDuplicate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AdapterGuarantee {
    PreservesReadyValid,
    PreservesOrder,
    NoDrop,
    NoDuplicate,
    ZeroExtendPayload,
    SignExtendPayload,
    CdcFifoAssumed,
}

impl AdapterGuarantee {
    fn parse(value: &str) -> Option<Self> {
        match value.trim() {
            "preserves_ready_valid" => Some(Self::PreservesReadyValid),
            "preserves_order" => Some(Self::PreservesOrder),
            "no_drop" | "preserves_no_drop" => Some(Self::NoDrop),
            "no_duplicate" | "preserves_no_duplicate" => Some(Self::NoDuplicate),
            "zero_extend_payload" => Some(Self::ZeroExtendPayload),
            "sign_extend_payload" => Some(Self::SignExtendPayload),
            "cdc_fifo_assumed" => Some(Self::CdcFifoAssumed),
            _ => None,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::PreservesReadyValid => "preserves_ready_valid",
            Self::PreservesOrder => "preserves_order",
            Self::NoDrop => "no_drop",
            Self::NoDuplicate => "no_duplicate",
            Self::ZeroExtendPayload => "zero_extend_payload",
            Self::SignExtendPayload => "sign_extend_payload",
            Self::CdcFifoAssumed => "cdc_fifo_assumed",
        }
    }
}

pub fn parse_contract_expr(expr: &str) -> Result<ContractExpr, String> {
    parse_contract_implication(expr.trim())
}

fn parse_contract_implication(expr: &str) -> Result<ContractExpr, String> {
    if let Some((lhs, rhs)) = split_once_trimmed(expr, "->") {
        return Ok(ContractExpr::Implication(
            Box::new(parse_contract_or(lhs)?),
            Box::new(parse_contract_or(rhs)?),
        ));
    }
    parse_contract_or(expr)
}

fn parse_contract_or(expr: &str) -> Result<ContractExpr, String> {
    if let Some((lhs, rhs)) = split_once_trimmed(expr, "|") {
        return Ok(ContractExpr::Or(
            Box::new(parse_contract_and(lhs)?),
            Box::new(parse_contract_and(rhs)?),
        ));
    }
    parse_contract_and(expr)
}

fn parse_contract_and(expr: &str) -> Result<ContractExpr, String> {
    if let Some((lhs, rhs)) = split_once_trimmed(expr, "&") {
        return Ok(ContractExpr::And(
            Box::new(parse_contract_until(lhs)?),
            Box::new(parse_contract_until(rhs)?),
        ));
    }
    parse_contract_until(expr)
}

fn parse_contract_until(expr: &str) -> Result<ContractExpr, String> {
    if let Some((lhs, rhs)) = split_once_trimmed(expr, " until ") {
        return Ok(ContractExpr::Until(
            Box::new(parse_contract_atom(lhs)?),
            Box::new(parse_contract_atom(rhs)?),
        ));
    }
    parse_contract_atom(expr)
}

fn parse_contract_atom(expr: &str) -> Result<ContractExpr, String> {
    let expr = expr.trim();
    if let Some(inner) = expr
        .strip_prefix("stable(")
        .and_then(|s| s.strip_suffix(')'))
    {
        return Ok(ContractExpr::Stable(Ident::from(inner.trim())));
    }
    if let Some(inner) = expr.strip_prefix("fire(").and_then(|s| s.strip_suffix(')')) {
        let Some((valid, ready)) = split_once_trimmed(inner, ",") else {
            return Err(format!("invalid fire event expression `{expr}`"));
        };
        return Ok(ContractExpr::Fire {
            valid: Ident::from(valid),
            ready: Ident::from(ready),
        });
    }
    if expr.is_empty() {
        Err("empty contract expression".to_string())
    } else {
        Ok(ContractExpr::Ident(Ident::from(expr)))
    }
}

fn split_once_trimmed<'a>(expr: &'a str, needle: &str) -> Option<(&'a str, &'a str)> {
    let (lhs, rhs) = expr.split_once(needle)?;
    Some((lhs.trim(), rhs.trim()))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InterfaceDef {
    pub name: Ident,
    pub domain: Ident,
    pub fields: Vec<FieldDef>,
    pub contracts: Vec<ContractDef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortDir {
    In,
    Out,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PortDef {
    pub name: Ident,
    #[serde(rename = "direction")]
    pub dir: PortDir,
    pub interface: Ident,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModuleDef {
    pub name: Ident,
    pub domain: Ident,
    #[serde(rename = "extern")]
    pub is_extern: bool,
    pub ports: Vec<PortDef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterDef {
    pub name: Ident,
    pub from_interface: Ident,
    pub from_domain: Ident,
    pub to_interface: Ident,
    pub to_domain: Ident,
    pub kind: Ident,
    pub attributes: Vec<(Ident, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstanceDef {
    pub name: Ident,
    pub module: Ident,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Endpoint {
    pub instance: Ident,
    pub port: Ident,
}

impl Endpoint {
    pub fn parse(s: &str) -> Option<Self> {
        let (inst, port) = s.trim().split_once('.')?;
        Some(Self {
            instance: Ident::from(inst),
            port: Ident::from(port),
        })
    }
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.instance, self.port)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectDef {
    pub from: Endpoint,
    pub to: Endpoint,
    pub adapter: Option<Ident>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComposeDef {
    pub name: Ident,
    pub domain: Ident,
    pub instances: Vec<InstanceDef>,
    pub connections: Vec<ConnectDef>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Design {
    pub clock_domains: Vec<ClockDomain>,
    pub interfaces: Vec<InterfaceDef>,
    pub modules: Vec<ModuleDef>,
    pub adapters: Vec<AdapterDef>,
    pub composes: Vec<ComposeDef>,
}

pub const MICO_AST_SCHEMA_VERSION: &str = "mico.ast.v0";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AstDocument {
    pub schema_version: String,
    pub kind: String,
    pub clock_domains: Vec<ClockDomain>,
    pub interfaces: Vec<InterfaceDef>,
    pub modules: Vec<ModuleDef>,
    pub adapters: Vec<AstAdapterDef>,
    pub composes: Vec<ComposeDef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AstAdapterDef {
    pub name: Ident,
    pub from_interface: Ident,
    pub from_domain: Ident,
    pub to_interface: Ident,
    pub to_domain: Ident,
    pub kind: Ident,
    pub attributes: Vec<AstAttribute>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AstAttribute {
    pub name: Ident,
    pub value: String,
}

impl AstDocument {
    pub fn from_design(design: &Design) -> Self {
        Self {
            schema_version: MICO_AST_SCHEMA_VERSION.to_string(),
            kind: "design".to_string(),
            clock_domains: design.clock_domains.clone(),
            interfaces: design.interfaces.clone(),
            modules: design.modules.clone(),
            adapters: design.adapters.iter().map(AstAdapterDef::from).collect(),
            composes: design.composes.clone(),
        }
    }

    pub fn into_design(self) -> Result<Design, Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();
        if self.schema_version != MICO_AST_SCHEMA_VERSION {
            diagnostics.push(
                Diagnostic::error(
                    "JsonSchemaError",
                    format!(
                        "expected schema_version `{}`, found `{}`",
                        MICO_AST_SCHEMA_VERSION, self.schema_version
                    ),
                )
                .with_label(
                    LabelStyle::Primary,
                    "unsupported MICO JSON AST schema version",
                )
                .with_node("schema_version", self.schema_version)
                .with_repair(RepairAction::FixSyntax),
            );
        }
        if self.kind != "design" {
            diagnostics.push(
                Diagnostic::error(
                    "JsonSchemaError",
                    format!("expected JSON AST kind `design`, found `{}`", self.kind),
                )
                .with_label(LabelStyle::Primary, "unsupported MICO JSON AST kind")
                .with_node("kind", self.kind)
                .with_repair(RepairAction::FixSyntax),
            );
        }

        if diagnostics.is_empty() {
            Ok(Design {
                clock_domains: self.clock_domains,
                interfaces: self.interfaces,
                modules: self.modules,
                adapters: self.adapters.into_iter().map(AdapterDef::from).collect(),
                composes: self.composes,
            })
        } else {
            Err(diagnostics)
        }
    }
}

impl From<&AdapterDef> for AstAdapterDef {
    fn from(adapter: &AdapterDef) -> Self {
        Self {
            name: adapter.name.clone(),
            from_interface: adapter.from_interface.clone(),
            from_domain: adapter.from_domain.clone(),
            to_interface: adapter.to_interface.clone(),
            to_domain: adapter.to_domain.clone(),
            kind: adapter.kind.clone(),
            attributes: adapter
                .attributes
                .iter()
                .map(|(name, value)| AstAttribute {
                    name: name.clone(),
                    value: value.clone(),
                })
                .collect(),
        }
    }
}

impl From<AstAdapterDef> for AdapterDef {
    fn from(adapter: AstAdapterDef) -> Self {
        Self {
            name: adapter.name,
            from_interface: adapter.from_interface,
            from_domain: adapter.from_domain,
            to_interface: adapter.to_interface,
            to_domain: adapter.to_domain,
            kind: adapter.kind,
            attributes: adapter
                .attributes
                .into_iter()
                .map(|attr| (attr.name, attr.value))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResetPolarity {
    ActiveHigh,
    ActiveLow,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedClockDomain {
    pub name: Ident,
    pub clock: Ident,
    pub reset: Ident,
    pub reset_polarity: ResetPolarity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolKind {
    ReadyValid,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceProtocol {
    pub kind: ProtocolKind,
    pub payload_fields: Vec<Ident>,
    pub valid: Option<Ident>,
    pub ready: Option<Ident>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedField {
    pub name: Ident,
    pub ty: ScalarType,
    pub width_bits: Option<u32>,
    pub role: Role,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedInterface {
    pub name: Ident,
    pub domain: Ident,
    pub fields: Vec<TypedField>,
    pub contracts: Vec<ContractDef>,
    pub protocol: InterfaceProtocol,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedPort {
    pub name: Ident,
    pub dir: PortDir,
    pub interface: Ident,
    pub domain: Ident,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedModule {
    pub name: Ident,
    pub domain: Ident,
    pub is_extern: bool,
    pub ports: Vec<TypedPort>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterKind {
    CdcFifo,
    WidthAdapter,
    SkidBuffer,
    Pipeline,
    Custom(Ident),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedAdapter {
    pub name: Ident,
    pub from_interface: Ident,
    pub from_domain: Ident,
    pub to_interface: Ident,
    pub to_domain: Ident,
    pub kind: AdapterKind,
    pub attributes: Vec<(Ident, String)>,
    pub contracts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedInstance {
    pub name: Ident,
    pub module: Ident,
    pub domain: Ident,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedEndpoint {
    pub endpoint: Endpoint,
    pub module: Ident,
    pub port_dir: PortDir,
    pub interface: Ident,
    pub domain: Ident,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedConnectionContracts {
    pub source_interface: Vec<ContractDef>,
    pub sink_interface: Vec<ContractDef>,
    pub adapter_contracts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedConnection {
    pub from: TypedEndpoint,
    pub to: TypedEndpoint,
    pub adapter: Option<Ident>,
    pub adapter_kind: Option<AdapterKind>,
    pub contracts: TypedConnectionContracts,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedCompose {
    pub name: Ident,
    pub domain: Ident,
    pub instances: Vec<TypedInstance>,
    pub connections: Vec<TypedConnection>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedDesign {
    pub clock_domains: Vec<TypedClockDomain>,
    pub interfaces: Vec<TypedInterface>,
    pub modules: Vec<TypedModule>,
    pub adapters: Vec<TypedAdapter>,
    pub composes: Vec<TypedCompose>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Note,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Note => "note",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: &'static str,
    pub message: String,
    pub span: Option<SourceSpan>,
    pub labels: Vec<DiagnosticLabel>,
    pub nodes: Vec<DiagnosticNode>,
    pub hints: Vec<String>,
    pub repair_action: Option<RepairAction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelStyle {
    Primary,
    Secondary,
}

impl LabelStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            LabelStyle::Primary => "primary",
            LabelStyle::Secondary => "secondary",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticLabel {
    pub style: LabelStyle,
    pub message: String,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticNode {
    pub kind: &'static str,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairAction {
    CheckFile,
    FixSyntax,
    AddDeclaration,
    RenameDuplicate,
    FixEndpoint,
    ReverseConnection,
    UseAdapter,
    FixAdapterDeclaration,
    UseKnownAdapterKind,
    MatchInterface,
    AddContract,
    FixProtocol,
    FixWidth,
    ReportCompilerBug,
}

impl RepairAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            RepairAction::CheckFile => "check_file",
            RepairAction::FixSyntax => "fix_syntax",
            RepairAction::AddDeclaration => "add_declaration",
            RepairAction::RenameDuplicate => "rename_duplicate",
            RepairAction::FixEndpoint => "fix_endpoint",
            RepairAction::ReverseConnection => "reverse_connection",
            RepairAction::UseAdapter => "use_adapter",
            RepairAction::FixAdapterDeclaration => "fix_adapter_declaration",
            RepairAction::UseKnownAdapterKind => "use_known_adapter_kind",
            RepairAction::MatchInterface => "match_interface",
            RepairAction::AddContract => "add_contract",
            RepairAction::FixProtocol => "fix_protocol",
            RepairAction::FixWidth => "fix_width",
            RepairAction::ReportCompilerBug => "report_compiler_bug",
        }
    }
}

impl Diagnostic {
    pub fn error(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code,
            message: message.into(),
            span: None,
            labels: Vec::new(),
            nodes: Vec::new(),
            hints: Vec::new(),
            repair_action: None,
        }
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hints.push(hint.into());
        self
    }

    pub fn with_span(mut self, span: SourceSpan) -> Self {
        self.span = Some(span);
        self
    }

    pub fn with_label(mut self, style: LabelStyle, message: impl Into<String>) -> Self {
        self.labels.push(DiagnosticLabel {
            style,
            message: message.into(),
            span: None,
        });
        self
    }

    pub fn with_spanned_label(
        mut self,
        style: LabelStyle,
        message: impl Into<String>,
        span: SourceSpan,
    ) -> Self {
        self.labels.push(DiagnosticLabel {
            style,
            message: message.into(),
            span: Some(span),
        });
        self
    }

    pub fn with_node(mut self, kind: &'static str, name: impl fmt::Display) -> Self {
        self.nodes.push(DiagnosticNode {
            kind,
            name: name.to_string(),
        });
        self
    }

    pub fn with_repair(mut self, action: RepairAction) -> Self {
        self.repair_action = Some(action);
        self
    }
}

pub fn check_design(design: &Design) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    check_top_level_duplicates(design, &mut diags);

    let clock_names: HashSet<_> = design
        .clock_domains
        .iter()
        .map(|d| d.name.clone())
        .collect();
    let interfaces: HashMap<_, _> = design
        .interfaces
        .iter()
        .map(|i| (i.name.clone(), i))
        .collect();
    let modules: HashMap<_, _> = design.modules.iter().map(|m| (m.name.clone(), m)).collect();
    let adapters: HashMap<_, _> = design
        .adapters
        .iter()
        .map(|a| (a.name.clone(), a))
        .collect();

    for iface in &design.interfaces {
        check_interface_fields(iface, &mut diags);
        if !clock_names.contains(&iface.domain) {
            diags.push(
                Diagnostic::error(
                    "UnknownClockDomain",
                    format!(
                        "interface `{}` references unknown clock domain `{}`",
                        iface.name, iface.domain
                    ),
                )
                .with_label(LabelStyle::Primary, "interface domain is unresolved")
                .with_node("interface", &iface.name)
                .with_node("clock_domain", &iface.domain)
                .with_repair(RepairAction::AddDeclaration),
            );
        }
    }

    for module in &design.modules {
        check_module_ports(module, &mut diags);
        if !clock_names.contains(&module.domain) {
            diags.push(
                Diagnostic::error(
                    "UnknownClockDomain",
                    format!(
                        "module `{}` references unknown clock domain `{}`",
                        module.name, module.domain
                    ),
                )
                .with_label(LabelStyle::Primary, "module domain is unresolved")
                .with_node("module", &module.name)
                .with_node("clock_domain", &module.domain)
                .with_repair(RepairAction::AddDeclaration),
            );
        }
        for port in &module.ports {
            if !interfaces.contains_key(&port.interface) {
                diags.push(
                    Diagnostic::error(
                        "UnknownInterface",
                        format!(
                            "module `{}` port `{}` references unknown interface `{}`",
                            module.name, port.name, port.interface
                        ),
                    )
                    .with_label(LabelStyle::Primary, "port interface is unresolved")
                    .with_node("module", &module.name)
                    .with_node("port", format!("{}.{}", module.name, port.name))
                    .with_node("interface", &port.interface)
                    .with_repair(RepairAction::AddDeclaration),
                );
            }
        }
    }

    for adapter in &design.adapters {
        check_adapter_decl(adapter, &clock_names, &interfaces, &mut diags);
    }

    for compose in &design.composes {
        if !clock_names.contains(&compose.domain) {
            diags.push(
                Diagnostic::error(
                    "UnknownClockDomain",
                    format!(
                        "compose `{}` references unknown clock domain `{}`",
                        compose.name, compose.domain
                    ),
                )
                .with_label(LabelStyle::Primary, "compose domain is unresolved")
                .with_node("compose", &compose.name)
                .with_node("clock_domain", &compose.domain)
                .with_repair(RepairAction::AddDeclaration),
            );
        }
        check_compose_instances(compose, &mut diags);

        let instances: HashMap<_, _> = compose
            .instances
            .iter()
            .map(|i| (i.name.clone(), i))
            .collect();
        for inst in &compose.instances {
            if !modules.contains_key(&inst.module) {
                diags.push(
                    Diagnostic::error(
                        "UnknownModule",
                        format!(
                            "compose `{}` instantiates unknown module `{}` as `{}`",
                            compose.name, inst.module, inst.name
                        ),
                    )
                    .with_label(LabelStyle::Primary, "instance module is unresolved")
                    .with_node("compose", &compose.name)
                    .with_node("instance", &inst.name)
                    .with_node("module", &inst.module)
                    .with_repair(RepairAction::AddDeclaration),
                );
            }
        }

        for conn in &compose.connections {
            let src = resolve_endpoint(&instances, &modules, &conn.from);
            let dst = resolve_endpoint(&instances, &modules, &conn.to);

            match (src, dst) {
                (Ok((src_mod, src_port)), Ok((dst_mod, dst_port))) => {
                    if src_port.dir != PortDir::Out {
                        diags.push(
                            Diagnostic::error(
                                "DirectionMismatch",
                                format!("source endpoint `{}` is not an output port", conn.from),
                            )
                            .with_label(LabelStyle::Primary, "source endpoint must be an output")
                            .with_node("endpoint", &conn.from)
                            .with_node("port", format!("{}.{}", src_mod.name, src_port.name))
                            .with_repair(RepairAction::ReverseConnection),
                        );
                    }
                    if dst_port.dir != PortDir::In {
                        diags.push(
                            Diagnostic::error(
                                "DirectionMismatch",
                                format!("sink endpoint `{}` is not an input port", conn.to),
                            )
                            .with_label(LabelStyle::Primary, "sink endpoint must be an input")
                            .with_node("endpoint", &conn.to)
                            .with_node("port", format!("{}.{}", dst_mod.name, dst_port.name))
                            .with_repair(RepairAction::ReverseConnection),
                        );
                    }

                    let same_interface = src_port.interface == dst_port.interface;
                    let same_domain = src_mod.domain == dst_mod.domain;

                    if let Some(adapter_name) = &conn.adapter {
                        match adapters.get(adapter_name) {
                            Some(adapter) => {
                                if adapter.from_interface != src_port.interface
                                    || adapter.to_interface != dst_port.interface
                                    || adapter.from_domain != src_mod.domain
                                    || adapter.to_domain != dst_mod.domain
                                {
                                    diags.push(
                                        Diagnostic::error(
                                            "AdapterMismatch",
                                            format!(
                                                "adapter `{}` does not match `{}` ({:?}@{}) -> `{}` ({:?}@{})",
                                                adapter_name,
                                                conn.from,
                                                src_port.interface,
                                                src_mod.domain,
                                                conn.to,
                                                dst_port.interface,
                                                dst_mod.domain
                                            ),
                                        )
                                        .with_label(
                                            LabelStyle::Primary,
                                            "adapter declaration does not match connection endpoints",
                                        )
                                        .with_node("adapter", adapter_name)
                                        .with_node("endpoint", &conn.from)
                                        .with_node("endpoint", &conn.to)
                                        .with_repair(RepairAction::FixAdapterDeclaration),
                                    );
                                } else if let (Some(src_interface), Some(dst_interface)) = (
                                    interfaces.get(&src_port.interface),
                                    interfaces.get(&dst_port.interface),
                                ) {
                                    check_adapter_application(
                                        adapter,
                                        src_interface,
                                        dst_interface,
                                        &mut diags,
                                    );
                                }
                            }
                            None => diags.push(
                                Diagnostic::error(
                                    "UnknownAdapter",
                                    format!(
                                        "connection `{}` -> `{}` references unknown adapter `{}`",
                                        conn.from, conn.to, adapter_name
                                    ),
                                )
                                .with_label(LabelStyle::Primary, "connection adapter is unresolved")
                                .with_node("adapter", adapter_name)
                                .with_node("endpoint", &conn.from)
                                .with_node("endpoint", &conn.to)
                                .with_repair(RepairAction::AddDeclaration),
                            ),
                        }
                    } else {
                        if same_interface && same_domain {
                            continue;
                        }
                        if !same_interface {
                            diags.push(
                                Diagnostic::error(
                                    "InterfaceMismatch",
                                    format!(
                                        "direct connection `{}` -> `{}` uses incompatible interfaces `{}` and `{}`",
                                        conn.from, conn.to, src_port.interface, dst_port.interface
                                    ),
                                )
                                .with_label(
                                    LabelStyle::Primary,
                                    "direct connection crosses interface types",
                                )
                                .with_node("endpoint", &conn.from)
                                .with_node("endpoint", &conn.to)
                                .with_node("interface", &src_port.interface)
                                .with_node("interface", &dst_port.interface)
                                .with_hint("declare an explicit adapter or use matching interfaces")
                                .with_repair(RepairAction::UseAdapter),
                            );
                        }
                        if !same_domain {
                            diags.push(
                                Diagnostic::error(
                                    "ClockDomainMismatch",
                                    format!(
                                        "direct connection `{}` -> `{}` crosses domains `{}` and `{}`",
                                        conn.from, conn.to, src_mod.domain, dst_mod.domain
                                    ),
                                )
                                .with_label(
                                    LabelStyle::Primary,
                                    "direct connection crosses clock domains",
                                )
                                .with_node("endpoint", &conn.from)
                                .with_node("endpoint", &conn.to)
                                .with_node("clock_domain", &src_mod.domain)
                                .with_node("clock_domain", &dst_mod.domain)
                                .with_hint("use an explicit CDC adapter such as AsyncFifo")
                                .with_repair(RepairAction::UseAdapter),
                            );
                        }
                    }
                }
                (Err(e), _) | (_, Err(e)) => diags.push(e),
            }
        }
    }

    diags
}

fn check_top_level_duplicates(design: &Design, diags: &mut Vec<Diagnostic>) {
    check_duplicate_idents(
        design.clock_domains.iter().map(|item| &item.name),
        "clock domain",
        diags,
    );
    check_duplicate_idents(
        design.interfaces.iter().map(|item| &item.name),
        "interface",
        diags,
    );
    check_duplicate_idents(
        design.modules.iter().map(|item| &item.name),
        "module",
        diags,
    );
    check_duplicate_idents(
        design.adapters.iter().map(|item| &item.name),
        "adapter",
        diags,
    );
    check_duplicate_idents(
        design.composes.iter().map(|item| &item.name),
        "compose",
        diags,
    );
}

fn check_duplicate_idents<'a>(
    names: impl Iterator<Item = &'a Ident>,
    label: &'static str,
    diags: &mut Vec<Diagnostic>,
) {
    let mut seen = HashSet::new();
    for name in names {
        if !seen.insert(name.clone()) {
            diags.push(
                Diagnostic::error(
                    "DuplicateDeclaration",
                    format!("duplicate {label} declaration `{name}`"),
                )
                .with_label(LabelStyle::Primary, format!("duplicate {label} name"))
                .with_node(label, name)
                .with_repair(RepairAction::RenameDuplicate),
            );
        }
    }
}

fn check_interface_fields(interface: &InterfaceDef, diags: &mut Vec<Diagnostic>) {
    let mut fields = HashSet::new();
    for field in &interface.fields {
        if !fields.insert(field.name.clone()) {
            diags.push(
                Diagnostic::error(
                    "DuplicateField",
                    format!(
                        "interface `{}` declares duplicate field `{}`",
                        interface.name, field.name
                    ),
                )
                .with_label(LabelStyle::Primary, "interface field name is duplicated")
                .with_node("interface", &interface.name)
                .with_node("field", format!("{}.{}", interface.name, field.name))
                .with_repair(RepairAction::RenameDuplicate),
            );
        }
    }
}

fn check_module_ports(module: &ModuleDef, diags: &mut Vec<Diagnostic>) {
    let mut ports = HashSet::new();
    for port in &module.ports {
        if !ports.insert(port.name.clone()) {
            diags.push(
                Diagnostic::error(
                    "DuplicatePort",
                    format!(
                        "module `{}` declares duplicate port `{}`",
                        module.name, port.name
                    ),
                )
                .with_label(LabelStyle::Primary, "module port name is duplicated")
                .with_node("module", &module.name)
                .with_node("port", format!("{}.{}", module.name, port.name))
                .with_repair(RepairAction::RenameDuplicate),
            );
        }
    }
}

fn check_compose_instances(compose: &ComposeDef, diags: &mut Vec<Diagnostic>) {
    let mut instances = HashSet::new();
    for instance in &compose.instances {
        if !instances.insert(instance.name.clone()) {
            diags.push(
                Diagnostic::error(
                    "DuplicateInstance",
                    format!(
                        "compose `{}` declares duplicate instance `{}`",
                        compose.name, instance.name
                    ),
                )
                .with_label(LabelStyle::Primary, "compose instance name is duplicated")
                .with_node("compose", &compose.name)
                .with_node("instance", &instance.name)
                .with_repair(RepairAction::RenameDuplicate),
            );
        }
    }
}

fn check_adapter_decl(
    adapter: &AdapterDef,
    clock_names: &HashSet<Ident>,
    interfaces: &HashMap<Ident, &InterfaceDef>,
    diags: &mut Vec<Diagnostic>,
) {
    if !interfaces.contains_key(&adapter.from_interface) {
        diags.push(
            Diagnostic::error(
                "UnknownInterface",
                format!(
                    "adapter `{}` references unknown source interface `{}`",
                    adapter.name, adapter.from_interface
                ),
            )
            .with_label(
                LabelStyle::Primary,
                "adapter source interface is unresolved",
            )
            .with_node("adapter", &adapter.name)
            .with_node("interface", &adapter.from_interface)
            .with_repair(RepairAction::AddDeclaration),
        );
    }
    if !interfaces.contains_key(&adapter.to_interface) {
        diags.push(
            Diagnostic::error(
                "UnknownInterface",
                format!(
                    "adapter `{}` references unknown destination interface `{}`",
                    adapter.name, adapter.to_interface
                ),
            )
            .with_label(
                LabelStyle::Primary,
                "adapter destination interface is unresolved",
            )
            .with_node("adapter", &adapter.name)
            .with_node("interface", &adapter.to_interface)
            .with_repair(RepairAction::AddDeclaration),
        );
    }
    if !clock_names.contains(&adapter.from_domain) {
        diags.push(
            Diagnostic::error(
                "UnknownClockDomain",
                format!(
                    "adapter `{}` references unknown source domain `{}`",
                    adapter.name, adapter.from_domain
                ),
            )
            .with_label(LabelStyle::Primary, "adapter source domain is unresolved")
            .with_node("adapter", &adapter.name)
            .with_node("clock_domain", &adapter.from_domain)
            .with_repair(RepairAction::AddDeclaration),
        );
    }
    if !clock_names.contains(&adapter.to_domain) {
        diags.push(
            Diagnostic::error(
                "UnknownClockDomain",
                format!(
                    "adapter `{}` references unknown destination domain `{}`",
                    adapter.name, adapter.to_domain
                ),
            )
            .with_label(
                LabelStyle::Primary,
                "adapter destination domain is unresolved",
            )
            .with_node("adapter", &adapter.name)
            .with_node("clock_domain", &adapter.to_domain)
            .with_repair(RepairAction::AddDeclaration),
        );
    }

    let Some(kind) = recognized_adapter_kind(&adapter.kind) else {
        diags.push(
            Diagnostic::error(
                "UnknownAdapterKind",
                format!(
                    "adapter `{}` uses unsupported kind `{}`",
                    adapter.name, adapter.kind
                ),
            )
            .with_label(LabelStyle::Primary, "adapter kind is not in the v0 library")
            .with_node("adapter", &adapter.name)
            .with_node("adapter_kind", &adapter.kind)
            .with_hint("use cdc_fifo, width_adapter, skid_buffer, pipeline, or custom")
            .with_repair(RepairAction::UseKnownAdapterKind),
        );
        return;
    };

    let (Some(from_interface), Some(to_interface)) = (
        interfaces.get(&adapter.from_interface),
        interfaces.get(&adapter.to_interface),
    ) else {
        return;
    };

    check_adapter_kind_rules(adapter, &kind, from_interface, to_interface, diags);
}

fn check_adapter_application(
    adapter: &AdapterDef,
    from_interface: &InterfaceDef,
    to_interface: &InterfaceDef,
    diags: &mut Vec<Diagnostic>,
) {
    if recognized_adapter_kind(&adapter.kind).is_none() {
        return;
    }

    let kind = AdapterKind::from_ident(&adapter.kind);
    let guarantees = adapter_guarantees(adapter, &kind, diags);
    let source_requirements = interface_contract_requirements(from_interface);
    let requirements = interface_contract_requirements(to_interface);
    if from_interface.name != to_interface.name
        && requirements.iter().any(|requirement| {
            !source_requirements.contains(requirement)
                || !requirement_covered(*requirement, &guarantees)
        })
    {
        diags.push(
            Diagnostic::error(
                "ContractViolation",
                format!(
                    "source interface `{}` and adapter `{}` do not cover all v0 contracts required by interface `{}`",
                    from_interface.name,
                    adapter.name, to_interface.name
                ),
            )
            .with_label(
                LabelStyle::Primary,
                "source and adapter guarantees do not cover sink contract requirements",
            )
            .with_node("interface", &from_interface.name)
            .with_node("adapter", &adapter.name)
            .with_node("interface", &to_interface.name)
            .with_hint("add a known adapter contract such as `contract preserves_ready_valid;`")
            .with_repair(RepairAction::AddContract),
        );
    }
}

fn check_adapter_kind_rules(
    adapter: &AdapterDef,
    kind: &AdapterKind,
    from_interface: &InterfaceDef,
    to_interface: &InterfaceDef,
    diags: &mut Vec<Diagnostic>,
) {
    match kind {
        AdapterKind::CdcFifo => {
            if adapter.from_domain == adapter.to_domain {
                diags.push(
                    Diagnostic::error(
                        "AdapterMismatch",
                        format!(
                            "cdc_fifo adapter `{}` must connect different clock domains",
                            adapter.name
                        ),
                    )
                    .with_label(LabelStyle::Primary, "cdc_fifo requires distinct domains")
                    .with_node("adapter", &adapter.name)
                    .with_node("clock_domain", &adapter.from_domain)
                    .with_node("clock_domain", &adapter.to_domain)
                    .with_repair(RepairAction::FixAdapterDeclaration),
                );
            }
            require_ready_valid(adapter, from_interface, to_interface, diags);
            require_equal_payload_width(adapter, from_interface, to_interface, diags);
        }
        AdapterKind::WidthAdapter => {
            if adapter.from_domain != adapter.to_domain {
                diags.push(
                    Diagnostic::error(
                        "AdapterMismatch",
                        format!(
                            "width_adapter `{}` must stay within one clock domain",
                            adapter.name
                        ),
                    )
                    .with_label(LabelStyle::Primary, "width adapter cannot cross domains")
                    .with_node("adapter", &adapter.name)
                    .with_node("clock_domain", &adapter.from_domain)
                    .with_node("clock_domain", &adapter.to_domain)
                    .with_repair(RepairAction::FixAdapterDeclaration),
                );
            }
            require_ready_valid(adapter, from_interface, to_interface, diags);
            require_changed_payload_width(adapter, from_interface, to_interface, diags);
        }
        AdapterKind::SkidBuffer | AdapterKind::Pipeline => {
            if adapter.from_domain != adapter.to_domain {
                diags.push(
                    Diagnostic::error(
                        "AdapterMismatch",
                        format!(
                            "{}`{}` must stay within one clock domain",
                            adapter_kind_label(kind),
                            adapter.name
                        ),
                    )
                    .with_label(LabelStyle::Primary, "latency adapter cannot cross domains")
                    .with_node("adapter", &adapter.name)
                    .with_node("clock_domain", &adapter.from_domain)
                    .with_node("clock_domain", &adapter.to_domain)
                    .with_repair(RepairAction::FixAdapterDeclaration),
                );
            }
            if adapter.from_interface != adapter.to_interface {
                diags.push(
                    Diagnostic::error(
                        "AdapterMismatch",
                        format!(
                            "{}`{}` must preserve interface type `{}`",
                            adapter_kind_label(kind),
                            adapter.name,
                            adapter.from_interface
                        ),
                    )
                    .with_label(
                        LabelStyle::Primary,
                        "latency adapter must preserve interface type",
                    )
                    .with_node("adapter", &adapter.name)
                    .with_node("interface", &adapter.from_interface)
                    .with_node("interface", &adapter.to_interface)
                    .with_repair(RepairAction::FixAdapterDeclaration),
                );
            }
        }
        AdapterKind::Custom(_) => {}
    }
}

fn recognized_adapter_kind(kind: &Ident) -> Option<AdapterKind> {
    let parsed = AdapterKind::from_ident(kind);
    match &parsed {
        AdapterKind::Custom(custom) if custom.0 != "custom" => None,
        _ => Some(parsed),
    }
}

fn require_ready_valid(
    adapter: &AdapterDef,
    from_interface: &InterfaceDef,
    to_interface: &InterfaceDef,
    diags: &mut Vec<Diagnostic>,
) {
    let from_protocol = infer_protocol(from_interface);
    let to_protocol = infer_protocol(to_interface);
    if from_protocol.kind != ProtocolKind::ReadyValid
        || to_protocol.kind != ProtocolKind::ReadyValid
    {
        diags.push(
            Diagnostic::error(
                "ProtocolMismatch",
                format!(
                    "adapter `{}` requires ready/valid source and sink interfaces",
                    adapter.name
                ),
            )
            .with_label(
                LabelStyle::Primary,
                "adapter protocol requirement is not met",
            )
            .with_node("adapter", &adapter.name)
            .with_node("interface", &from_interface.name)
            .with_node("interface", &to_interface.name)
            .with_repair(RepairAction::FixProtocol),
        );
    }
}

fn require_equal_payload_width(
    adapter: &AdapterDef,
    from_interface: &InterfaceDef,
    to_interface: &InterfaceDef,
    diags: &mut Vec<Diagnostic>,
) {
    if let (Some(from_width), Some(to_width)) = (
        ready_valid_payload_width(from_interface),
        ready_valid_payload_width(to_interface),
    ) {
        if from_width != to_width {
            diags.push(
                Diagnostic::error(
                    "WidthMismatch",
                    format!(
                        "adapter `{}` requires equal ready/valid payload widths, found {} and {}",
                        adapter.name, from_width, to_width
                    ),
                )
                .with_label(
                    LabelStyle::Primary,
                    "adapter payload widths are incompatible",
                )
                .with_node("adapter", &adapter.name)
                .with_node("interface", &from_interface.name)
                .with_node("interface", &to_interface.name)
                .with_repair(RepairAction::FixWidth),
            );
        }
    }
}

fn require_changed_payload_width(
    adapter: &AdapterDef,
    from_interface: &InterfaceDef,
    to_interface: &InterfaceDef,
    diags: &mut Vec<Diagnostic>,
) {
    match (
        ready_valid_payload_width(from_interface),
        ready_valid_payload_width(to_interface),
    ) {
        (Some(from_width), Some(to_width)) if from_width != to_width => {}
        (Some(width), Some(_)) => diags.push(
            Diagnostic::error(
                "AdapterMismatch",
                format!(
                    "width_adapter `{}` must change payload width, but both sides are {} bits",
                    adapter.name, width
                ),
            )
            .with_label(LabelStyle::Primary, "width adapter must change payload width")
            .with_node("adapter", &adapter.name)
            .with_node("interface", &from_interface.name)
            .with_node("interface", &to_interface.name)
            .with_repair(RepairAction::FixAdapterDeclaration),
        ),
        _ => diags.push(
            Diagnostic::error(
                "WidthMismatch",
                format!(
                    "width_adapter `{}` requires exactly one known ready/valid payload width on each side",
                    adapter.name
                ),
            )
            .with_label(LabelStyle::Primary, "width adapter payload width is unknown")
            .with_node("adapter", &adapter.name)
            .with_node("interface", &from_interface.name)
            .with_node("interface", &to_interface.name)
            .with_repair(RepairAction::FixWidth),
        ),
    }
}

fn ready_valid_payload_width(interface: &InterfaceDef) -> Option<u32> {
    let protocol = infer_protocol(interface);
    if protocol.kind != ProtocolKind::ReadyValid || protocol.payload_fields.len() != 1 {
        return None;
    }
    let payload_name = &protocol.payload_fields[0];
    interface
        .fields
        .iter()
        .find(|field| field.name == *payload_name)
        .and_then(|field| scalar_width_bits(&field.ty))
}

fn adapter_kind_label(kind: &AdapterKind) -> &'static str {
    match kind {
        AdapterKind::CdcFifo => "cdc_fifo adapter ",
        AdapterKind::WidthAdapter => "width_adapter ",
        AdapterKind::SkidBuffer => "skid_buffer ",
        AdapterKind::Pipeline => "pipeline ",
        AdapterKind::Custom(_) => "custom adapter ",
    }
}

pub fn build_typed_ir(design: &Design) -> Result<TypedDesign, Vec<Diagnostic>> {
    let mut diagnostics = check_design(design);
    if diagnostics.iter().any(|d| d.severity == Severity::Error) {
        return Err(diagnostics);
    }

    let clock_domains = design
        .clock_domains
        .iter()
        .map(|domain| TypedClockDomain {
            name: domain.name.clone(),
            clock: domain.clock.clone(),
            reset: domain.reset.clone(),
            reset_polarity: infer_reset_polarity(&domain.reset),
        })
        .collect::<Vec<_>>();

    let interfaces = design
        .interfaces
        .iter()
        .map(|interface| TypedInterface {
            name: interface.name.clone(),
            domain: interface.domain.clone(),
            fields: interface
                .fields
                .iter()
                .map(|field| TypedField {
                    name: field.name.clone(),
                    ty: field.ty.clone(),
                    width_bits: scalar_width_bits(&field.ty),
                    role: field.role.clone(),
                })
                .collect(),
            contracts: interface.contracts.clone(),
            protocol: infer_protocol(interface),
        })
        .collect::<Vec<_>>();

    let modules = design
        .modules
        .iter()
        .map(|module| TypedModule {
            name: module.name.clone(),
            domain: module.domain.clone(),
            is_extern: module.is_extern,
            ports: module
                .ports
                .iter()
                .map(|port| TypedPort {
                    name: port.name.clone(),
                    dir: port.dir,
                    interface: port.interface.clone(),
                    domain: module.domain.clone(),
                })
                .collect(),
        })
        .collect::<Vec<_>>();

    let adapters = design
        .adapters
        .iter()
        .map(|adapter| TypedAdapter {
            name: adapter.name.clone(),
            from_interface: adapter.from_interface.clone(),
            from_domain: adapter.from_domain.clone(),
            to_interface: adapter.to_interface.clone(),
            to_domain: adapter.to_domain.clone(),
            kind: AdapterKind::from_ident(&adapter.kind),
            attributes: adapter.attributes.clone(),
            contracts: adapter_contracts(adapter),
        })
        .collect::<Vec<_>>();

    let module_map: HashMap<_, _> = design
        .modules
        .iter()
        .map(|module| (module.name.clone(), module))
        .collect();
    let interface_map: HashMap<_, _> = design
        .interfaces
        .iter()
        .map(|interface| (interface.name.clone(), interface))
        .collect();
    let adapter_map: HashMap<_, _> = design
        .adapters
        .iter()
        .map(|adapter| (adapter.name.clone(), adapter))
        .collect();

    let mut composes = Vec::new();
    for compose in &design.composes {
        let instance_map: HashMap<_, _> = compose
            .instances
            .iter()
            .map(|instance| (instance.name.clone(), instance))
            .collect();

        let mut typed_instances = Vec::new();
        for instance in &compose.instances {
            match module_map.get(&instance.module) {
                Some(module) => typed_instances.push(TypedInstance {
                    name: instance.name.clone(),
                    module: instance.module.clone(),
                    domain: module.domain.clone(),
                }),
                None => diagnostics.push(
                    Diagnostic::error(
                        "InternalIrError",
                        format!(
                            "cannot lower instance `{}` because module `{}` is unresolved",
                            instance.name, instance.module
                        ),
                    )
                    .with_label(
                        LabelStyle::Primary,
                        "typed IR lowering saw unresolved module",
                    )
                    .with_node("instance", &instance.name)
                    .with_node("module", &instance.module)
                    .with_repair(RepairAction::ReportCompilerBug),
                ),
            }
        }

        let mut typed_connections = Vec::new();
        for connection in &compose.connections {
            match (
                resolve_endpoint(&instance_map, &module_map, &connection.from),
                resolve_endpoint(&instance_map, &module_map, &connection.to),
            ) {
                (Ok((src_module, src_port)), Ok((dst_module, dst_port))) => {
                    let adapter = connection
                        .adapter
                        .as_ref()
                        .and_then(|name| adapter_map.get(name));
                    let contracts = TypedConnectionContracts {
                        source_interface: interface_map
                            .get(&src_port.interface)
                            .map(|interface| interface.contracts.clone())
                            .unwrap_or_default(),
                        sink_interface: interface_map
                            .get(&dst_port.interface)
                            .map(|interface| interface.contracts.clone())
                            .unwrap_or_default(),
                        adapter_contracts: adapter
                            .map(|adapter| adapter_contracts(adapter))
                            .unwrap_or_default(),
                    };

                    typed_connections.push(TypedConnection {
                        from: TypedEndpoint {
                            endpoint: connection.from.clone(),
                            module: src_module.name.clone(),
                            port_dir: src_port.dir,
                            interface: src_port.interface.clone(),
                            domain: src_module.domain.clone(),
                        },
                        to: TypedEndpoint {
                            endpoint: connection.to.clone(),
                            module: dst_module.name.clone(),
                            port_dir: dst_port.dir,
                            interface: dst_port.interface.clone(),
                            domain: dst_module.domain.clone(),
                        },
                        adapter: connection.adapter.clone(),
                        adapter_kind: adapter.map(|adapter| AdapterKind::from_ident(&adapter.kind)),
                        contracts,
                    });
                }
                (Err(err), _) | (_, Err(err)) => diagnostics.push(err),
            }
        }

        composes.push(TypedCompose {
            name: compose.name.clone(),
            domain: compose.domain.clone(),
            instances: typed_instances,
            connections: typed_connections,
        });
    }

    if diagnostics.iter().any(|d| d.severity == Severity::Error) {
        Err(diagnostics)
    } else {
        Ok(TypedDesign {
            clock_domains,
            interfaces,
            modules,
            adapters,
            composes,
        })
    }
}

impl AdapterKind {
    fn from_ident(kind: &Ident) -> Self {
        match kind.0.as_str() {
            "cdc_fifo" | "async_fifo" => AdapterKind::CdcFifo,
            "width_adapter" | "width" | "extend" | "slice" => AdapterKind::WidthAdapter,
            "skid_buffer" | "skid" => AdapterKind::SkidBuffer,
            "pipeline" | "pipe" => AdapterKind::Pipeline,
            _ => AdapterKind::Custom(kind.clone()),
        }
    }
}

fn scalar_width_bits(ty: &ScalarType) -> Option<u32> {
    match ty {
        ScalarType::Bool => Some(1),
        ScalarType::UInt(width) => Some(*width),
        ScalarType::Named(_) => None,
    }
}

fn infer_reset_polarity(reset: &Ident) -> ResetPolarity {
    let name = reset.0.to_ascii_lowercase();
    if name.ends_with("_n") || name.ends_with("rstn") || name.ends_with("resetn") {
        ResetPolarity::ActiveLow
    } else if name == "rst" || name == "reset" || name.ends_with("_rst") || name.ends_with("_reset")
    {
        ResetPolarity::ActiveHigh
    } else {
        ResetPolarity::Unknown
    }
}

fn infer_protocol(interface: &InterfaceDef) -> InterfaceProtocol {
    let valid = interface
        .fields
        .iter()
        .find(|field| {
            field.role == Role::Producer && field.name.0 == "valid" && field.ty == ScalarType::Bool
        })
        .map(|field| field.name.clone());
    let ready = interface
        .fields
        .iter()
        .find(|field| {
            field.role == Role::Consumer && field.name.0 == "ready" && field.ty == ScalarType::Bool
        })
        .map(|field| field.name.clone());
    let payload_fields = interface
        .fields
        .iter()
        .filter(|field| field.role == Role::Producer && field.name.0 != "valid")
        .map(|field| field.name.clone())
        .collect::<Vec<_>>();

    InterfaceProtocol {
        kind: if valid.is_some() && ready.is_some() {
            ProtocolKind::ReadyValid
        } else {
            ProtocolKind::Custom
        },
        payload_fields,
        valid,
        ready,
    }
}

fn adapter_contracts(adapter: &AdapterDef) -> Vec<String> {
    adapter
        .attributes
        .iter()
        .filter(|(name, _)| name.0 == "contract")
        .map(|(_, value)| value.clone())
        .collect()
}

fn adapter_guarantees(
    adapter: &AdapterDef,
    kind: &AdapterKind,
    diags: &mut Vec<Diagnostic>,
) -> Vec<AdapterGuarantee> {
    let mut guarantees = Vec::new();
    for value in adapter_contracts(adapter) {
        match AdapterGuarantee::parse(&value) {
            Some(guarantee) if adapter_kind_allows_guarantee(kind, guarantee) => {
                guarantees.push(guarantee);
            }
            Some(guarantee) => diags.push(
                Diagnostic::error(
                    "ContractViolation",
                    format!(
                        "adapter `{}` of kind `{}` cannot claim contract `{}`",
                        adapter.name,
                        adapter_kind_label(kind).trim(),
                        guarantee.as_str()
                    ),
                )
                .with_label(LabelStyle::Primary, "adapter guarantee is not valid for this kind")
                .with_node("adapter", &adapter.name)
                .with_node("contract", guarantee.as_str())
                .with_repair(RepairAction::AddContract),
            ),
            None => diags.push(
                Diagnostic::error(
                    "ContractViolation",
                    format!(
                        "adapter `{}` declares unknown contract guarantee `{}`",
                        adapter.name, value
                    ),
                )
                .with_label(LabelStyle::Primary, "adapter guarantee is not in the v0 subset")
                .with_node("adapter", &adapter.name)
                .with_node("contract", value)
                .with_hint(
                    "use preserves_ready_valid, preserves_order, no_drop, no_duplicate, zero_extend_payload, sign_extend_payload, or cdc_fifo_assumed",
                )
                .with_repair(RepairAction::AddContract),
            ),
        }
    }
    guarantees
}

fn adapter_kind_allows_guarantee(kind: &AdapterKind, guarantee: AdapterGuarantee) -> bool {
    match kind {
        AdapterKind::CdcFifo => matches!(
            guarantee,
            AdapterGuarantee::PreservesReadyValid
                | AdapterGuarantee::PreservesOrder
                | AdapterGuarantee::NoDrop
                | AdapterGuarantee::NoDuplicate
                | AdapterGuarantee::CdcFifoAssumed
        ),
        AdapterKind::WidthAdapter => matches!(
            guarantee,
            AdapterGuarantee::PreservesReadyValid
                | AdapterGuarantee::ZeroExtendPayload
                | AdapterGuarantee::SignExtendPayload
        ),
        AdapterKind::SkidBuffer | AdapterKind::Pipeline => matches!(
            guarantee,
            AdapterGuarantee::PreservesReadyValid
                | AdapterGuarantee::PreservesOrder
                | AdapterGuarantee::NoDrop
                | AdapterGuarantee::NoDuplicate
        ),
        AdapterKind::Custom(_) => true,
    }
}

fn interface_contract_requirements(interface: &InterfaceDef) -> Vec<ContractRequirement> {
    let mut requirements = Vec::new();
    for contract in &interface.contracts {
        for requirement in contract_requirements(contract) {
            if !requirements.contains(&requirement) {
                requirements.push(requirement);
            }
        }
    }
    requirements
}

fn contract_requirements(contract: &ContractDef) -> Vec<ContractRequirement> {
    let mut requirements = Vec::new();
    let name = contract.name.0.as_str();
    if name == "stable_payload" {
        requirements.push(ContractRequirement::StablePayload);
    }
    if name == "fire" {
        requirements.push(ContractRequirement::FireEvent);
    }
    if name.contains("order") {
        requirements.push(ContractRequirement::Order);
    }
    if name.contains("no_drop") {
        requirements.push(ContractRequirement::NoDrop);
    }
    if name.contains("no_duplicate") {
        requirements.push(ContractRequirement::NoDuplicate);
    }

    if let Ok(expr) = parse_contract_expr(&contract.expr) {
        collect_contract_expr_requirements(&expr, &mut requirements);
    }
    requirements
}

fn collect_contract_expr_requirements(
    expr: &ContractExpr,
    requirements: &mut Vec<ContractRequirement>,
) {
    match expr {
        ContractExpr::Stable(_) => {
            push_requirement(requirements, ContractRequirement::StablePayload)
        }
        ContractExpr::Fire { .. } => push_requirement(requirements, ContractRequirement::FireEvent),
        ContractExpr::And(lhs, rhs)
        | ContractExpr::Or(lhs, rhs)
        | ContractExpr::Implication(lhs, rhs)
        | ContractExpr::Until(lhs, rhs) => {
            collect_contract_expr_requirements(lhs, requirements);
            collect_contract_expr_requirements(rhs, requirements);
        }
        ContractExpr::Ident(_) => {}
    }
}

fn push_requirement(requirements: &mut Vec<ContractRequirement>, requirement: ContractRequirement) {
    if !requirements.contains(&requirement) {
        requirements.push(requirement);
    }
}

fn requirement_covered(requirement: ContractRequirement, guarantees: &[AdapterGuarantee]) -> bool {
    match requirement {
        ContractRequirement::StablePayload | ContractRequirement::FireEvent => {
            guarantees.iter().any(|guarantee| {
                matches!(
                    guarantee,
                    AdapterGuarantee::PreservesReadyValid | AdapterGuarantee::PreservesOrder
                )
            })
        }
        ContractRequirement::Order => guarantees.contains(&AdapterGuarantee::PreservesOrder),
        ContractRequirement::NoDrop => {
            guarantees.contains(&AdapterGuarantee::NoDrop)
                || guarantees.contains(&AdapterGuarantee::PreservesOrder)
        }
        ContractRequirement::NoDuplicate => {
            guarantees.contains(&AdapterGuarantee::NoDuplicate)
                || guarantees.contains(&AdapterGuarantee::PreservesOrder)
        }
    }
}

fn resolve_endpoint<'a>(
    instances: &HashMap<Ident, &'a InstanceDef>,
    modules: &HashMap<Ident, &'a ModuleDef>,
    ep: &Endpoint,
) -> Result<(&'a ModuleDef, &'a PortDef), Diagnostic> {
    let inst = instances.get(&ep.instance).ok_or_else(|| {
        Diagnostic::error(
            "UnknownInstance",
            format!("unknown instance `{}` in endpoint `{}`", ep.instance, ep),
        )
        .with_label(LabelStyle::Primary, "endpoint instance is unresolved")
        .with_node("endpoint", ep)
        .with_node("instance", &ep.instance)
        .with_repair(RepairAction::FixEndpoint)
    })?;
    let module = modules.get(&inst.module).ok_or_else(|| {
        Diagnostic::error(
            "UnknownModule",
            format!(
                "instance `{}` references unknown module `{}`",
                inst.name, inst.module
            ),
        )
        .with_label(LabelStyle::Primary, "instance module is unresolved")
        .with_node("instance", &inst.name)
        .with_node("module", &inst.module)
        .with_repair(RepairAction::AddDeclaration)
    })?;
    let port = module
        .ports
        .iter()
        .find(|p| p.name == ep.port)
        .ok_or_else(|| {
            Diagnostic::error(
                "UnknownPort",
                format!(
                    "module `{}` has no port `{}` in endpoint `{}`",
                    module.name, ep.port, ep
                ),
            )
            .with_label(LabelStyle::Primary, "endpoint port is unresolved")
            .with_node("endpoint", ep)
            .with_node("module", &module.name)
            .with_node("port", format!("{}.{}", module.name, ep.port))
            .with_repair(RepairAction::FixEndpoint)
        })?;
    Ok((module, port))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_empty_design() {
        let d = Design::default();
        assert!(check_design(&d).is_empty());
    }

    #[test]
    fn typed_ir_infers_ready_valid_protocol_and_widths() {
        let typed = build_typed_ir(&stream_design()).unwrap();

        assert_eq!(
            typed.clock_domains[0].reset_polarity,
            ResetPolarity::ActiveHigh
        );
        assert_eq!(typed.interfaces[0].protocol.kind, ProtocolKind::ReadyValid);
        assert_eq!(typed.interfaces[0].protocol.payload_fields[0].0, "payload");
        assert_eq!(typed.interfaces[0].fields[0].width_bits, Some(32));
        assert_eq!(typed.interfaces[0].fields[1].width_bits, Some(1));
    }

    #[test]
    fn typed_ir_resolves_endpoint_and_contract_metadata() {
        let typed = build_typed_ir(&stream_design()).unwrap();
        let connection = &typed.composes[0].connections[0];

        assert_eq!(connection.from.endpoint.to_string(), "p.tx");
        assert_eq!(connection.from.module.0, "Producer");
        assert_eq!(connection.from.port_dir, PortDir::Out);
        assert_eq!(connection.from.interface.0, "StreamU32");
        assert_eq!(
            connection.contracts.source_interface[0].name.0,
            "stable_payload"
        );
        assert!(connection.adapter.is_none());
    }

    #[test]
    fn typed_ir_preserves_adapter_kind_and_contracts() {
        let typed = build_typed_ir(&cdc_design()).unwrap();

        assert_eq!(
            typed.clock_domains[0].reset_polarity,
            ResetPolarity::ActiveLow
        );
        assert_eq!(typed.adapters[0].kind, AdapterKind::CdcFifo);
        assert_eq!(typed.adapters[0].contracts, vec!["preserves_order"]);
        assert_eq!(
            typed.composes[0].connections[0].adapter_kind,
            Some(AdapterKind::CdcFifo)
        );
        assert_eq!(
            typed.composes[0].connections[0].contracts.adapter_contracts,
            vec!["preserves_order"]
        );
    }

    #[test]
    fn typed_ir_rejects_invalid_design() {
        let mut design = stream_design();
        design.interfaces.push(InterfaceDef {
            name: id("StreamU64"),
            domain: id("Sys"),
            fields: vec![FieldDef {
                name: id("payload"),
                ty: ScalarType::UInt(64),
                role: Role::Producer,
            }],
            contracts: vec![],
        });
        design.modules.push(ModuleDef {
            name: id("Sink64"),
            domain: id("Sys"),
            is_extern: true,
            ports: vec![PortDef {
                name: id("rx"),
                dir: PortDir::In,
                interface: id("StreamU64"),
            }],
        });
        design.composes[0].instances[1].module = id("Sink64");

        let diagnostics = build_typed_ir(&design).unwrap_err();
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "InterfaceMismatch")
        );
    }

    #[test]
    fn checker_rejects_duplicate_names() {
        let mut design = stream_design();
        design.clock_domains.push(ClockDomain {
            name: id("Sys"),
            clock: id("clk2"),
            reset: id("rst2"),
        });
        design.interfaces[0].fields.push(FieldDef {
            name: id("payload"),
            ty: ScalarType::UInt(32),
            role: Role::Producer,
        });
        design.modules[0].ports.push(PortDef {
            name: id("tx"),
            dir: PortDir::Out,
            interface: id("StreamU32"),
        });
        design.composes[0].instances.push(InstanceDef {
            name: id("p"),
            module: id("Producer"),
        });

        let diagnostics = check_design(&design);
        assert_has_code(&diagnostics, "DuplicateDeclaration");
        assert_has_code(&diagnostics, "DuplicateField");
        assert_has_code(&diagnostics, "DuplicatePort");
        assert_has_code(&diagnostics, "DuplicateInstance");
    }

    #[test]
    fn checker_rejects_unknown_compose_domain() {
        let mut design = stream_design();
        design.composes[0].domain = id("Missing");

        let diagnostics = check_design(&design);
        assert_has_code(&diagnostics, "UnknownClockDomain");
    }

    #[test]
    fn checker_accepts_explicit_width_adapter() {
        let diagnostics = check_design(&width_adapter_design());
        assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    }

    #[test]
    fn checker_rejects_unknown_adapter_kind() {
        let mut design = width_adapter_design();
        design.adapters[0].kind = id("magic_bridge");

        let diagnostics = check_design(&design);
        assert_has_code(&diagnostics, "UnknownAdapterKind");
    }

    #[test]
    fn checker_requires_adapter_contract_for_contract_sink() {
        let mut design = width_adapter_design();
        design.adapters[0].attributes.clear();

        let diagnostics = check_design(&design);
        assert_has_code(&diagnostics, "ContractViolation");
    }

    #[test]
    fn parses_ready_valid_contract_subset() {
        let expr = parse_contract_expr("valid -> stable(payload) until ready").unwrap();

        assert!(matches!(expr, ContractExpr::Implication(_, _)));
    }

    #[test]
    fn checker_rejects_unknown_adapter_contract_guarantee() {
        let mut design = width_adapter_design();
        design.adapters[0].attributes = vec![(id("contract"), "preserves_magic".to_string())];

        let diagnostics = check_design(&design);
        assert_has_code(&diagnostics, "ContractViolation");
    }

    #[test]
    fn checker_rejects_adapter_contract_for_wrong_kind() {
        let mut design = width_adapter_design();
        design.adapters[0].attributes = vec![(id("contract"), "preserves_order".to_string())];

        let diagnostics = check_design(&design);
        assert_has_code(&diagnostics, "ContractViolation");
    }

    #[test]
    fn checker_rejects_missing_cdc_contract_guarantee() {
        let mut design = cdc_design();
        design.adapters[0].attributes.clear();

        let diagnostics = check_design(&design);
        assert_has_code(&diagnostics, "ContractViolation");
    }

    #[test]
    fn checker_rejects_source_missing_sink_contract() {
        let mut design = width_adapter_design();
        design.interfaces[0].contracts.clear();

        let diagnostics = check_design(&design);
        assert_has_code(&diagnostics, "ContractViolation");
    }

    fn stream_design() -> Design {
        Design {
            clock_domains: vec![ClockDomain {
                name: id("Sys"),
                clock: id("clk"),
                reset: id("rst"),
            }],
            interfaces: vec![InterfaceDef {
                name: id("StreamU32"),
                domain: id("Sys"),
                fields: vec![
                    FieldDef {
                        name: id("payload"),
                        ty: ScalarType::UInt(32),
                        role: Role::Producer,
                    },
                    FieldDef {
                        name: id("valid"),
                        ty: ScalarType::Bool,
                        role: Role::Producer,
                    },
                    FieldDef {
                        name: id("ready"),
                        ty: ScalarType::Bool,
                        role: Role::Consumer,
                    },
                ],
                contracts: vec![ContractDef {
                    name: id("stable_payload"),
                    expr: "valid -> stable(payload) until ready".to_string(),
                }],
            }],
            modules: vec![
                ModuleDef {
                    name: id("Producer"),
                    domain: id("Sys"),
                    is_extern: true,
                    ports: vec![PortDef {
                        name: id("tx"),
                        dir: PortDir::Out,
                        interface: id("StreamU32"),
                    }],
                },
                ModuleDef {
                    name: id("Consumer"),
                    domain: id("Sys"),
                    is_extern: true,
                    ports: vec![PortDef {
                        name: id("rx"),
                        dir: PortDir::In,
                        interface: id("StreamU32"),
                    }],
                },
            ],
            adapters: vec![],
            composes: vec![ComposeDef {
                name: id("Top"),
                domain: id("Sys"),
                instances: vec![
                    InstanceDef {
                        name: id("p"),
                        module: id("Producer"),
                    },
                    InstanceDef {
                        name: id("c"),
                        module: id("Consumer"),
                    },
                ],
                connections: vec![ConnectDef {
                    from: Endpoint {
                        instance: id("p"),
                        port: id("tx"),
                    },
                    to: Endpoint {
                        instance: id("c"),
                        port: id("rx"),
                    },
                    adapter: None,
                }],
            }],
        }
    }

    fn cdc_design() -> Design {
        let mut design = stream_design();
        design.clock_domains = vec![
            ClockDomain {
                name: id("Aclk"),
                clock: id("aclk"),
                reset: id("arst_n"),
            },
            ClockDomain {
                name: id("Bclk"),
                clock: id("bclk"),
                reset: id("brst_n"),
            },
        ];
        design.interfaces[0].domain = id("Aclk");
        design.interfaces.push(InterfaceDef {
            name: id("StreamU32B"),
            domain: id("Bclk"),
            fields: design.interfaces[0].fields.clone(),
            contracts: design.interfaces[0].contracts.clone(),
        });
        design.modules[0].domain = id("Aclk");
        design.modules[1].domain = id("Bclk");
        design.modules[1].ports[0].interface = id("StreamU32B");
        design.adapters.push(AdapterDef {
            name: id("AsyncFifo32"),
            from_interface: id("StreamU32"),
            from_domain: id("Aclk"),
            to_interface: id("StreamU32B"),
            to_domain: id("Bclk"),
            kind: id("cdc_fifo"),
            attributes: vec![(id("contract"), "preserves_order".to_string())],
        });
        design.composes[0].domain = id("Aclk");
        design.composes[0].connections[0].adapter = Some(id("AsyncFifo32"));
        design
    }

    fn width_adapter_design() -> Design {
        let mut design = stream_design();
        design.interfaces.push(InterfaceDef {
            name: id("StreamU64"),
            domain: id("Sys"),
            fields: vec![
                FieldDef {
                    name: id("payload"),
                    ty: ScalarType::UInt(64),
                    role: Role::Producer,
                },
                FieldDef {
                    name: id("valid"),
                    ty: ScalarType::Bool,
                    role: Role::Producer,
                },
                FieldDef {
                    name: id("ready"),
                    ty: ScalarType::Bool,
                    role: Role::Consumer,
                },
            ],
            contracts: vec![ContractDef {
                name: id("stable_payload"),
                expr: "valid -> stable(payload) until ready".to_string(),
            }],
        });
        design.modules[1].ports[0].interface = id("StreamU64");
        design.adapters.push(AdapterDef {
            name: id("Widen32To64"),
            from_interface: id("StreamU32"),
            from_domain: id("Sys"),
            to_interface: id("StreamU64"),
            to_domain: id("Sys"),
            kind: id("width_adapter"),
            attributes: vec![
                (id("mode"), "zero_extend".to_string()),
                (id("contract"), "preserves_ready_valid".to_string()),
            ],
        });
        design.composes[0].connections[0].adapter = Some(id("Widen32To64"));
        design
    }

    fn assert_has_code(diagnostics: &[Diagnostic], code: &'static str) {
        assert!(
            diagnostics.iter().any(|diagnostic| diagnostic.code == code),
            "expected diagnostic code `{code}` in {diagnostics:#?}"
        );
    }

    fn id(value: &str) -> Ident {
        Ident::from(value)
    }
}
