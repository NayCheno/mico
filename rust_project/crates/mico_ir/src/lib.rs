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
}
