use mico_ir::*;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

pub fn parse_mico(source: &str) -> Result<Design, Vec<ParseError>> {
    let mut parser = Parser::new(source);
    parser.parse();
    if parser.errors.is_empty() {
        Ok(parser.design)
    } else {
        Err(parser.errors)
    }
}

struct Parser<'a> {
    lines: Vec<(usize, &'a str)>,
    idx: usize,
    design: Design,
    errors: Vec<ParseError>,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Self {
        let lines = source
            .lines()
            .enumerate()
            .map(|(i, l)| (i + 1, strip_comment(l).trim()))
            .filter(|(_, l)| !l.is_empty())
            .collect();
        Self {
            lines,
            idx: 0,
            design: Design::default(),
            errors: Vec::new(),
        }
    }

    fn parse(&mut self) {
        while let Some((line_no, line)) = self.peek() {
            if line.starts_with("clockdom ") {
                self.parse_clockdom(line_no, line);
                self.idx += 1;
            } else if line.starts_with("interface ") {
                self.parse_interface();
            } else if line.starts_with("extern module ") || line.starts_with("module ") {
                self.parse_module();
            } else if line.starts_with("adapter ") {
                self.parse_adapter();
            } else if line.starts_with("compose ") {
                self.parse_compose();
            } else {
                self.err(line_no, format!("unrecognized declaration: `{line}`"));
                self.idx += 1;
            }
        }
    }

    fn peek(&self) -> Option<(usize, &'a str)> {
        self.lines.get(self.idx).copied()
    }

    fn next(&mut self) -> Option<(usize, &'a str)> {
        let v = self.peek();
        if v.is_some() {
            self.idx += 1;
        }
        v
    }

    fn err(&mut self, line: usize, message: String) {
        self.errors.push(ParseError { line, message });
    }

    fn parse_clockdom(&mut self, line_no: usize, line: &str) {
        // clockdom Sys(clk, rst);
        let rest = trim_stmt(line.trim_start_matches("clockdom "));
        match rest.split_once('(') {
            Some((name, args)) => {
                let args = args.trim_end_matches(')');
                let mut parts = args.split(',').map(str::trim);
                match (parts.next(), parts.next()) {
                    (Some(clk), Some(rst)) => self.design.clock_domains.push(ClockDomain {
                        name: Ident::from(name.trim()),
                        clock: Ident::from(clk),
                        reset: Ident::from(rst),
                    }),
                    _ => self.err(line_no, "clockdom requires clock and reset".to_string()),
                }
            }
            None => self.err(line_no, "invalid clockdom syntax".to_string()),
        }
    }

    fn parse_interface(&mut self) {
        let (line_no, header) = self.next().expect("peeked");
        let header = header.trim_end_matches('{').trim();
        let Some((name_part, domain_part)) =
            header.trim_start_matches("interface ").split_once('@')
        else {
            self.err(line_no, "interface must specify @ClockDomain".to_string());
            return;
        };
        let mut fields = Vec::new();
        let mut contracts = Vec::new();
        while let Some((ln, line)) = self.next() {
            if line == "}" {
                break;
            }
            if line.starts_with("producer ") {
                parse_fields(
                    line.trim_start_matches("producer "),
                    Role::Producer,
                    &mut fields,
                );
            } else if line.starts_with("consumer ") {
                parse_fields(
                    line.trim_start_matches("consumer "),
                    Role::Consumer,
                    &mut fields,
                );
            } else if line.starts_with("contract ") {
                let body = trim_stmt(line.trim_start_matches("contract "));
                if let Some((name, expr)) = body.split_once(':') {
                    contracts.push(ContractDef {
                        name: Ident::from(name),
                        expr: expr.trim().to_string(),
                    });
                } else {
                    self.err(ln, "contract must be `contract name: expr;`".to_string());
                }
            } else {
                self.err(ln, format!("invalid interface member: `{line}`"));
            }
        }
        self.design.interfaces.push(InterfaceDef {
            name: Ident::from(name_part.trim()),
            domain: Ident::from(domain_part.trim()),
            fields,
            contracts,
        });
    }

    fn parse_module(&mut self) {
        let (line_no, header) = self.next().expect("peeked");
        let is_extern = header.starts_with("extern module ");
        let prefix = if is_extern {
            "extern module "
        } else {
            "module "
        };
        let header = header.trim_end_matches('{').trim();
        let Some((name_part, domain_part)) = header.trim_start_matches(prefix).split_once('@')
        else {
            self.err(line_no, "module must specify @ClockDomain".to_string());
            return;
        };
        let mut ports = Vec::new();
        while let Some((ln, line)) = self.next() {
            if line == "}" {
                break;
            }
            let stmt = trim_stmt(line);
            let (dir, rest) = if let Some(rest) = stmt.strip_prefix("in ") {
                (PortDir::In, rest)
            } else if let Some(rest) = stmt.strip_prefix("out ") {
                (PortDir::Out, rest)
            } else {
                self.err(ln, format!("invalid module port: `{line}`"));
                continue;
            };
            if let Some((name, iface)) = rest.split_once(':') {
                ports.push(PortDef {
                    name: Ident::from(name),
                    dir,
                    interface: Ident::from(iface),
                });
            } else {
                self.err(
                    ln,
                    "port must be `in name: Interface;` or `out name: Interface;`".to_string(),
                );
            }
        }
        self.design.modules.push(ModuleDef {
            name: Ident::from(name_part.trim()),
            domain: Ident::from(domain_part.trim()),
            is_extern,
            ports,
        });
    }

    fn parse_adapter(&mut self) {
        let (line_no, header) = self.next().expect("peeked");
        // adapter AsyncFifo32 from StreamU32@Aclk to StreamU32@Bclk {
        let h = header.trim_end_matches('{').trim();
        let rest = h.trim_start_matches("adapter ");
        let Some((name, rem)) = rest.split_once(" from ") else {
            self.err(line_no, "adapter requires `from`".to_string());
            return;
        };
        let Some((from, to)) = rem.split_once(" to ") else {
            self.err(line_no, "adapter requires `to`".to_string());
            return;
        };
        let Some((from_if, from_dom)) = from.split_once('@') else {
            self.err(
                line_no,
                "adapter from endpoint requires Interface@Domain".to_string(),
            );
            return;
        };
        let Some((to_if, to_dom)) = to.split_once('@') else {
            self.err(
                line_no,
                "adapter to endpoint requires Interface@Domain".to_string(),
            );
            return;
        };
        let mut kind = Ident::from("custom");
        let mut attributes = Vec::new();
        while let Some((ln, line)) = self.next() {
            if line == "}" {
                break;
            }
            let stmt = trim_stmt(line);
            if let Some(k) = stmt.strip_prefix("kind ") {
                kind = Ident::from(k);
            } else if let Some((k, v)) = stmt.split_once(' ') {
                attributes.push((Ident::from(k), v.trim().to_string()));
            } else {
                self.err(ln, format!("invalid adapter member: `{line}`"));
            }
        }
        self.design.adapters.push(AdapterDef {
            name: Ident::from(name),
            from_interface: Ident::from(from_if),
            from_domain: Ident::from(from_dom),
            to_interface: Ident::from(to_if),
            to_domain: Ident::from(to_dom),
            kind,
            attributes,
        });
    }

    fn parse_compose(&mut self) {
        let (line_no, header) = self.next().expect("peeked");
        let header = header.trim_end_matches('{').trim();
        let Some((name_part, domain_part)) = header.trim_start_matches("compose ").split_once('@')
        else {
            self.err(line_no, "compose must specify @ClockDomain".to_string());
            return;
        };
        let mut instances = Vec::new();
        let mut connections = Vec::new();
        while let Some((ln, line)) = self.next() {
            if line == "}" {
                break;
            }
            let stmt = trim_stmt(line);
            if let Some(rest) = stmt.strip_prefix("inst ") {
                if let Some((name, module)) = rest.split_once(':') {
                    instances.push(InstanceDef {
                        name: Ident::from(name),
                        module: Ident::from(module),
                    });
                } else {
                    self.err(ln, "instance must be `inst name: Module;`".to_string());
                }
            } else if let Some(rest) = stmt.strip_prefix("connect ") {
                if let Some((from, to)) = rest.split_once(" -> ") {
                    match (Endpoint::parse(from), Endpoint::parse(to)) {
                        (Some(from), Some(to)) => connections.push(ConnectDef {
                            from,
                            to,
                            adapter: None,
                        }),
                        _ => self.err(ln, "connect endpoints must be `inst.port`".to_string()),
                    }
                } else {
                    self.err(ln, "connect must be `connect a.b -> c.d;`".to_string());
                }
            } else if let Some(rest) = stmt.strip_prefix("adapt ") {
                // adapt a.b -> AdapterName -> c.d;
                let parts: Vec<_> = rest.split(" -> ").collect();
                if parts.len() == 3 {
                    match (Endpoint::parse(parts[0]), Endpoint::parse(parts[2])) {
                        (Some(from), Some(to)) => connections.push(ConnectDef {
                            from,
                            to,
                            adapter: Some(Ident::from(parts[1])),
                        }),
                        _ => self.err(ln, "adapt endpoints must be `inst.port`".to_string()),
                    }
                } else {
                    self.err(
                        ln,
                        "adapt must be `adapt a.b -> Adapter -> c.d;`".to_string(),
                    );
                }
            } else {
                self.err(ln, format!("invalid compose member: `{line}`"));
            }
        }
        self.design.composes.push(ComposeDef {
            name: Ident::from(name_part.trim()),
            domain: Ident::from(domain_part.trim()),
            instances,
            connections,
        });
    }
}

fn strip_comment(line: &str) -> &str {
    line.split_once("//").map(|(a, _)| a).unwrap_or(line)
}

fn trim_stmt(s: &str) -> &str {
    s.trim().trim_end_matches(';').trim()
}

fn parse_fields(s: &str, role: Role, fields: &mut Vec<FieldDef>) {
    let s = trim_stmt(s);
    for item in s.split(',').map(str::trim).filter(|x| !x.is_empty()) {
        if let Some((name, ty)) = item.split_once(':') {
            fields.push(FieldDef {
                name: Ident::from(name),
                ty: ScalarType::parse(ty),
                role: role.clone(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_clockdom_only() {
        let d = parse_mico("clockdom Sys(clk, rst);").unwrap();
        assert_eq!(d.clock_domains.len(), 1);
    }
}
