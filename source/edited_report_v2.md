# 面向 LLM 的模块中心 RTL 互连语言：CCF-A 视角评估、可行方案与创新点重构

**版本**：v2 编辑稿
**主题**：RTL 模块化、自动连线、LLM 协同硬件设计
**建议短名**：MICO-HDL / MICO-Connect（Module-Interface-Contract-Oriented HDL）

---

## 0. 本次编辑结论

原始想法可以保留，但需要从“重新发明 HDL，让 LLM 自动连线”改写为一个更窄、更强、也更容易发表的研究问题：

> **不是让 LLM 直接替代 RTL 工程师写 Verilog，而是设计一个模块中心的接口/契约/互连前端，把复杂连线转化为可类型检查、可形式验证、可降级到 Verilog/SystemVerilog 的受约束图综合问题。**

本次编辑重点做了四类调整：

1. **收窄问题边界**：先不做完整 HDL，也不替代所有 RTL；先做“已有 leaf RTL / IP 模块的接口建模、拓扑连接、adapter 合成与验证”。
2. **强化论文创新点**：引入接口类型、时序域、协议契约、adapter synthesis、LLM 受约束生成、ModuleComposeBench 六个支点。
3. **加入最接近相关工作**：尤其是 2026 年 CPPL，它已经提出“LLM 友好的前端 DSL + JSON IR + CIRCT 降级”。因此本 idea 的新颖性必须避开“结构化前端 + CIRCT”这一已被占据的表述，转向“互连级 DSL + 协议/时序契约 + 自动 adapter + 模块组合 benchmark”。
4. **给出 CCF-A 严格版本的方案**：包含形式化问题定义、系统架构、算法、实验设计、对比基线、评分与风险。

---

## 1. 执行摘要

该 idea **值得做**，但原始表述下还不足以达到 CCF-A 强度。若只说“做一种新语言，让 LLM 根据模块自动连线”，审稿人会立刻联想到 SystemVerilog interface、Chisel Bundle/bulk connect、Amaranth wiring、CIRCT/ESI、CPPL 等已有工作。这样会被认为是“工程整合”而不是明确的新问题。

优化后的核心命题应当是：

> **面向 LLM 的模块互连语言与编译器：把模块连接从低层端口绑定提升为接口图、协议契约与 adapter 合成；LLM 只负责提出候选连接/拓扑，编译器负责类型、时序、协议和形式性质的验证，并确定性降级到 RTL。**

该方向的最佳投稿形态不是“又一门 HDL”，而是：

- 一个**窄核心语言**：`interface`、`clockdom`、`contract`、`adapter`、`compose`；
- 一个**形式化模型**：连接合法性、协议 refinement、CDC 禁止隐式通过、adapter 保真；
- 一个**LLM 协同生成系统**：受约束 JSON/AST 生成 + 编译器诊断反馈 + 局部修复；
- 一个**新 benchmark**：ModuleComposeBench，专门评估模块互连、协议桥接、CDC、wrapper、top-level glue code，而不是只评估单模块 Verilog 生成；
- 一个**端到端工具链**：导入现有 RTL 模块，生成 wrapper/top/interconnect/SVA，降级到 Verilog/SystemVerilog，接入 lint、仿真、综合、形式验证。

### 总评分

| 版本 | 评分 | 判断 |
|---|---:|---|
| 原始 idea：新式 HDL + LLM 自动连线 | **6.4 / 10** | 有价值，但创新边界不清，容易被 SystemVerilog interface、Chisel、Amaranth、CPPL 覆盖。 |
| 优化后 idea：模块互连 DSL + 契约验证 + adapter 合成 + LLM 受约束生成 | **8.1 / 10** | 可以形成较强 CCF-A 论文雏形；关键在 benchmark、形式化、强基线和开源 artifact。 |
| 若实现充分、实验扎实、对 CPPL/Chisel/Amaranth 有强对比 | **8.4 / 10** | 有希望冲击 DAC/ICCAD/ASPLOS/PLDI/OOPSLA 等 A 类取向会议。 |

---

## 2. 为什么这个问题真实存在

### 2.1 Verilog 的痛点不是“语法”，而是“连接熵”

传统 RTL 顶层集成代码常包含大量：

- 端口声明；
- 宽度对齐；
- ready/valid 等握手信号成组传递；
- clock/reset 扩散；
- wrapper 与桥接逻辑；
- CDC/RDC 边界；
- 协议转换；
- 参数化实例化；
- 命名约定依赖。

这些代码很多不表达核心功能，而是表达“如何把已有东西接起来”。这类文本对人类是负担，对 LLM 也是负担，因为 LLM 需要在长上下文中追踪大量远距离约束。

可以定义一个研究指标：

```text
Connection Entropy CE = primitive_signal_edges / semantic_interface_edges
```

其中：

- `primitive_signal_edges`：最终 RTL 中逐根 wire/port 连接数量；
- `semantic_interface_edges`：设计者真正关心的接口级连接数量，例如 `producer.tx -> fifo.in`。

如果一个顶层模块中 300 根低层线实际只对应 15 条语义连接，那么 CE = 20。CE 越高，LLM 越容易出现漏连、错连、方向错、宽度错、reset 漏传、ready/valid 反接等问题。

### 2.2 SystemVerilog/Chisel/Amaranth 已说明“接口化”是正确方向

SystemVerilog interface 已经证明，端口组和 modport 是必要抽象；Chisel 官方文档也明确指出 interface class 能复用接口、显著减少 wiring，并通过 bulk connection 连接 producer/consumer；Amaranth 的 `amaranth.lib.wiring` 也提供 Signature、In/Out、connect 等机制来可靠、方便地声明和连接组件接口。

这说明你的直觉是对的：**模块连接不应该长期停留在逐信号手工连线层面。**

但现有机制仍不足：

| 机制 | 已解决 | 未解决 |
|---|---|---|
| SystemVerilog interface | 信号打包、方向 modport | LLM 友好结构化生成、协议契约、跨域 adapter、形式化连接搜索不足 |
| Chisel Bundle / Decoupled / bulk connect | 强接口、生成式编程、Scala 生态 | 宿主语言复杂，LLM 需理解 Scala/elaboration；契约和自动 adapter 不是核心目标 |
| Amaranth Signature / connect | Python 中清晰声明方向和接口 | 更偏 Python HDL 组件机制；缺少面向 LLM 的互连图综合与 benchmark |
| CIRCT / ESI | typed IR、channel、service、Verilog emission | 主要是后端/IR，不是人类与 LLM 友好的前端语言 |
| CPPL | LLM 友好前端 + JSON IR + CIRCT | 更偏模块内部电路生成与层次结构；没有把协议契约、CDC、adapter 合成和互连 benchmark 作为主问题 |

因此，研究切入点不应是“有没有接口抽象”，而是：

> **如何把接口抽象扩展成 LLM 可生成、编译器可验证、adapter 可合成、协议/时序可证明的模块互连系统。**

---

## 3. 相关工作与差异定位

### 3.1 最接近工作：CPPL

CPPL 是当前最接近本 idea 的工作。它提出：

- Python 前端 DSL 声明模块接口和层次；
- JSON 型 CPPL IR 供 LLM 生成；
- 编译器做宽度推断、结构检查、端口绑定检查；
- 降级到 CIRCT，再生成 synthesizable Verilog；
- 在 RTLLM 上优于 direct Verilog 和 direct CIRCT IR generation。

这会削弱原始 idea 的新颖性。若论文只说“LLM 不直接写 Verilog，而写结构化 IR，再由 CIRCT 生成 RTL”，会与 CPPL 高度重叠。

### 3.2 与 CPPL 的建议差异

你的方向必须从 CPPL 的“电路生成前端”切到“模块互连与契约综合”。建议差异如下：

| 维度 | CPPL | 建议方案 MICO-HDL |
|---|---|---|
| 核心任务 | 从规格生成模块电路/层次 | 从已有模块库生成互连、wrapper、adapter、contract |
| LLM 输出 | JSON circuit IR / operation graph | interface graph、compose graph、adapter plan、contract skeleton |
| 关键检查 | 宽度、端口、层次、IR 合法性 | 宽度、方向、角色、clock domain、reset、协议、latency、CDC/RDC、liveness/safety |
| 后端 | CIRCT | CIRCT HW/ESI/Verif/LTL + SVA + conservative SV |
| benchmark | RTLLM | ModuleComposeBench：模块组合、协议桥接、CDC、bus/stream interconnect |
| 主要创新 | compiler-mediated LLM generation | contract-guided module interconnect synthesis |
| 论文卖点 | LLM 生成更可靠 | LLM 连接更可靠，互连错误可静态拒绝，adapter 可合成并可验证 |

一句话：

> **CPPL 让 LLM 更容易“生成电路”；MICO-HDL 让 LLM 更安全地“组合电路”。**

这是可发表差异点。

### 3.3 其他相关工作综述

| 类别 | 代表 | 对本课题的意义 | 对创新性的威胁 |
|---|---|---|---|
| 传统 HDL | Verilog/SystemVerilog interface | 证明接口封装有需求 | 中等：说明“少连线”本身不是新贡献 |
| 嵌入式 HDL | Chisel、Amaranth、SpinalHDL | 强接口、bulk connect、生成式硬件 | 高：必须说明为什么不是 Chisel 插件 |
| 现代 HDL | Veryl、Spade、Anvil | 现代语法、时序安全、类型系统 | 中高：Anvil 的 timing contract 很接近“时序安全组合” |
| 编译器 IR | CIRCT HW/ESI/Verif/LTL、FIRRTL、Calyx | 后端和形式验证基础 | 低到中：可作为后端复用，不必竞争 |
| LLM4RTL | VerilogEval、RTLLM、OpenLLM-RTL、ProtocolLLM | 证明直接 Verilog 生成仍有明显缺口 | 低：它们是 benchmark/baseline，不是同类语言 |
| 形式验证 | AutoSVA | 证明模块交互可抽象成 transaction 并自动生成 SVA | 中高：需要扩展到连接综合和契约驱动编译 |
| 结构化生成 | Grammar-Constrained / Grammar-Aligned Decoding | 支持“LLM 输出受约束 AST/JSON/DSL” | 低：可作为方法组件 |

---

## 4. 重新定义研究问题

### 4.1 不推荐的表述

以下表述不够强：

> 做一种新的 HDL，比 Verilog 更适合 LLM，模块定义好后让 LLM 自动连线。

问题：

- “新 HDL”范围过大；
- “适合 LLM”定义不清；
- “自动连线”像工程便利功能；
- 与 Chisel、Amaranth、SystemVerilog interface、CPPL 的边界不清；
- 难以证明 CCF-A 级别的科学贡献。

### 4.2 推荐的 CCF-A 级表述

推荐改为：

> **给定一组已有 RTL/IP 模块、接口签名、时钟/复位域、协议契约和系统级连接意图，如何利用 LLM 生成候选模块互连图，并由编译器静态验证方向、宽度、协议、时序域与安全/活性性质，必要时自动合成 adapter，最终生成可综合 RTL 和可验证 SVA？**

这将问题变成：

- 一个 PL/EDA 问题：语言、类型系统、语义、编译；
- 一个形式方法问题：contract、refinement、SVA、LTL；
- 一个 LLM-for-Hardware 问题：结构化生成、反馈修复、benchmark；
- 一个工程系统问题：导入现有 Verilog，输出可综合 RTL。

### 4.3 形式化定义

设模块库为：

```text
M = {m_i}
```

每个模块 `m` 有接口集合：

```text
I(m) = {p_j : InterfaceType(role, width, domain, reset, protocol, latency, contract)}
```

一个连接候选为：

```text
c = (m_a.p_x -> m_b.p_y)
```

连接合法当且仅当：

```text
Compatible(c) =
    RoleCompatible(p_x, p_y)
 ∧ WidthCompatible(p_x, p_y) or ExistsAdapter(width)
 ∧ DomainCompatible(p_x, p_y) or ExistsAdapter(CDC/RDC)
 ∧ ProtocolRefines(p_x.protocol, p_y.protocol) or ExistsAdapter(protocol_bridge)
 ∧ LatencyCompatible(p_x, p_y)
 ∧ ContractSatisfiable(p_x.contract, p_y.contract)
```

自动连线不是“猜”，而是一个受约束搜索问题：

```text
Find graph G = (V, E, A)
where:
  V = module instances
  E = direct interface connections
  A = inserted adapters
such that:
  all edges are compatible,
  all required endpoints are connected,
  no forbidden implicit CDC/RDC exists,
  all generated adapters satisfy refinement obligations,
  generated RTL passes type/lint/sim/formal/synthesis checks.
```

LLM 的作用：

```text
LLM proposes candidate G and missing annotations.
Compiler validates, repairs locally, or rejects.
```

编译器的作用：

```text
Compiler is the authority, not the LLM.
```

这是整个方案的安全边界。

---

## 5. 可行方案：MICO-HDL / MICO-Connect

### 5.1 语言核心

MICO-HDL 不应首先覆盖完整 RTL。第一版只需要五个一等对象：

| 对象 | 作用 | 示例 |
|---|---|---|
| `interface` | 描述成组端口、角色、方向、协议 | `Stream<T>`、`AXILiteMaster`、`APBSlave` |
| `clockdom` | 描述时钟/复位域 | `Sys(clk, rst, sync, active_high)` |
| `contract` | 描述安全/活性/稳定性/延迟性质 | `valid -> stable(payload) until ready` |
| `adapter` | 描述非直连转换 | width bridge、CDC FIFO、pipeline、protocol bridge |
| `compose` | 描述模块实例与接口连接图 | `connect cpu.imem -> axi_xbar.s0` |

### 5.2 最小语言示例

```text
clockdom Sys(clk: clock, rst: reset(sync, active_high))
clockdom AxiClk(aclk: clock, aresetn: reset(sync, active_low))

interface Stream<T> @domain {
  role producer {
    payload : T
    valid   : Bool
  }
  role consumer {
    ready   : Bool
  }

  contract ready_valid {
    safety stable_payload: valid & !ready -> stable(payload)
    event fire := valid & ready
  }
}

extern module Producer(out tx: Stream<u32> @Sys)
extern module Consumer(in  rx: Stream<u32> @Sys)
extern module Fifo<T, depth>(in enq: Stream<T> @Sys, out deq: Stream<T> @Sys)

compose Top @Sys {
  inst p = Producer()
  inst q = Fifo<u32, depth=4>()
  inst c = Consumer()

  connect p.tx -> q.enq
  connect q.deq -> c.rx
}
```

跨域时不允许隐式通过：

```text
compose CrossDomainTop {
  inst src = Producer() @Sys
  inst dst = Consumer() @AxiClk

  // 合法：显式 adapter
  adapt src.tx -> AsyncFifo<u32>(src=Sys, dst=AxiClk, depth=8) -> dst.rx

  // 非法：直接跨域连接
  // connect src.tx -> dst.rx
}
```

### 5.3 编译流程

```text
Verilog/SystemVerilog leaf modules
        │
        ▼
interface extractor / manual interface schema
        │
        ▼
MICO interface + contract library
        │
        ▼
LLM generates compose graph / adapter plan / missing annotations
        │
        ▼
MICO compiler
  - name resolution
  - type checking
  - role/direction checking
  - width/domain/reset checking
  - protocol compatibility checking
  - adapter insertion
  - contract elaboration
        │
        ▼
CIRCT HW / ESI / Verif / LTL lowering
        │
        ▼
SystemVerilog / Verilog + SVA + wrapper reports
        │
        ▼
lint / simulation / formal / synthesis / PPA evaluation
        │
        ▼
structured diagnostics back to LLM
```

### 5.4 为什么不是“LLM 魔法连线”

该系统必须默认保守：

- 多个候选接收方匹配时，报 `AmbiguousConnectError`；
- 宽度不一致时，必须有显式或可证明 adapter；
- clock/reset domain 不一致时，必须插入 CDC/RDC adapter；
- 协议不同，例如 APB to AXI-Lite，必须是协议桥；
- ready/valid、AXI channel、request/response 的 temporal contract 必须生成 SVA 或 LTL obligation；
- 编译器不能因为 LLM 猜测而静默接线。

建议原则：

> **LLM proposes, compiler disposes.**

---

## 6. 核心创新点设计

### 创新点 1：面向 LLM 的模块互连语言，而不是完整 HDL

贡献表述：

> 提出一种专门描述模块接口、连接拓扑、时序域、协议契约和 adapter 的窄 HDL 前端，将顶层 RTL 连接从逐信号文本生成问题转化为接口图综合问题。

价值：

- 比完整 HDL 更容易落地；
- 可以复用现有 RTL/IP；
- 直接打中 LLM 最容易犯错的地方：port binding、命名、宽度、协议、CDC；
- 审稿人更容易接受问题边界。

### 创新点 2：契约驱动的自动连线与 adapter 合成

贡献表述：

> 定义接口契约和连接类型系统，使直接连接、协议桥、pipeline stage、CDC FIFO、width adapter 都有明确的合法性条件和证明义务。

可形式化性质：

```text
Theorem 1: Well-typed connection safety
If compose graph G type-checks, then generated RTL has no width/direction/domain mismatch.

Theorem 2: No implicit CDC
If two endpoints are in different clock domains, every path between them contains a declared CDC adapter.

Theorem 3: Adapter refinement
If adapter A is inserted between interface P and Q, and A satisfies refinement obligation O_A,
then all properties required by Q are preserved under assumptions exported by P.
```

### 创新点 3：LLM 受约束生成的互连图，而非自由文本 Verilog

贡献表述：

> 设计一种 LLM-facing schema，让模型输出 interface graph / adapter plan / contract skeleton，再经语法约束、类型检查和 EDA 反馈迭代修复。

示例 JSON：

```json
{
  "instances": [
    {"name": "cpu", "module": "IbexCore"},
    {"name": "xbar", "module": "AXILiteXbar"},
    {"name": "uart", "module": "UartApb"}
  ],
  "connections": [
    {"from": "cpu.data_axi", "to": "xbar.s0"},
    {"from": "xbar.m0", "to": "uart.apb", "adapter": "AXILiteToAPB"}
  ],
  "contracts": [
    {"interface": "cpu.data_axi", "property": "eventual_response", "bound": 32}
  ]
}
```

该 JSON 不是最终交付，编译器会把它 lowering 到 MICO IR，再生成 RTL。

### 创新点 4：ModuleComposeBench

现有 VerilogEval/RTLLM 主要评估从自然语言生成单模块 RTL；ProtocolLLM 关注协议模块生成；它们并没有系统评估“已有模块库的互连、adapter、CDC、top wrapper”。因此可提出一个新 benchmark：

> **ModuleComposeBench：评估 LLM 在已有 RTL/IP 模块库上完成接口抽象、模块组合、adapter 插入、协议连接、CDC 处理和顶层 wrapper 生成的能力。**

任务类型：

| 任务 | 例子 | 主要错误 |
|---|---|---|
| Simple compose | producer -> FIFO -> consumer | ready/valid 反接、漏 ready |
| Bus attach | CPU AXI-Lite -> xbar -> UART/GPIO | address map、channel 漏接 |
| Protocol bridge | AXI-Lite -> APB | response/latency、ready/valid 语义 |
| CDC compose | sensor_clk -> async FIFO -> sys_clk | 隐式跨域、reset 错 |
| Wrapper generation | existing Verilog leaf -> typed interface | port 命名映射错、宽度错 |
| Ambiguous graph | 多个 master/slave 匹配 | 错误自动猜测 |
| Fault injection | 随机扰动 width/domain/protocol | 类型系统是否拒绝 |

### 创新点 5：连接熵与可维护性指标

除 pass@1 外，提出互连专用指标：

| 指标 | 含义 |
|---|---|
| Compose-Pass@1 | 一次生成的 compose graph 通过类型/协议检查比例 |
| Adapter-Pass@1 | 自动选择正确 adapter 的比例 |
| Contract-Proof Rate | 自动生成性质被形式工具证明的比例 |
| CDC-Rejection Rate | 非法跨域连接被拒绝的比例 |
| Ambiguity-Rejection Precision | 多重候选时是否拒绝而不是乱连 |
| Connection Entropy Reduction | 语义连接数相对低层 wire 数的压缩率 |
| Human Edit Distance | 人类需要修改的连接/adapter 数 |
| RTL QoR Delta | 面积、时序、功耗相对手写基线的偏差 |
| Diagnostic Repair Turns | LLM 根据编译器诊断修复所需轮数 |

这些指标能让论文不是“演示系统”，而是可量化研究。

---

## 7. CCF-A 严格评审下的可行性分析

### 7.1 原始 idea 的弱点

| 问题 | 严重程度 | 说明 |
|---|---:|---|
| “新 HDL”过大 | 高 | 需要解释为何不是 Chisel、Amaranth、Veryl、Spade、Anvil、CPPL。 |
| “适合 LLM”不可测 | 高 | 必须定义结构合法率、类型通过率、修复轮数、token 成本等指标。 |
| “自动连线”像工程功能 | 高 | 要上升为连接类型系统、契约、adapter synthesis。 |
| 缺少 benchmark | 高 | CCF-A 很难只凭语言设计接受，必须有强实验。 |
| 缺少理论贡献 | 中高 | 需要至少一个 soundness/refinement/typing theorem。 |
| 工业落地风险 | 中 | 新语言采纳难；需要输出保守 Verilog/SV、导入已有 RTL。 |

### 7.2 优化后可行性

| 维度 | 评分 | 解释 |
|---|---:|---|
| 问题重要性 | 8.5 | 顶层互连、wrapper、adapter、CDC 是真实痛点；LLM 直接写 RTL 的失败模式也集中在结构和时序约束。 |
| 新颖性 | 7.8 | CPPL/Chisel/Amaranth/Anvil 形成强相关工作，但“互连级 + contract + adapter + benchmark”仍有空间。 |
| 技术深度 | 8.4 | 类型系统、协议契约、adapter refinement、CIRCT lowering、形式验证闭环都可形成实质贡献。 |
| 工程可行性 | 7.6 | 可复用 CIRCT、Verilator、Yosys/SymbiYosys、cocotb；难点在接口抽取与协议库。 |
| 实验可验证性 | 8.2 | 可以构建 ModuleComposeBench，并与 direct Verilog、SV interface、Chisel、Amaranth、CPPL 对比。 |
| 影响力 | 7.8 | 若工具可导入现有 RTL，会对 SoC 集成、IP glue、教育和 LLM4EDA 有明显价值。 |
| CCF-A 命中率 | 7.3 | 取决于形式化是否扎实、benchmark 是否真实、artifact 是否可复现。 |

### 7.3 最终评分

| 方案 | 综合评分 | 推荐度 |
|---|---:|---|
| 只做“LLM 友好 HDL” | 6.4 | 不推荐作为论文主线。 |
| 做“SystemVerilog/Chisel 自动连线插件” | 7.2 | 工程可行，但学术新颖性较弱。 |
| 做“CPPL 类结构化前端 + CIRCT” | 7.0 | 已有 CPPL，除非有明显新理论或实验。 |
| 做“MICO-HDL：契约驱动模块互连 DSL + adapter synthesis + ModuleComposeBench” | **8.1** | 推荐主线。 |
| 做“完整新 HDL + timing-safe + LLM + industrial flow” | 7.0 | 上限高，但范围过大，周期长，失败风险高。 |

---

## 8. 论文主张与摘要草案

### 8.1 论文标题建议

1. **Contract-Guided Module Composition for LLM-Assisted RTL Integration**
2. **MICO: A Module-Interface-Contract Language for Safe LLM-Assisted Hardware Interconnect**
3. **From Port Binding to Interface Graphs: Compiler-Checked RTL Composition with Large Language Models**
4. **LLM-Assisted Hardware Integration via Typed Interfaces, Protocol Contracts, and Adapter Synthesis**

最推荐标题：

> **MICO: Contract-Guided Module Composition for LLM-Assisted RTL Integration**

原因：

- MICO 是系统名；
- Contract-Guided 强调理论；
- Module Composition 避开“又一门 HDL”；
- LLM-Assisted RTL Integration 直接点出应用。

### 8.2 摘要草案

> Large language models have shown promise in RTL generation, yet direct Verilog emission remains brittle for integration-heavy designs where correctness depends on port binding, protocol semantics, clock-domain boundaries, and glue logic. We observe that many LLM failures in hardware design arise not from local datapath construction but from high-entropy module interconnect. This paper presents MICO, a module-interface-contract language and compiler for LLM-assisted RTL integration. MICO represents hardware systems as typed interface graphs with explicit roles, clock/reset domains, temporal contracts, and adapter obligations. LLMs propose candidate composition graphs, while the compiler statically checks direction, width, protocol, domain, and contract compatibility, synthesizes required adapters, and lowers verified designs to SystemVerilog and SVA through CIRCT. We introduce ModuleComposeBench, a benchmark for evaluating module-level composition across wrapper generation, bus attachment, protocol bridging, CDC insertion, and ambiguity rejection. Compared with direct Verilog generation, SystemVerilog interface prompting, and compiler-mediated baselines, MICO improves composition pass rates, reduces human edits, and catches unsafe interconnect before simulation or synthesis. Our results suggest that LLM-assisted hardware design should move from free-form RTL text generation toward compiler-checked module composition.

---

## 9. 实验设计

### 9.1 对比基线

必须包含强基线，否则 CCF-A 很容易被拒。

| 基线 | 目的 |
|---|---|
| Direct Verilog prompting | 证明直接写顶层 RTL 的错误率高。 |
| Direct SystemVerilog interface prompting | 证明仅用 SV interface 不够。 |
| Chisel / Amaranth 风格生成 | 对比已有接口/bulk connect DSL。 |
| CPPL-style JSON IR | 对比最接近的 LLM-friendly compiler frontend。 |
| Deterministic solver without LLM | 区分 LLM 的收益和编译器规则的收益。 |
| LLM + compiler feedback | 测试诊断修复闭环。 |

### 9.2 评测任务

建议分三级：

#### Level 1：接口连接

- ready/valid stream；
- request/response；
- simple FIFO pipeline；
- producer/consumer graph。

#### Level 2：adapter 合成

- width up/down cast；
- signed/unsigned conversion；
- skid buffer；
- pipeline register；
- sync FIFO；
- AXI-Lite to APB bridge。

#### Level 3：SoC 子系统集成

- CPU + bus + UART/GPIO/timer；
- DMA + SRAM + interrupt controller；
- multi-clock sensor subsystem；
- accelerator + host interface；
- NoC/router port composition。

### 9.3 指标

| 指标 | 意义 |
|---|---|
| Type Pass@1 | 生成结果一次通过 MICO 类型/契约检查比例 |
| Compose Pass@1 | 完整互连图一次通过比例 |
| Lint Pass@1 | 生成 RTL 通过 Verilator/slang 等检查比例 |
| Simulation Pass@1 | testbench 通过比例 |
| Formal Pass | SVA/LTL 性质证明比例 |
| Synthesis Pass | Yosys 或商业工具综合通过比例 |
| QoR Delta | 面积/时序/功耗相对手写基线偏差 |
| Repair Turns | 编译器反馈后修复轮数 |
| Token Cost | 输入输出 token 成本 |
| Human Fix Count | 人工修复连接数量 |
| Unsafe Rejection | 非法连接被拒绝比例 |

### 9.4 消融实验

| 消融 | 目的 |
|---|---|
| 去掉 contract | 证明协议/时序契约有用。 |
| 去掉 adapter synthesis | 证明自动桥接有用。 |
| 去掉 grammar-constrained decoding | 证明结构化生成有用。 |
| 去掉 compiler feedback | 证明闭环修复有用。 |
| 去掉 clockdom type | 证明 CDC/RDC 静态检查有用。 |
| 只用 LLM 不用 deterministic checker | 证明不能信任自由生成。 |

---

## 10. 工程实现路线

### Phase A：互连层最小原型

目标：支持已有 Verilog 模块导入和顶层 wrapper 生成。

功能：

- 解析模块端口；
- 手写或半自动生成 interface schema；
- 支持 `compose`；
- 检查方向、宽度、缺失连接、多驱动；
- 输出 Verilog wrapper/top。

### Phase B：协议接口库

目标：让常见接口一等化。

接口库：

- ReadyValid / Decoupled；
- ReqResp；
- AXI-Lite；
- APB；
- Wishbone；
- TileLink 子集；
- SRAM-like interface；
- interrupt/event interface。

### Phase C：adapter synthesis

目标：非同构接口也能安全连接。

adapter：

- width adapter；
- pipeline/skid buffer；
- FIFO；
- async FIFO；
- reset synchronizer；
- AXI-Lite to APB；
- stream packetizer/depacketizer。

### Phase D：contract 与验证

目标：每种接口和 adapter 生成 SVA/LTL。

输出：

- interface assumptions；
- module guarantees；
- adapter refinement properties；
- deadlock/liveness checks；
- binding files。

### Phase E：LLM pipeline

目标：把 LLM 限制在可检查结构里。

流程：

1. 检索模块库和接口文档；
2. 生成候选接口映射；
3. 生成 compose graph；
4. 编译器诊断；
5. LLM 局部修复；
6. 生成最终 RTL/SVA/report。

---

## 11. 风险与应对

| 风险 | 影响 | 应对 |
|---|---:|---|
| CPPL 已经很接近 | 高 | 明确差异：互连级、契约、CDC、adapter、ModuleComposeBench。 |
| 新语言 adoption 难 | 高 | 定位为互连前端，不替代 leaf RTL；导入 Verilog，导出 Verilog/SV。 |
| 协议契约难写 | 中高 | 先覆盖 ready/valid、req/resp、AXI-Lite/APB 子集。 |
| LLM 仍会胡连 | 中 | 编译器保守拒绝；LLM 只提出候选，不拥有最终决策权。 |
| adapter 影响 PPA | 中 | 统计 QoR delta；支持 no-auto-adapter 策略；显示插入报告。 |
| benchmark 被认为太人工 | 中 | 结合真实开源 RTL + synthetic fault injection + hand-written baseline。 |
| 形式验证状态爆炸 | 中 | 仅验证接口控制逻辑，不验证完整 datapath。 |
| 工具链不稳定 | 中 | 第一版输出保守 SystemVerilog/Verilog，CIRCT 作为可选优化后端。 |

---

## 12. 最终判断

### 12.1 是否可行

**可行。** 技术支撑已经存在：

- 接口抽象：SystemVerilog interface、Chisel、Amaranth；
- LLM 结构化生成：grammar-constrained / grammar-aligned decoding；
- 硬件 IR 后端：CIRCT HW/ESI/Verif/LTL；
- 形式验证路径：SVA、AutoSVA、SymbiYosys/Jasper 类工具；
- LLM4RTL benchmark：VerilogEval、RTLLM、OpenLLM-RTL、ProtocolLLM；
- 近邻前端：CPPL。

### 12.2 是否有创新

**有，但需要优化定位。**

原始创新点“不够保险”；优化后创新点可以是：

1. **第一个面向 LLM 的模块互连专用语言/IR**；
2. **接口 + 时钟域 + 协议 + 契约的一体化类型系统**；
3. **契约驱动 adapter synthesis**；
4. **LLM 候选连接图 + 编译器验证/拒绝/修复闭环**；
5. **ModuleComposeBench：互连级 LLM4RTL 基准**；
6. **从 MICO 降级到 Verilog/SV/SVA/CIRCT 的端到端 artifact**。

### 12.3 能否达到 CCF-A

结论：**有机会，但必须按系统论文 + PL/EDA 论文标准做。**

最低 CCF-A 要求：

- 与 CPPL、Chisel、Amaranth、SV interface、AutoSVA、Anvil 明确对比；
- 有形式化定义，不只是语法设计；
- 有公开可复现工具链；
- 有新 benchmark；
- 有强基线；
- 有真实或半真实 RTL case study；
- 有 QoR 和安全拒绝能力评估；
- 不把 LLM 输出当正确性来源。

### 12.4 总分

| 项目 | 原始 idea | 优化后 idea |
|---|---:|---:|
| 问题重要性 | 8.0 | 8.5 |
| 新颖性 | 6.0 | 7.8 |
| 技术深度 | 5.8 | 8.4 |
| 可实现性 | 7.2 | 7.6 |
| 实验可验证性 | 6.0 | 8.2 |
| 工程影响 | 7.0 | 8.0 |
| CCF-A 潜力 | 5.8 | 7.3 |
| 综合 | **6.4** | **8.1** |

最终建议：

> **继续推进，但不要以“新 HDL 替代 Verilog”为论文主线。主线应改成“面向 LLM 的契约驱动模块互连编译器”。这是更窄、更硬、更容易证明贡献，也更容易与 CPPL 等近邻工作区分的版本。**

---

## 13. 推荐引用文献

[R1] Shuo Yin et al. *CPPL: A Circuit Prompt Programming Language*. arXiv:2605.17892, 2026. https://arxiv.org/abs/2605.17892
[R2] Chisel Documentation. *Interfaces & Connections*. https://www.chisel-lang.org/docs/explanations/interfaces-and-connections
[R3] Amaranth Documentation. *Interfaces and connections*. https://amaranth-lang.org/docs/amaranth/latest/stdlib/wiring.html
[R4] CIRCT Documentation. *HW Dialect*. https://circt.llvm.org/docs/Dialects/HW/
[R5] CIRCT Documentation. *ESI Dialect*. https://circt.llvm.org/docs/Dialects/ESI/
[R6] Jason Zhijingcheng Yu et al. *Anvil: A General-Purpose Timing-Safe Hardware Description Language*. arXiv:2503.19447, accepted ASPLOS 2026. https://arxiv.org/abs/2503.19447
[R7] Marcelo Orenes-Vera et al. *AutoSVA: Democratizing Formal Verification of RTL Module Interactions*. arXiv:2104.04003, 2021. https://arxiv.org/abs/2104.04003
[R8] Mingjie Liu et al. *VerilogEval: Evaluating Large Language Models for Verilog Code Generation*. arXiv:2309.07544, 2023. https://arxiv.org/abs/2309.07544
[R9] Yao Lu et al. *RTLLM: An Open-Source Benchmark for Design RTL Generation with Large Language Model*. arXiv:2308.05345, 2023. https://arxiv.org/abs/2308.05345
[R10] Shang Liu et al. *OpenLLM-RTL: Open Dataset and Benchmark for LLM-Aided Design RTL Generation*. arXiv:2503.15112, ICCAD 2024. https://arxiv.org/abs/2503.15112
[R11] Arnav Sheth et al. *ProtocolLLM: RTL Benchmark for SystemVerilog Generation of Communication Protocols*. arXiv:2506.07945, 2025. https://arxiv.org/abs/2506.07945
[R12] Weimin Fu et al. *Synthesis-in-the-Loop Evaluation of LLMs for RTL Generation: Quality, Reliability, and Failure Modes*. arXiv:2603.11287, 2026. https://arxiv.org/abs/2603.11287
[R13] Yi Liu et al. *DeepRTL: Bridging Verilog Understanding and Generation with a Unified Representation Model*. arXiv:2502.15832, ICLR 2025 Spotlight. https://arxiv.org/abs/2502.15832
[R14] Saibo Geng et al. *Grammar-Constrained Decoding for Structured NLP Tasks without Finetuning*. EMNLP 2023. https://arxiv.org/abs/2305.13971
[R15] Kanghee Park et al. *Grammar-Aligned Decoding*. NeurIPS 2024. https://arxiv.org/abs/2405.21047
[R16] Naoya Hatta et al. *Veryl: A New Hardware Description Language as an Alternative to SystemVerilog*. arXiv:2411.12983, 2024. https://arxiv.org/abs/2411.12983
[R17] Jason Blocklove et al. *Can EDA Tool Feedback Improve Verilog Generation by LLMs?* arXiv:2411.11856, 2024. https://arxiv.org/abs/2411.11856
