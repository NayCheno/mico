use mico_ir::*;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet, HashMap};

pub fn emit_json_ir(typed: &TypedDesign) -> String {
    let mut out = serde_json::to_string_pretty(&typed_design_json(typed))
        .expect("MICO typed IR JSON serialization should be infallible");
    out.push('\n');
    out
}

fn typed_design_json(typed: &TypedDesign) -> Value {
    json!({
        "schema_version": "mico.ir.v0",
        "kind": "typed_design",
        "clock_domains": typed.clock_domains.iter().map(clock_domain_json).collect::<Vec<_>>(),
        "interfaces": typed.interfaces.iter().map(interface_json).collect::<Vec<_>>(),
        "modules": typed.modules.iter().map(module_json).collect::<Vec<_>>(),
        "adapters": typed.adapters.iter().map(adapter_json).collect::<Vec<_>>(),
        "composes": typed.composes.iter().map(compose_json).collect::<Vec<_>>(),
    })
}

fn clock_domain_json(domain: &TypedClockDomain) -> Value {
    json!({
        "name": ident_str(&domain.name),
        "clock": ident_str(&domain.clock),
        "reset": ident_str(&domain.reset),
        "reset_polarity": reset_polarity_str(domain.reset_polarity),
    })
}

fn interface_json(interface: &TypedInterface) -> Value {
    json!({
        "name": ident_str(&interface.name),
        "domain": ident_str(&interface.domain),
        "fields": interface.fields.iter().map(field_json).collect::<Vec<_>>(),
        "contracts": interface.contracts.iter().map(contract_json).collect::<Vec<_>>(),
        "protocol": protocol_json(&interface.protocol),
    })
}

fn field_json(field: &TypedField) -> Value {
    json!({
        "name": ident_str(&field.name),
        "role": role_str(&field.role),
        "type": scalar_type_json(&field.ty),
        "width_bits": field.width_bits,
    })
}

fn contract_json(contract: &ContractDef) -> Value {
    json!({
        "name": ident_str(&contract.name),
        "expr": &contract.expr,
    })
}

fn protocol_json(protocol: &InterfaceProtocol) -> Value {
    json!({
        "kind": protocol_kind_str(protocol.kind),
        "payload_fields": protocol.payload_fields.iter().map(ident_str).collect::<Vec<_>>(),
        "valid": protocol.valid.as_ref().map(ident_str),
        "ready": protocol.ready.as_ref().map(ident_str),
    })
}

fn module_json(module: &TypedModule) -> Value {
    json!({
        "name": ident_str(&module.name),
        "domain": ident_str(&module.domain),
        "extern": module.is_extern,
        "ports": module.ports.iter().map(port_json).collect::<Vec<_>>(),
    })
}

fn port_json(port: &TypedPort) -> Value {
    json!({
        "name": ident_str(&port.name),
        "direction": port_dir_str(port.dir),
        "interface": ident_str(&port.interface),
        "domain": ident_str(&port.domain),
    })
}

fn adapter_json(adapter: &TypedAdapter) -> Value {
    json!({
        "name": ident_str(&adapter.name),
        "from_interface": ident_str(&adapter.from_interface),
        "from_domain": ident_str(&adapter.from_domain),
        "to_interface": ident_str(&adapter.to_interface),
        "to_domain": ident_str(&adapter.to_domain),
        "kind": adapter_kind_json(&adapter.kind),
        "attributes": adapter.attributes.iter().map(attribute_json).collect::<Vec<_>>(),
        "contracts": &adapter.contracts,
    })
}

fn attribute_json((name, value): &(Ident, String)) -> Value {
    json!({
        "name": ident_str(name),
        "value": value,
    })
}

fn compose_json(compose: &TypedCompose) -> Value {
    json!({
        "name": ident_str(&compose.name),
        "domain": ident_str(&compose.domain),
        "instances": compose.instances.iter().map(instance_json).collect::<Vec<_>>(),
        "connections": compose.connections.iter().map(connection_json).collect::<Vec<_>>(),
    })
}

fn instance_json(instance: &TypedInstance) -> Value {
    json!({
        "name": ident_str(&instance.name),
        "module": ident_str(&instance.module),
        "domain": ident_str(&instance.domain),
    })
}

fn connection_json(connection: &TypedConnection) -> Value {
    json!({
        "from": endpoint_json(&connection.from),
        "to": endpoint_json(&connection.to),
        "adapter": connection.adapter.as_ref().map(ident_str),
        "adapter_kind": connection.adapter_kind.as_ref().map(adapter_kind_json),
        "contracts": {
            "source_interface": connection.contracts.source_interface.iter().map(contract_json).collect::<Vec<_>>(),
            "sink_interface": connection.contracts.sink_interface.iter().map(contract_json).collect::<Vec<_>>(),
            "adapter_contracts": &connection.contracts.adapter_contracts,
        },
    })
}

fn endpoint_json(endpoint: &TypedEndpoint) -> Value {
    json!({
        "endpoint": endpoint.endpoint.to_string(),
        "instance": ident_str(&endpoint.endpoint.instance),
        "port": ident_str(&endpoint.endpoint.port),
        "module": ident_str(&endpoint.module),
        "port_dir": port_dir_str(endpoint.port_dir),
        "interface": ident_str(&endpoint.interface),
        "domain": ident_str(&endpoint.domain),
    })
}

fn scalar_type_json(ty: &ScalarType) -> Value {
    match ty {
        ScalarType::Bool => json!({"kind": "bool", "width_bits": 1}),
        ScalarType::UInt(width) => json!({"kind": "uint", "width_bits": width}),
        ScalarType::Named(name) => json!({"kind": "named", "name": ident_str(name)}),
    }
}

fn adapter_kind_json(kind: &AdapterKind) -> Value {
    match kind {
        AdapterKind::CdcFifo => json!({"kind": "cdc_fifo"}),
        AdapterKind::WidthAdapter => json!({"kind": "width_adapter"}),
        AdapterKind::SkidBuffer => json!({"kind": "skid_buffer"}),
        AdapterKind::Pipeline => json!({"kind": "pipeline"}),
        AdapterKind::Custom(name) => json!({"kind": "custom", "name": ident_str(name)}),
    }
}

fn reset_polarity_str(polarity: ResetPolarity) -> &'static str {
    match polarity {
        ResetPolarity::ActiveHigh => "active_high",
        ResetPolarity::ActiveLow => "active_low",
        ResetPolarity::Unknown => "unknown",
    }
}

fn protocol_kind_str(kind: ProtocolKind) -> &'static str {
    match kind {
        ProtocolKind::ReadyValid => "ready_valid",
        ProtocolKind::Custom => "custom",
    }
}

fn role_str(role: &Role) -> &'static str {
    match role {
        Role::Producer => "producer",
        Role::Consumer => "consumer",
    }
}

fn port_dir_str(dir: PortDir) -> &'static str {
    match dir {
        PortDir::In => "in",
        PortDir::Out => "out",
    }
}

fn ident_str(ident: &Ident) -> &str {
    &ident.0
}

pub fn emit_traceability_report(typed: &TypedDesign) -> String {
    let mut out = serde_json::to_string_pretty(&traceability_json(typed))
        .expect("MICO traceability JSON serialization should be infallible");
    out.push('\n');
    out
}

fn traceability_json(typed: &TypedDesign) -> Value {
    let interface_map: HashMap<_, _> = typed
        .interfaces
        .iter()
        .map(|interface| (interface.name.clone(), interface))
        .collect();
    let adapter_map: HashMap<_, _> = typed
        .adapters
        .iter()
        .map(|adapter| (adapter.name.clone(), adapter))
        .collect();
    let domain_map: HashMap<_, _> = typed
        .clock_domains
        .iter()
        .map(|domain| (domain.name.clone(), domain))
        .collect();

    json!({
        "schema_version": "mico.traceability.v0",
        "kind": "traceability_report",
        "composes": typed.composes.iter().map(|compose| {
            trace_compose_json(compose, typed, &interface_map, &adapter_map, &domain_map)
        }).collect::<Vec<_>>(),
    })
}

fn trace_compose_json(
    compose: &TypedCompose,
    typed: &TypedDesign,
    interface_map: &HashMap<Ident, &TypedInterface>,
    adapter_map: &HashMap<Ident, &TypedAdapter>,
    domain_map: &HashMap<Ident, &TypedClockDomain>,
) -> Value {
    let mut wires = BTreeMap::<String, Option<u32>>::new();
    let mut bindings = HashMap::<PortFieldKey, String>::new();
    let mut adapter_instances = Vec::new();
    build_connection_bindings(
        compose,
        interface_map,
        adapter_map,
        &mut wires,
        &mut bindings,
        &mut adapter_instances,
    );
    let assertions = ready_valid_assertions(compose, interface_map, domain_map, &wires, &bindings);

    json!({
        "name": ident_str(&compose.name),
        "domain": ident_str(&compose.domain),
        "top_module": sv_ident(&compose.name),
        "sva_module": sva_module_name(&compose.name),
        "clock_reset_ports": top_clock_reset_ports(typed),
        "wires": wires.iter().map(|(name, width)| {
            json!({"name": name, "width_bits": width})
        }).collect::<Vec<_>>(),
        "connections": compose.connections.iter().enumerate().map(|(idx, connection)| {
            trace_connection_json(&compose.name, idx, connection, interface_map, &bindings)
        }).collect::<Vec<_>>(),
        "sva_properties": assertions.iter().map(assertion_json).collect::<Vec<_>>(),
    })
}

fn trace_connection_json(
    compose_name: &Ident,
    index: usize,
    connection: &TypedConnection,
    interface_map: &HashMap<Ident, &TypedInterface>,
    bindings: &HashMap<PortFieldKey, String>,
) -> Value {
    let source_bindings = trace_endpoint_fields(&connection.from, interface_map, bindings);
    let sink_bindings = trace_endpoint_fields(&connection.to, interface_map, bindings);
    let mut field_bindings = source_bindings.clone();
    field_bindings.extend(sink_bindings.clone());
    let adapter_instance = connection
        .adapter
        .as_ref()
        .map(|name| adapter_instance_name(name, index));

    json!({
        "index": index,
        "kind": if connection.adapter.is_some() { "adapt" } else { "connect" },
        "source_ref": connection_source_ref_json(compose_name, index),
        "from": endpoint_json(&connection.from),
        "to": endpoint_json(&connection.to),
        "adapter": connection.adapter.as_ref().map(ident_str),
        "adapter_instance": adapter_instance,
        "adapter_boundary": adapter_boundary_json(connection, index, &source_bindings, &sink_bindings),
        "field_bindings": field_bindings,
    })
}

fn connection_source_ref_json(compose_name: &Ident, index: usize) -> Value {
    json!({
        "kind": "compose_connection",
        "compose": ident_str(compose_name),
        "index": index,
        "span": Value::Null,
    })
}

fn adapter_boundary_json(
    connection: &TypedConnection,
    index: usize,
    source_bindings: &[Value],
    sink_bindings: &[Value],
) -> Value {
    match &connection.adapter {
        Some(adapter) => json!({
            "adapter": ident_str(adapter),
            "instance": adapter_instance_name(adapter, index),
            "from_endpoint": connection.from.endpoint.to_string(),
            "to_endpoint": connection.to.endpoint.to_string(),
            "input_signals": source_bindings,
            "output_signals": sink_bindings,
        }),
        None => Value::Null,
    }
}

fn trace_endpoint_fields(
    endpoint: &TypedEndpoint,
    interface_map: &HashMap<Ident, &TypedInterface>,
    bindings: &HashMap<PortFieldKey, String>,
) -> Vec<Value> {
    let Some(interface) = interface_map.get(&endpoint.interface) else {
        return Vec::new();
    };

    interface
        .fields
        .iter()
        .filter_map(|field| {
            let key = endpoint_field_key(endpoint, &field.name);
            bindings.get(&key).map(|signal| {
                json!({
                    "endpoint": endpoint.endpoint.to_string(),
                    "field": ident_str(&field.name),
                    "role": role_str(&field.role),
                    "leaf_port": leaf_field_port(&endpoint.endpoint.port, &field.name),
                    "signal": signal,
                    "width_bits": field.width_bits,
                })
            })
        })
        .collect()
}

fn leaf_field_port(port: &Ident, field: &Ident) -> String {
    sv_ident_str(&format!("{}_{}", port, field))
}

pub fn emit_systemverilog(design: &Design) -> String {
    let mut out = String::new();
    out.push_str("`default_nettype none\n\n");
    out.push_str("// Generated by MICO.\n");
    out.push_str("// Conservative SystemVerilog wrapper emitted from checked typed IR.\n\n");

    match build_typed_ir(design) {
        Ok(typed) => emit_typed_systemverilog(&typed, &mut out),
        Err(diagnostics) => {
            out.push_str("// Unable to emit SystemVerilog because typed IR construction failed.\n");
            for diagnostic in diagnostics {
                out.push_str(&format!(
                    "// {:?} [{}] {}\n",
                    diagnostic.severity, diagnostic.code, diagnostic.message
                ));
            }
        }
    }

    out.push_str("`default_nettype wire\n");
    out
}

fn emit_typed_systemverilog(typed: &TypedDesign, out: &mut String) {
    if typed.composes.is_empty() {
        out.push_str("// No compose blocks found.\n");
        return;
    }

    let module_map: HashMap<_, _> = typed
        .modules
        .iter()
        .map(|module| (module.name.clone(), module))
        .collect();
    let interface_map: HashMap<_, _> = typed
        .interfaces
        .iter()
        .map(|interface| (interface.name.clone(), interface))
        .collect();
    let adapter_map: HashMap<_, _> = typed
        .adapters
        .iter()
        .map(|adapter| (adapter.name.clone(), adapter))
        .collect();
    let domain_map: HashMap<_, _> = typed
        .clock_domains
        .iter()
        .map(|domain| (domain.name.clone(), domain))
        .collect();

    for compose in &typed.composes {
        emit_compose_module(
            compose,
            typed,
            &module_map,
            &interface_map,
            &adapter_map,
            &domain_map,
            out,
        );
    }
}

type PortFieldKey = (Ident, Ident, Ident);

#[derive(Debug, Clone)]
struct AdapterInstance {
    module_name: String,
    instance_name: String,
    kind: Option<AdapterKind>,
    from_domain: Ident,
    to_domain: Ident,
    input_signals: Vec<(String, String)>,
    output_signals: Vec<(String, String)>,
}

fn emit_compose_module(
    compose: &TypedCompose,
    typed: &TypedDesign,
    module_map: &HashMap<Ident, &TypedModule>,
    interface_map: &HashMap<Ident, &TypedInterface>,
    adapter_map: &HashMap<Ident, &TypedAdapter>,
    domain_map: &HashMap<Ident, &TypedClockDomain>,
    out: &mut String,
) {
    let mut wires = BTreeMap::<String, Option<u32>>::new();
    let mut bindings = HashMap::<PortFieldKey, String>::new();
    let mut adapter_instances = Vec::new();

    build_connection_bindings(
        compose,
        interface_map,
        adapter_map,
        &mut wires,
        &mut bindings,
        &mut adapter_instances,
    );

    emit_module_header(&compose.name, typed, out);
    emit_checked_connection_comments(compose, out);
    emit_wire_decls(&wires, out);
    emit_leaf_instances(
        compose,
        module_map,
        interface_map,
        domain_map,
        &bindings,
        out,
    );
    emit_adapter_instances(&adapter_instances, domain_map, out);
    out.push_str("endmodule\n\n");
}

fn build_connection_bindings(
    compose: &TypedCompose,
    interface_map: &HashMap<Ident, &TypedInterface>,
    adapter_map: &HashMap<Ident, &TypedAdapter>,
    wires: &mut BTreeMap<String, Option<u32>>,
    bindings: &mut HashMap<PortFieldKey, String>,
    adapter_instances: &mut Vec<AdapterInstance>,
) {
    for (idx, connection) in compose.connections.iter().enumerate() {
        let Some(source_interface) = interface_map.get(&connection.from.interface) else {
            continue;
        };
        let Some(sink_interface) = interface_map.get(&connection.to.interface) else {
            continue;
        };

        if let Some(adapter_name) = &connection.adapter {
            let Some(adapter) = adapter_map.get(adapter_name) else {
                continue;
            };
            let instance_name = adapter_instance_name(adapter_name, idx);
            let input_prefix = format!(
                "{}__{}_in",
                endpoint_prefix(&connection.from.endpoint),
                instance_name
            );
            let output_prefix = format!(
                "{}__{}",
                instance_name,
                endpoint_prefix(&connection.to.endpoint)
            );

            let input_signals = bind_endpoint_fields(
                bindings,
                wires,
                &connection.from,
                source_interface,
                &input_prefix,
            );
            let output_signals = bind_endpoint_fields(
                bindings,
                wires,
                &connection.to,
                sink_interface,
                &output_prefix,
            );

            adapter_instances.push(AdapterInstance {
                module_name: sv_ident(adapter_name),
                instance_name,
                kind: connection.adapter_kind.clone(),
                from_domain: adapter.from_domain.clone(),
                to_domain: adapter.to_domain.clone(),
                input_signals,
                output_signals,
            });
        } else {
            let prefix = format!(
                "{}__{}",
                endpoint_prefix(&connection.from.endpoint),
                endpoint_prefix(&connection.to.endpoint)
            );
            bind_direct_fields(
                bindings,
                wires,
                &connection.from,
                &connection.to,
                source_interface,
                &prefix,
            );
        }
    }
}

fn bind_direct_fields(
    bindings: &mut HashMap<PortFieldKey, String>,
    wires: &mut BTreeMap<String, Option<u32>>,
    from: &TypedEndpoint,
    to: &TypedEndpoint,
    interface: &TypedInterface,
    prefix: &str,
) {
    for field in &interface.fields {
        let signal = format!("{}_{}", prefix, sv_ident(&field.name));
        wires.entry(signal.clone()).or_insert(field.width_bits);
        bindings.insert(endpoint_field_key(from, &field.name), signal.clone());
        bindings.insert(endpoint_field_key(to, &field.name), signal);
    }
}

fn bind_endpoint_fields(
    bindings: &mut HashMap<PortFieldKey, String>,
    wires: &mut BTreeMap<String, Option<u32>>,
    endpoint: &TypedEndpoint,
    interface: &TypedInterface,
    prefix: &str,
) -> Vec<(String, String)> {
    let mut signals = Vec::new();
    for field in &interface.fields {
        let field_name = sv_ident(&field.name);
        let signal = format!("{}_{}", prefix, field_name);
        wires.entry(signal.clone()).or_insert(field.width_bits);
        bindings.insert(endpoint_field_key(endpoint, &field.name), signal.clone());
        signals.push((field_name, signal));
    }
    signals
}

fn emit_module_header(name: &Ident, typed: &TypedDesign, out: &mut String) {
    let ports = top_clock_reset_ports(typed);
    if ports.is_empty() {
        out.push_str(&format!("module {};\n\n", sv_ident(name)));
        return;
    }

    out.push_str(&format!("module {}(\n", sv_ident(name)));
    for (idx, port) in ports.iter().enumerate() {
        out.push_str(&format!(
            "  input logic {}{}\n",
            port,
            if idx + 1 == ports.len() { "" } else { "," }
        ));
    }
    out.push_str(");\n\n");
}

fn top_clock_reset_ports(typed: &TypedDesign) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut ports = Vec::new();
    for domain in &typed.clock_domains {
        for signal in [&domain.clock, &domain.reset] {
            let signal = sv_ident(signal);
            if seen.insert(signal.clone()) {
                ports.push(signal);
            }
        }
    }
    ports
}

fn emit_checked_connection_comments(compose: &TypedCompose, out: &mut String) {
    out.push_str("  // Checked interface connections\n");
    for connection in &compose.connections {
        match &connection.adapter {
            Some(adapter) => out.push_str(&format!(
                "  // adapt {} -> {} -> {}\n",
                connection.from.endpoint, adapter, connection.to.endpoint
            )),
            None => out.push_str(&format!(
                "  // connect {} -> {}\n",
                connection.from.endpoint, connection.to.endpoint
            )),
        }
    }
    out.push('\n');
}

fn emit_wire_decls(wires: &BTreeMap<String, Option<u32>>, out: &mut String) {
    if wires.is_empty() {
        return;
    }

    out.push_str("  // Interface field wires\n");
    for (name, width) in wires {
        match width {
            Some(width) if *width > 1 => {
                out.push_str(&format!("  logic [{}:0] {};\n", width - 1, name));
            }
            _ => out.push_str(&format!("  logic {};\n", name)),
        }
    }
    out.push('\n');
}

fn emit_leaf_instances(
    compose: &TypedCompose,
    module_map: &HashMap<Ident, &TypedModule>,
    interface_map: &HashMap<Ident, &TypedInterface>,
    domain_map: &HashMap<Ident, &TypedClockDomain>,
    bindings: &HashMap<PortFieldKey, String>,
    out: &mut String,
) {
    out.push_str("  // Leaf instances\n");
    for instance in &compose.instances {
        let Some(module) = module_map.get(&instance.module) else {
            continue;
        };
        let Some(domain) = domain_map.get(&module.domain) else {
            continue;
        };
        let mut port_maps = vec![
            ("clk".to_string(), sv_ident(&domain.clock)),
            ("rst".to_string(), sv_ident(&domain.reset)),
        ];

        for port in &module.ports {
            let Some(interface) = interface_map.get(&port.interface) else {
                continue;
            };
            for field in &interface.fields {
                let key = (instance.name.clone(), port.name.clone(), field.name.clone());
                if let Some(signal) = bindings.get(&key) {
                    port_maps.push((
                        sv_ident_str(&format!("{}_{}", port.name, field.name)),
                        signal.clone(),
                    ));
                }
            }
        }

        emit_instance(
            &sv_ident(&instance.module),
            &sv_ident(&instance.name),
            &port_maps,
            out,
        );
    }
}

fn emit_adapter_instances(
    instances: &[AdapterInstance],
    domain_map: &HashMap<Ident, &TypedClockDomain>,
    out: &mut String,
) {
    if instances.is_empty() {
        return;
    }

    out.push_str("  // Explicit adapters\n");
    for instance in instances {
        let mut port_maps = Vec::new();
        match instance.kind {
            Some(AdapterKind::CdcFifo) => {
                if let (Some(from), Some(to)) = (
                    domain_map.get(&instance.from_domain),
                    domain_map.get(&instance.to_domain),
                ) {
                    port_maps.push(("src_clk".to_string(), sv_ident(&from.clock)));
                    port_maps.push(("src_rst".to_string(), sv_ident(&from.reset)));
                    port_maps.push(("dst_clk".to_string(), sv_ident(&to.clock)));
                    port_maps.push(("dst_rst".to_string(), sv_ident(&to.reset)));
                }
            }
            _ => {
                if let Some(domain) = domain_map.get(&instance.from_domain) {
                    port_maps.push(("clk".to_string(), sv_ident(&domain.clock)));
                    port_maps.push(("rst".to_string(), sv_ident(&domain.reset)));
                }
            }
        }

        for (field, signal) in &instance.input_signals {
            port_maps.push((format!("in_{field}"), signal.clone()));
        }
        for (field, signal) in &instance.output_signals {
            port_maps.push((format!("out_{field}"), signal.clone()));
        }

        emit_instance(
            &instance.module_name,
            &instance.instance_name,
            &port_maps,
            out,
        );
    }
}

fn emit_instance(module: &str, instance: &str, port_maps: &[(String, String)], out: &mut String) {
    out.push_str(&format!("  {module} {instance} (\n"));
    for (idx, (port, signal)) in port_maps.iter().enumerate() {
        out.push_str(&format!(
            "    .{}({}){}\n",
            port,
            signal,
            if idx + 1 == port_maps.len() { "" } else { "," }
        ));
    }
    out.push_str("  );\n\n");
}

fn endpoint_field_key(endpoint: &TypedEndpoint, field: &Ident) -> PortFieldKey {
    (
        endpoint.endpoint.instance.clone(),
        endpoint.endpoint.port.clone(),
        field.clone(),
    )
}

fn endpoint_prefix(endpoint: &Endpoint) -> String {
    sv_ident_str(&format!("{}_{}", endpoint.instance, endpoint.port))
}

fn adapter_instance_name(adapter_name: &Ident, index: usize) -> String {
    format!("{}_{}", sv_ident_lower(adapter_name), index)
}

fn sv_ident(ident: &Ident) -> String {
    sv_ident_str(&ident.0)
}

fn sv_ident_lower(ident: &Ident) -> String {
    sv_ident_str(&ident.0.to_ascii_lowercase())
}

fn sv_ident_str(value: &str) -> String {
    let mut out = String::new();
    for ch in value.chars() {
        if ch == '_' || ch.is_ascii_alphanumeric() {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.is_empty() || out.as_bytes()[0].is_ascii_digit() {
        out.insert(0, '_');
    }
    out
}

#[derive(Debug, Clone)]
struct ReadyValidAssertion {
    name: String,
    endpoint: String,
    interface: Ident,
    domain: Ident,
    clock: String,
    reset: String,
    reset_inactive: String,
    contract_id: String,
    payload: String,
    payload_width: Option<u32>,
    valid: String,
    ready: String,
}

pub fn emit_sva_skeleton(design: &Design) -> String {
    let mut out = String::new();
    out.push_str("`default_nettype none\n\n");
    out.push_str("// MICO-generated SVA skeleton.\n");
    out.push_str(
        "// Bind ports to generated wrapper wires before treating assertions as proof.\n\n",
    );

    match build_typed_ir(design) {
        Ok(typed) => emit_typed_sva_skeleton(&typed, &mut out),
        Err(diagnostics) => {
            out.push_str("// Unable to emit SVA because typed IR construction failed.\n");
            for diagnostic in diagnostics {
                out.push_str(&format!(
                    "// {} [{}] {}\n",
                    diagnostic.severity.as_str(),
                    diagnostic.code,
                    diagnostic.message
                ));
            }
        }
    }

    out.push_str("`default_nettype wire\n");
    out
}

fn emit_typed_sva_skeleton(typed: &TypedDesign, out: &mut String) {
    let interface_map: HashMap<_, _> = typed
        .interfaces
        .iter()
        .map(|interface| (interface.name.clone(), interface))
        .collect();
    let adapter_map: HashMap<_, _> = typed
        .adapters
        .iter()
        .map(|adapter| (adapter.name.clone(), adapter))
        .collect();
    let domain_map: HashMap<_, _> = typed
        .clock_domains
        .iter()
        .map(|domain| (domain.name.clone(), domain))
        .collect();

    for compose in &typed.composes {
        let mut wires = BTreeMap::<String, Option<u32>>::new();
        let mut bindings = HashMap::<PortFieldKey, String>::new();
        let mut adapter_instances = Vec::new();
        build_connection_bindings(
            compose,
            &interface_map,
            &adapter_map,
            &mut wires,
            &mut bindings,
            &mut adapter_instances,
        );
        let assertions =
            ready_valid_assertions(compose, &interface_map, &domain_map, &wires, &bindings);

        emit_sva_module_header(&compose.name, typed, &wires, out);
        emit_sva_connection_comments(compose, out);
        if assertions.is_empty() {
            out.push_str("  // No ready/valid stable-payload assertions were inferred.\n");
        } else {
            for assertion in &assertions {
                emit_ready_valid_assertion(assertion, out);
            }
        }
        out.push_str("endmodule\n\n");
    }
}

fn emit_sva_module_header(
    name: &Ident,
    typed: &TypedDesign,
    wires: &BTreeMap<String, Option<u32>>,
    out: &mut String,
) {
    let module_name = sva_module_name(name);
    let mut seen = BTreeSet::new();
    let mut ports = top_clock_reset_ports(typed)
        .into_iter()
        .filter_map(|port| {
            if seen.insert(port.clone()) {
                Some((port, None))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    for (wire, width) in wires {
        if seen.insert(wire.clone()) {
            ports.push((wire.clone(), *width));
        }
    }

    if ports.is_empty() {
        out.push_str(&format!("module {module_name};\n\n"));
        return;
    }

    out.push_str(&format!("module {module_name}(\n"));
    for (idx, (port, width)) in ports.iter().enumerate() {
        out.push_str("  input logic ");
        if let Some(width) = width.filter(|width| *width > 1) {
            out.push_str(&format!("[{}:0] ", width - 1));
        }
        out.push_str(port);
        out.push_str(if idx + 1 == ports.len() { "\n" } else { ",\n" });
    }
    out.push_str(");\n\n");
}

fn emit_sva_connection_comments(compose: &TypedCompose, out: &mut String) {
    out.push_str("  // Traceability: checked MICO connections\n");
    for (idx, connection) in compose.connections.iter().enumerate() {
        match &connection.adapter {
            Some(adapter) => out.push_str(&format!(
                "  // [{}] adapt {} -> {} -> {}\n",
                idx, connection.from.endpoint, adapter, connection.to.endpoint
            )),
            None => out.push_str(&format!(
                "  // [{}] connect {} -> {}\n",
                idx, connection.from.endpoint, connection.to.endpoint
            )),
        }
    }
    out.push('\n');
}

fn emit_ready_valid_assertion(assertion: &ReadyValidAssertion, out: &mut String) {
    out.push_str(&format!(
        "  // {}: stable payload while valid is held without ready on {} ({}, {})\n",
        assertion.name, assertion.endpoint, assertion.interface, assertion.contract_id
    ));
    out.push_str("  logic ");
    if let Some(width) = assertion.payload_width.filter(|width| *width > 1) {
        out.push_str(&format!("[{}:0] ", width - 1));
    }
    out.push_str(&format!("{}_payload_q;\n", assertion.name));
    out.push_str(&format!(
        "  always_ff @(posedge {}) begin : {}\n",
        assertion.clock, assertion.name
    ));
    out.push_str(&format!(
        "    {}_payload_q <= {};\n",
        assertion.name, assertion.payload
    ));
    out.push_str(&format!(
        "    if ({} && {} && !{}) begin\n",
        assertion.reset_inactive, assertion.valid, assertion.ready
    ));
    out.push_str(&format!(
        "      assert ({} == {}_payload_q);\n",
        assertion.payload, assertion.name
    ));
    out.push_str("    end\n");
    out.push_str("  end\n\n");
}

fn ready_valid_assertions(
    compose: &TypedCompose,
    interface_map: &HashMap<Ident, &TypedInterface>,
    domain_map: &HashMap<Ident, &TypedClockDomain>,
    wires: &BTreeMap<String, Option<u32>>,
    bindings: &HashMap<PortFieldKey, String>,
) -> Vec<ReadyValidAssertion> {
    let mut assertions = Vec::new();
    let mut seen = BTreeSet::new();
    for (idx, connection) in compose.connections.iter().enumerate() {
        collect_ready_valid_assertion(
            idx,
            "src",
            &connection.from,
            interface_map,
            domain_map,
            wires,
            bindings,
            &mut seen,
            &mut assertions,
        );
        collect_ready_valid_assertion(
            idx,
            "dst",
            &connection.to,
            interface_map,
            domain_map,
            wires,
            bindings,
            &mut seen,
            &mut assertions,
        );
    }
    assertions
}

fn collect_ready_valid_assertion(
    connection_index: usize,
    side: &str,
    endpoint: &TypedEndpoint,
    interface_map: &HashMap<Ident, &TypedInterface>,
    domain_map: &HashMap<Ident, &TypedClockDomain>,
    wires: &BTreeMap<String, Option<u32>>,
    bindings: &HashMap<PortFieldKey, String>,
    seen: &mut BTreeSet<(String, String, String, String, String)>,
    assertions: &mut Vec<ReadyValidAssertion>,
) {
    let Some(interface) = interface_map.get(&endpoint.interface) else {
        return;
    };
    if interface.protocol.kind != ProtocolKind::ReadyValid {
        return;
    }
    if !interface_requires_stable_payload(interface) {
        return;
    }
    let Some(contract_id) = stable_payload_contract_id(interface) else {
        return;
    };
    let Some(payload_field) = interface.protocol.payload_fields.first() else {
        return;
    };
    let (Some(valid_field), Some(ready_field)) =
        (&interface.protocol.valid, &interface.protocol.ready)
    else {
        return;
    };

    let Some(payload) = bindings.get(&endpoint_field_key(endpoint, payload_field)) else {
        return;
    };
    let Some(valid) = bindings.get(&endpoint_field_key(endpoint, valid_field)) else {
        return;
    };
    let Some(ready) = bindings.get(&endpoint_field_key(endpoint, ready_field)) else {
        return;
    };
    let Some(domain) = domain_map.get(&endpoint.domain) else {
        return;
    };

    let clock = sv_ident(&domain.clock);
    let reset = sv_ident(&domain.reset);
    let reset_inactive = reset_inactive_expr(&reset, domain.reset_polarity);
    let key = (
        clock.clone(),
        reset_inactive.clone(),
        payload.clone(),
        valid.clone(),
        ready.clone(),
    );
    if !seen.insert(key) {
        return;
    }

    assertions.push(ReadyValidAssertion {
        name: sv_ident_str(&format!(
            "mico_conn_{}_{}_{}_stable_payload",
            connection_index,
            side,
            endpoint_prefix(&endpoint.endpoint)
        )),
        endpoint: endpoint.endpoint.to_string(),
        interface: endpoint.interface.clone(),
        domain: endpoint.domain.clone(),
        clock,
        reset,
        reset_inactive,
        contract_id,
        payload: payload.clone(),
        payload_width: wires.get(payload).copied().flatten(),
        valid: valid.clone(),
        ready: ready.clone(),
    });
}

fn interface_requires_stable_payload(interface: &TypedInterface) -> bool {
    interface
        .contracts
        .iter()
        .any(|contract| contract_requires_stable_payload(contract))
}

fn stable_payload_contract_id(interface: &TypedInterface) -> Option<String> {
    interface
        .contracts
        .iter()
        .find(|contract| contract_requires_stable_payload(contract))
        .map(|contract| format!("{}.{}", interface.name, contract.name))
}

fn contract_requires_stable_payload(contract: &ContractDef) -> bool {
    if contract.name.0 == "stable_payload" {
        return true;
    }
    parse_contract_expr(&contract.expr)
        .map(|expr| contract_expr_has_stable(&expr))
        .unwrap_or(false)
}

fn contract_expr_has_stable(expr: &ContractExpr) -> bool {
    match expr {
        ContractExpr::Stable(_) => true,
        ContractExpr::And(lhs, rhs)
        | ContractExpr::Or(lhs, rhs)
        | ContractExpr::Implication(lhs, rhs)
        | ContractExpr::Until(lhs, rhs) => {
            contract_expr_has_stable(lhs) || contract_expr_has_stable(rhs)
        }
        ContractExpr::Ident(_) | ContractExpr::Fire { .. } => false,
    }
}

fn assertion_json(assertion: &ReadyValidAssertion) -> Value {
    json!({
        "name": &assertion.name,
        "kind": "ready_valid_stable_payload",
        "contract_id": &assertion.contract_id,
        "endpoint": &assertion.endpoint,
        "interface": ident_str(&assertion.interface),
        "domain": ident_str(&assertion.domain),
        "clock": &assertion.clock,
        "reset": &assertion.reset,
        "payload": &assertion.payload,
        "valid": &assertion.valid,
        "ready": &assertion.ready,
    })
}

fn reset_inactive_expr(reset: &str, polarity: ResetPolarity) -> String {
    match polarity {
        ResetPolarity::ActiveHigh | ResetPolarity::Unknown => format!("!{reset}"),
        ResetPolarity::ActiveLow => reset.to_string(),
    }
}

fn sva_module_name(name: &Ident) -> String {
    format!("mico_sva_{}", sv_ident(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct GoldenCase {
        name: &'static str,
        source: &'static str,
        sv: &'static str,
        sva: &'static str,
        trace: &'static str,
    }

    const GOLDEN_CASES: &[GoldenCase] = &[
        GoldenCase {
            name: "stream_fifo",
            source: include_str!("../../../examples/stream_fifo.mico"),
            sv: include_str!("../tests/fixtures/golden/stream_fifo.sv"),
            sva: include_str!("../tests/fixtures/golden/stream_fifo.sva"),
            trace: include_str!("../tests/fixtures/golden/stream_fifo.trace.json"),
        },
        GoldenCase {
            name: "cdc_fifo",
            source: include_str!("../../../examples/cdc_fifo.mico"),
            sv: include_str!("../tests/fixtures/golden/cdc_fifo.sv"),
            sva: include_str!("../tests/fixtures/golden/cdc_fifo.sva"),
            trace: include_str!("../tests/fixtures/golden/cdc_fifo.trace.json"),
        },
        GoldenCase {
            name: "width_adapter",
            source: include_str!("../../../examples/width_adapter.mico"),
            sv: include_str!("../tests/fixtures/golden/width_adapter.sv"),
            sva: include_str!("../tests/fixtures/golden/width_adapter.sva"),
            trace: include_str!("../tests/fixtures/golden/width_adapter.trace.json"),
        },
        GoldenCase {
            name: "direct_stream",
            source: include_str!("../../../../benchmarks/tasks/T004_direct_stream/expected.mico"),
            sv: include_str!("../tests/fixtures/golden/direct_stream.sv"),
            sva: include_str!("../tests/fixtures/golden/direct_stream.sva"),
            trace: include_str!("../tests/fixtures/golden/direct_stream.trace.json"),
        },
        GoldenCase {
            name: "streaming_accelerator_case",
            source: include_str!(
                "../../../../benchmarks/tasks/T058_streaming_accelerator_case/expected.mico"
            ),
            sv: include_str!("../tests/fixtures/golden/streaming_accelerator_case.sv"),
            sva: include_str!("../tests/fixtures/golden/streaming_accelerator_case.sva"),
            trace: include_str!("../tests/fixtures/golden/streaming_accelerator_case.trace.json"),
        },
        GoldenCase {
            name: "width_protocol_bridge_case",
            source: include_str!(
                "../../../../benchmarks/tasks/T059_width_protocol_bridge_case/expected.mico"
            ),
            sv: include_str!("../tests/fixtures/golden/width_protocol_bridge_case.sv"),
            sva: include_str!("../tests/fixtures/golden/width_protocol_bridge_case.sva"),
            trace: include_str!("../tests/fixtures/golden/width_protocol_bridge_case.trace.json"),
        },
        GoldenCase {
            name: "register_status_case",
            source: include_str!(
                "../../../../benchmarks/tasks/T060_register_status_case/expected.mico"
            ),
            sv: include_str!("../tests/fixtures/golden/register_status_case.sv"),
            sva: include_str!("../tests/fixtures/golden/register_status_case.sva"),
            trace: include_str!("../tests/fixtures/golden/register_status_case.trace.json"),
        },
        GoldenCase {
            name: "protocol_bridge_case",
            source: include_str!(
                "../../../../benchmarks/tasks/T061_protocol_bridge_case/expected.mico"
            ),
            sv: include_str!("../tests/fixtures/golden/protocol_bridge_case.sv"),
            sva: include_str!("../tests/fixtures/golden/protocol_bridge_case.sva"),
            trace: include_str!("../tests/fixtures/golden/protocol_bridge_case.trace.json"),
        },
        GoldenCase {
            name: "multi_ip_telemetry_case",
            source: include_str!(
                "../../../../benchmarks/tasks/T062_multi_ip_telemetry_case/expected.mico"
            ),
            sv: include_str!("../tests/fixtures/golden/multi_ip_telemetry_case.sv"),
            sva: include_str!("../tests/fixtures/golden/multi_ip_telemetry_case.sva"),
            trace: include_str!("../tests/fixtures/golden/multi_ip_telemetry_case.trace.json"),
        },
        GoldenCase {
            name: "axi_apb_wrapper_case",
            source: include_str!(
                "../../../../benchmarks/tasks/T063_axi_apb_wrapper_case/expected.mico"
            ),
            sv: include_str!("../tests/fixtures/golden/axi_apb_wrapper_case.sv"),
            sva: include_str!("../tests/fixtures/golden/axi_apb_wrapper_case.sva"),
            trace: include_str!("../tests/fixtures/golden/axi_apb_wrapper_case.trace.json"),
        },
        GoldenCase {
            name: "video_filter_pipeline_case",
            source: include_str!(
                "../../../../benchmarks/tasks/T064_video_filter_pipeline_case/expected.mico"
            ),
            sv: include_str!("../tests/fixtures/golden/video_filter_pipeline_case.sv"),
            sva: include_str!("../tests/fixtures/golden/video_filter_pipeline_case.sva"),
            trace: include_str!("../tests/fixtures/golden/video_filter_pipeline_case.trace.json"),
        },
        GoldenCase {
            name: "cdc_event_status_case",
            source: include_str!(
                "../../../../benchmarks/tasks/T065_cdc_event_status_case/expected.mico"
            ),
            sv: include_str!("../tests/fixtures/golden/cdc_event_status_case.sv"),
            sva: include_str!("../tests/fixtures/golden/cdc_event_status_case.sva"),
            trace: include_str!("../tests/fixtures/golden/cdc_event_status_case.trace.json"),
        },
    ];

    #[test]
    fn golden_codegen_outputs_match_fixtures() {
        for case in GOLDEN_CASES {
            let design = mico_frontend::parse_mico(case.source)
                .unwrap_or_else(|errors| panic!("{} parse failed: {errors:#?}", case.name));
            assert_eq!(
                emit_systemverilog(&design),
                case.sv,
                "{} SystemVerilog golden drifted",
                case.name
            );
            assert_eq!(
                emit_sva_skeleton(&design),
                case.sva,
                "{} SVA golden drifted",
                case.name
            );
            let typed = build_typed_ir(&design)
                .unwrap_or_else(|errors| panic!("{} typed IR failed: {errors:#?}", case.name));
            assert_eq!(
                emit_traceability_report(&typed),
                case.trace,
                "{} traceability golden drifted",
                case.name
            );
        }
    }

    #[test]
    fn emits_direct_ready_valid_wires_and_ports() {
        let sv = emit_systemverilog(&stream_design());

        assert!(sv.starts_with("`default_nettype none"));
        assert!(sv.contains("module Top("));
        assert!(sv.contains("input logic clk"));
        assert!(sv.contains("logic [31:0] p_tx__c_rx_payload;"));
        assert!(sv.contains(".tx_payload(p_tx__c_rx_payload)"));
        assert!(sv.contains(".rx_ready(p_tx__c_rx_ready)"));
        assert!(sv.ends_with("`default_nettype wire\n"));
    }

    #[test]
    fn emits_width_adapter_instance() {
        let sv = emit_systemverilog(&width_adapter_design());

        assert!(sv.contains("Widen32To64 widen32to64_0"));
        assert!(sv.contains(".in_payload(s_tx__widen32to64_0_in_payload)"));
        assert!(sv.contains(".out_payload(widen32to64_0__t_rx_payload)"));
        assert!(sv.contains("logic [63:0] widen32to64_0__t_rx_payload;"));
    }

    #[test]
    fn emits_cdc_adapter_clocks() {
        let sv = emit_systemverilog(&cdc_design());

        assert!(sv.contains("input logic aclk"));
        assert!(sv.contains("input logic arst_n"));
        assert!(sv.contains("input logic bclk"));
        assert!(sv.contains("AsyncFifo32 asyncfifo32_0"));
        assert!(sv.contains(".src_clk(aclk)"));
        assert!(sv.contains(".dst_clk(bclk)"));
    }

    #[test]
    fn emits_versioned_typed_json_ir() {
        let typed = build_typed_ir(&stream_design()).unwrap();
        let json: serde_json::Value = serde_json::from_str(&emit_json_ir(&typed)).unwrap();

        assert_eq!(json["schema_version"], "mico.ir.v0");
        assert_eq!(json["kind"], "typed_design");
        assert_eq!(json["interfaces"][0]["protocol"]["kind"], "ready_valid");
        assert_eq!(
            json["composes"][0]["connections"][0]["from"]["endpoint"],
            "p.tx"
        );
        assert_eq!(
            json["composes"][0]["connections"][0]["to"]["endpoint"],
            "c.rx"
        );
    }

    #[test]
    fn emits_traceability_report_for_generated_wires() {
        let typed = build_typed_ir(&stream_design()).unwrap();
        let json: serde_json::Value =
            serde_json::from_str(&emit_traceability_report(&typed)).unwrap();

        assert_eq!(json["schema_version"], "mico.traceability.v0");
        assert_eq!(json["composes"][0]["top_module"], "Top");
        assert_eq!(
            json["composes"][0]["connections"][0]["field_bindings"][0]["signal"],
            "p_tx__c_rx_payload"
        );
        assert_eq!(
            json["composes"][0]["sva_properties"][0]["kind"],
            "ready_valid_stable_payload"
        );
    }

    #[test]
    fn traceability_covers_all_emitted_wires_adapter_boundaries_and_contracts() {
        for case in GOLDEN_CASES {
            let design = mico_frontend::parse_mico(case.source)
                .unwrap_or_else(|errors| panic!("{} parse failed: {errors:#?}", case.name));
            let typed = build_typed_ir(&design)
                .unwrap_or_else(|errors| panic!("{} typed IR failed: {errors:#?}", case.name));
            let trace: serde_json::Value =
                serde_json::from_str(&emit_traceability_report(&typed)).unwrap();
            let sv = emit_systemverilog(&design);

            let wires = trace["composes"][0]["wires"]
                .as_array()
                .unwrap_or_else(|| panic!("{} trace wires should be an array", case.name));
            for wire in wires {
                let name = wire["name"]
                    .as_str()
                    .unwrap_or_else(|| panic!("{} trace wire should have a name", case.name));
                assert!(
                    sv.contains(&format!(" {name};"))
                        || sv.contains(&format!(" {name},"))
                        || sv.contains(&format!("({name})")),
                    "{} emitted wire `{}` is missing from SystemVerilog",
                    case.name,
                    name
                );
            }

            let connections = trace["composes"][0]["connections"]
                .as_array()
                .unwrap_or_else(|| panic!("{} trace connections should be an array", case.name));
            for connection in connections {
                if connection["adapter"].is_string() {
                    assert!(
                        connection["adapter_boundary"].is_object(),
                        "{} adapted connection lacks adapter boundary trace",
                        case.name
                    );
                }
            }

            let properties = trace["composes"][0]["sva_properties"]
                .as_array()
                .unwrap_or_else(|| panic!("{} SVA properties should be an array", case.name));
            for property in properties {
                let contract_id = property["contract_id"]
                    .as_str()
                    .unwrap_or_else(|| panic!("{} property should have contract_id", case.name));
                assert!(
                    !contract_id.is_empty(),
                    "{} property has empty contract_id",
                    case.name
                );
            }
        }
    }

    #[test]
    fn emits_lintable_ready_valid_sva_skeleton() {
        let sva = emit_sva_skeleton(&stream_design());

        assert!(sva.starts_with("`default_nettype none"));
        assert!(sva.contains("module mico_sva_Top("));
        assert!(sva.contains("input logic [31:0] p_tx__c_rx_payload"));
        assert!(sva.contains("always_ff @(posedge clk)"));
        assert!(sva.contains("assert (p_tx__c_rx_payload =="));
        assert!(sva.ends_with("`default_nettype wire\n"));
    }

    fn stream_design() -> Design {
        Design {
            clock_domains: vec![ClockDomain {
                name: id("Sys"),
                clock: id("clk"),
                reset: id("rst"),
            }],
            interfaces: vec![stream_interface("StreamU32", "Sys", 32)],
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
                    from: endpoint("p", "tx"),
                    to: endpoint("c", "rx"),
                    adapter: None,
                }],
            }],
            source_map: SourceMap::default(),
        }
    }

    fn width_adapter_design() -> Design {
        let mut design = stream_design();
        design
            .interfaces
            .push(stream_interface("StreamU64", "Sys", 64));
        design.modules[0].name = id("Source32");
        design.modules[1].name = id("Sink64");
        design.modules[1].ports[0].interface = id("StreamU64");
        design.composes[0].instances[0].name = id("s");
        design.composes[0].instances[0].module = id("Source32");
        design.composes[0].instances[1].name = id("t");
        design.composes[0].instances[1].module = id("Sink64");
        design.composes[0].connections[0].from = endpoint("s", "tx");
        design.composes[0].connections[0].to = endpoint("t", "rx");
        design.composes[0].connections[0].adapter = Some(id("Widen32To64"));
        design.adapters.push(AdapterDef {
            name: id("Widen32To64"),
            from_interface: id("StreamU32"),
            from_domain: id("Sys"),
            to_interface: id("StreamU64"),
            to_domain: id("Sys"),
            kind: id("width_adapter"),
            attributes: vec![(id("contract"), "preserves_ready_valid".to_string())],
        });
        design
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
        design.interfaces[0] = stream_interface("StreamU32", "Aclk", 32);
        design
            .interfaces
            .push(stream_interface("StreamU32B", "Bclk", 32));
        design.modules[0].name = id("Dma");
        design.modules[0].domain = id("Aclk");
        design.modules[1].name = id("Aes");
        design.modules[1].domain = id("Bclk");
        design.modules[1].ports[0].interface = id("StreamU32B");
        design.composes[0].domain = id("Aclk");
        design.composes[0].instances[0].name = id("dma");
        design.composes[0].instances[0].module = id("Dma");
        design.composes[0].instances[1].name = id("aes");
        design.composes[0].instances[1].module = id("Aes");
        design.composes[0].connections[0].from = endpoint("dma", "tx");
        design.composes[0].connections[0].to = endpoint("aes", "rx");
        design.composes[0].connections[0].adapter = Some(id("AsyncFifo32"));
        design.adapters.push(AdapterDef {
            name: id("AsyncFifo32"),
            from_interface: id("StreamU32"),
            from_domain: id("Aclk"),
            to_interface: id("StreamU32B"),
            to_domain: id("Bclk"),
            kind: id("cdc_fifo"),
            attributes: vec![(id("contract"), "preserves_order".to_string())],
        });
        design
    }

    fn stream_interface(name: &str, domain: &str, width: u32) -> InterfaceDef {
        InterfaceDef {
            name: id(name),
            domain: id(domain),
            fields: vec![
                FieldDef {
                    name: id("payload"),
                    ty: ScalarType::UInt(width),
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
        }
    }

    fn endpoint(instance: &str, port: &str) -> Endpoint {
        Endpoint {
            instance: id(instance),
            port: id(port),
        }
    }

    fn id(value: &str) -> Ident {
        Ident::from(value)
    }
}
