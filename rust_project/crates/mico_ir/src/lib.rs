use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClockDomain {
    pub name: Ident,
    pub clock: Ident,
    pub reset: Ident,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Producer,
    Consumer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDef {
    pub name: Ident,
    pub ty: ScalarType,
    pub role: Role,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractDef {
    pub name: Ident,
    pub expr: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceDef {
    pub name: Ident,
    pub domain: Ident,
    pub fields: Vec<FieldDef>,
    pub contracts: Vec<ContractDef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortDir {
    In,
    Out,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortDef {
    pub name: Ident,
    pub dir: PortDir,
    pub interface: Ident,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleDef {
    pub name: Ident,
    pub domain: Ident,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstanceDef {
    pub name: Ident,
    pub module: Ident,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectDef {
    pub from: Endpoint,
    pub to: Endpoint,
    pub adapter: Option<Ident>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: &'static str,
    pub message: String,
    pub hints: Vec<String>,
}

impl Diagnostic {
    pub fn error(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code,
            message: message.into(),
            hints: Vec::new(),
        }
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hints.push(hint.into());
        self
    }
}

pub fn check_design(design: &Design) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
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
        if !clock_names.contains(&iface.domain) {
            diags.push(Diagnostic::error(
                "UnknownClockDomain",
                format!(
                    "interface `{}` references unknown clock domain `{}`",
                    iface.name, iface.domain
                ),
            ));
        }
    }

    for module in &design.modules {
        if !clock_names.contains(&module.domain) {
            diags.push(Diagnostic::error(
                "UnknownClockDomain",
                format!(
                    "module `{}` references unknown clock domain `{}`",
                    module.name, module.domain
                ),
            ));
        }
        for port in &module.ports {
            if !interfaces.contains_key(&port.interface) {
                diags.push(Diagnostic::error(
                    "UnknownInterface",
                    format!(
                        "module `{}` port `{}` references unknown interface `{}`",
                        module.name, port.name, port.interface
                    ),
                ));
            }
        }
    }

    for compose in &design.composes {
        let instances: HashMap<_, _> = compose
            .instances
            .iter()
            .map(|i| (i.name.clone(), i))
            .collect();
        for inst in &compose.instances {
            if !modules.contains_key(&inst.module) {
                diags.push(Diagnostic::error(
                    "UnknownModule",
                    format!(
                        "compose `{}` instantiates unknown module `{}` as `{}`",
                        compose.name, inst.module, inst.name
                    ),
                ));
            }
        }

        for conn in &compose.connections {
            let src = resolve_endpoint(&instances, &modules, &conn.from);
            let dst = resolve_endpoint(&instances, &modules, &conn.to);

            match (src, dst) {
                (Ok((src_mod, src_port)), Ok((dst_mod, dst_port))) => {
                    if src_port.dir != PortDir::Out {
                        diags.push(Diagnostic::error(
                            "DirectionMismatch",
                            format!("source endpoint `{}` is not an output port", conn.from),
                        ));
                    }
                    if dst_port.dir != PortDir::In {
                        diags.push(Diagnostic::error(
                            "DirectionMismatch",
                            format!("sink endpoint `{}` is not an input port", conn.to),
                        ));
                    }

                    let same_interface = src_port.interface == dst_port.interface;
                    let same_domain = src_mod.domain == dst_mod.domain;

                    if same_interface && same_domain {
                        continue;
                    }

                    if let Some(adapter_name) = &conn.adapter {
                        match adapters.get(adapter_name) {
                            Some(adapter) => {
                                if adapter.from_interface != src_port.interface
                                    || adapter.to_interface != dst_port.interface
                                    || adapter.from_domain != src_mod.domain
                                    || adapter.to_domain != dst_mod.domain
                                {
                                    diags.push(Diagnostic::error(
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
                                    ));
                                }
                            }
                            None => diags.push(Diagnostic::error(
                                "UnknownAdapter",
                                format!(
                                    "connection `{}` -> `{}` references unknown adapter `{}`",
                                    conn.from, conn.to, adapter_name
                                ),
                            )),
                        }
                    } else {
                        if !same_interface {
                            diags.push(Diagnostic::error(
                                "InterfaceMismatch",
                                format!(
                                    "direct connection `{}` -> `{}` uses incompatible interfaces `{}` and `{}`",
                                    conn.from, conn.to, src_port.interface, dst_port.interface
                                ),
                            ).with_hint("declare an explicit adapter or use matching interfaces"));
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
                                .with_hint("use an explicit CDC adapter such as AsyncFifo"),
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
                None => diagnostics.push(Diagnostic::error(
                    "InternalIrError",
                    format!(
                        "cannot lower instance `{}` because module `{}` is unresolved",
                        instance.name, instance.module
                    ),
                )),
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
    })?;
    let module = modules.get(&inst.module).ok_or_else(|| {
        Diagnostic::error(
            "UnknownModule",
            format!(
                "instance `{}` references unknown module `{}`",
                inst.name, inst.module
            ),
        )
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

    fn id(value: &str) -> Ident {
        Ident::from(value)
    }
}
