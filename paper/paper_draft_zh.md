# MICO: 面向 LLM 协同 RTL 集成的契约驱动模块互连语言与 Rust 编译器框架

**工作标题**：MICO: Contract-Guided Module Composition for LLM-Assisted RTL Integration
**短名**：MICO-HDL / MICO-Connect
**目标会议类型**：DAC / ICCAD / ASPLOS / PLDI/OOPSLA 取向的 CCF-A 系统、EDA、PL 论文
**版本**：research scaffold v0.1

---

## 摘要

大语言模型已经能生成小规模 Verilog 片段，但在真实 RTL 集成中仍容易受到端口命名、方向、位宽、握手协议、时钟/复位域、CDC/RDC、wrapper 和 adapter 选择等因素影响。本文提出 **MICO**，一种面向模块互连的契约驱动硬件前端语言与 Rust 编译器框架。MICO 的核心思想不是让 LLM 直接输出最终 Verilog，而是把 LLM 的输出限制为接口 schema、模块实例图、连接拓扑、adapter plan 与 contract skeleton；随后由编译器执行名字解析、接口兼容性检查、协议/时钟域检查、adapter 合法性验证，并生成可追溯的 SystemVerilog wrapper、SVA 断言以及面向 CIRCT HW/ESI/Verif/LTL 的中间表示。

与传统 HDL 相比，MICO 把设计主单元从单根 wire/port 提升为接口、契约和连接图；与 Chisel/Amaranth 等已有接口机制相比，MICO 的核心问题不是通用 RTL 生成，而是 **LLM 可控的模块级互连综合**；与 CPPL 等近期 LLM 友好电路前端相比，MICO 进一步把协议契约、时钟域、自动 adapter 合成和模块组合 benchmark 作为主贡献。本文给出 MICO 的问题定义、语言核心、静态语义、Rust 工具链设计、评测计划和 CCF-A 可行性分析，并提出一个新的评测集 **ModuleComposeBench**，专门衡量 LLM 在已有 RTL/IP 模块集成任务上的正确性、可修复性和工程收益。

---

## 1. 引言

现有 RTL 工程中，很多 top-level/subsystem 代码并不表达核心功能，而是表达大量模块之间的胶水连接。例如，一个 ready/valid stream 在 SystemVerilog 中可能展开为 `data`、`valid`、`ready`、可选 `last`、`strb`、`id` 等多根信号；一旦进入 AXI、TileLink、NoC、CDC、低功耗或多复位域场景，连接代码会迅速膨胀。对人类工程师而言，这些代码可读性差、修改成本高、容易漏连；对 LLM 而言，它们更糟，因为模型需要在长上下文中同时追踪远距离依赖和隐式协议约束。

传统 Verilog/SystemVerilog 的主工作单元是信号；但真实设计意图常以模块、接口、时序域、协议和系统拓扑表达。SystemVerilog interface、Chisel Bundle/bulk connect、Amaranth `Signature/connect()`、CIRCT ESI channel 都已经说明：接口抽象能够减少连接噪声，且具备工程价值。Chisel 文档明确指出接口类能促进复用、支持 producer/consumer 之间的 bulk connection 并显著减少 wiring；Amaranth wiring 文档也把接口对象声明、方向翻转、连接和可复用 component 作为标准库能力。MICO 的出发点是：这些能力仍然没有围绕 **LLM 驱动的模块互连** 形成一个专门的、可检查的研究对象。

本文提出的核心判断是：

> LLM 不应被要求直接生成最终 RTL；它应只生成候选的模块连接图和契约草案，所有正确性由编译器、仿真器、形式工具和综合工具验证。

这个判断直接回应了近年 LLM-for-RTL 的实证趋势。VerilogEval、RTLLM、OpenLLM-RTL、RTLCoder 等工作表明，RTL 生成可以通过仿真和 benchmark 自动评估，但直接 Verilog 生成仍有明显正确性和可泛化问题。ChipNeMo 显示领域适配、专用 tokenizer、继续预训练和检索增强对芯片设计任务有效；CPPL 则进一步说明，将 LLM 输出转化为可静态检查的前端 DSL/JSON IR 并降低到 CIRCT，比自由文本 RTL 更可靠。MICO 接受这一方向，但选择一个更窄且更可发表的切口：**模块互连、adapter 合成与契约验证**。

---

## 2. 动机：连接熵与 LLM 失败模式

### 2.1 连接熵

定义一个简单指标：

```text
Connection Entropy CE = primitive_signal_edges / semantic_interface_edges
```

`primitive_signal_edges` 是最终 RTL 中逐根 wire/port 的连接数量；`semantic_interface_edges` 是设计者真正关心的接口级连接数量。例如：

```text
producer.tx -> fifo.in -> consumer.rx
```

这可能只有 2 条语义连接，但在 RTL 中展开为十几到几十根 primitive signal edge。CE 越高，LLM 越容易犯以下错误：

- 端口漏连；
- ready/valid 方向反接；
- payload 宽度错配；
- reset 极性或同步/异步语义错；
- CDC 连接没有同步器；
- 参数化实例没有统一；
- wrapper 命名碰撞；
- 生成的代码可仿真但不可综合；
- SVA/testbench 覆盖不到实际协议错误。

### 2.2 为什么不是直接改进 prompt

单纯 prompt engineering 能提升结果，但无法从根本上消除不受约束文本生成的问题。若目标是 CCF-A 论文，必须把问题从“如何提示 LLM”提升到“如何定义一个可验证的中间语言与工具链”。MICO 因此采用以下原则：

1. **LLM 只提出候选**：模型输出 design graph、adapter plan、contract skeleton。
2. **编译器拒绝不安全连接**：方向、位宽、协议、时钟域不匹配必须报错。
3. **跨域连接默认禁止**：CDC/RDC 必须通过显式 adapter。
4. **adapter 需可解释**：自动插入的 buffer、pipeline、width adapter、CDC FIFO 必须出现在 elaboration report。
5. **验证 collateral 自动生成**：接口 contract 降级为 SVA/LTL/Verif 断言或约束。
6. **最终 RTL 是交付格式，不是 LLM 的主要生成目标**。

---

## 3. 相关工作与差异

### 3.1 SystemVerilog interface、Chisel、Amaranth

SystemVerilog interface 早已说明“模块间通信封装”是 HDL 的语言级需求。Chisel 更进一步提供 Bundle、Decoupled、bulk connect 与 interface reuse。Amaranth 的 wiring 库显式支持 `Signature`、`In/Out`、`connect()` 和 component 声明，使组件的端口方向和连接关系可被工具可靠处理。

MICO 与这些工作的主要差异不是“有没有接口”，而是：

- MICO 的核心目标是 **已有模块的互连与集成**，不是完整硬件描述；
- MICO 把 LLM 输出限制为 **结构化 AST/JSON**，而不是自由写宿主语言；
- MICO 将 **时钟/复位域、协议契约、adapter 合成、形式 collateral** 作为接口类型系统的一部分；
- MICO 提出专门的 **ModuleComposeBench**，评估互连任务而不是单模块生成。

### 3.2 CIRCT、ESI、Verif/LTL

CIRCT 的 HW dialect 已提供与 Verilog module 对齐的 IR；ESI 提供 typed channel、FIFO、valid/ready wrapping、buffer 和 service 等机制；Verif/LTL 则可承载 assert/assume/cover 与时间逻辑性质。因此，MICO 不应自建后端，而应采用 Rust 前端 + CIRCT lowering 的路线。Rust 负责解析、类型检查、诊断、incremental repair 和 artifact 管理；CIRCT 负责后端表示、优化和 Verilog/SystemVerilog emission。

### 3.3 CPPL 与 LLM 友好硬件前端

CPPL 是最接近的近期工作：它把 LLM 硬件生成从自由 RTL 转为 Python DSL、JSON IR、静态检查和 CIRCT lowering。若 MICO 只说“结构化语言 + CIRCT”，会被 CPPL 覆盖。因此 MICO 的论文边界必须更精确：

| 维度 | CPPL | MICO |
|---|---|---|
| 目标 | LLM 友好电路生成前端 | LLM 友好模块互连与集成前端 |
| 中心对象 | module/interface/hierarchy/operation IR | interface/clockdom/contract/adapter/compose graph |
| 主要正确性 | IR 合法性、width、port binding | 方向、协议、时钟域、CDC、adapter correctness、contract refinement |
| Benchmark | RTLLM 等代码生成任务 | ModuleComposeBench：已有 IP/RTL 模块互连任务 |
| 论文卖点 | 编译器介入 LLM 电路生成 | 编译器验证 LLM 提出的互连图与 adapter plan |

### 3.4 Anvil 与 AutoSVA

Anvil 说明时序安全可以作为 HDL 类型系统问题处理，尤其是跨模块稳定性与 timing contract。AutoSVA 说明模块交互中的 liveness/safety 性质可以自动生成形式验证 testbench。MICO 可把二者作为契约层支点：接口不是只有字段，还应绑定稳定性、握手、延迟和活性要求。

---

## 4. MICO 语言核心

MICO v0 的核心对象只有五种：

```text
clockdom    时钟/复位域
interface   具名协议接口，包含 role、field、contract
extern module 已有 RTL/IP 模块的接口声明
adapter     宽度、协议、CDC、pipeline、buffer 等显式适配器
compose     实例化与接口级连接图
```

一个最小例子：

```mico
clockdom Sys(clk, rst);

interface StreamU32 @Sys {
  producer payload:u32, valid:bool;
  consumer ready:bool;
  contract stable_payload: valid -> stable(payload) until ready;
  contract fire: valid & ready;
}

extern module Producer @Sys {
  out tx: StreamU32;
}

extern module Fifo @Sys {
  in  input: StreamU32;
  out output: StreamU32;
}

extern module Consumer @Sys {
  in rx: StreamU32;
}

compose Top @Sys {
  inst p: Producer;
  inst f: Fifo;
  inst c: Consumer;

  connect p.tx -> f.input;
  connect f.output -> c.rx;
}
```

MICO 的语义不是“凭名字猜连接”，而是：

```text
connect A.x -> B.y is legal iff:
1. A and B are existing instances;
2. x and y are existing ports;
3. port(x).direction = out and port(y).direction = in;
4. interface(x) = interface(y) or an explicit adapter exists;
5. clockdom(x) = clockdom(y) or an explicit CDC adapter exists;
6. all bound contracts of interface(x) refine or satisfy interface(y)'s assumptions;
7. lowering has a deterministic mapping to RTL/SVA artifacts.
```

若任何条件失败，编译器必须拒绝，并产生结构化诊断。诊断应可直接喂给 LLM 作为 repair prompt：

```json
{
  "error": "ClockDomainMismatch",
  "connection": "dma.m_axis -> aes.s_axis",
  "from_domain": "Aclk",
  "to_domain": "Bclk",
  "repair_hints": [
    "insert adapter AsyncFifo<StreamU32>",
    "or prove both ports are in the same clock domain"
  ]
}
```

---

## 5. Rust 工具链设计

Rust 是适合 MICO 的实现语言，原因如下：

- 编译器前端对 AST/IR 所有权、错误恢复、并行 pass、可测试性要求高；
- Rust 的 enum/trait/pattern matching 适合表达 typed IR、diagnostic、pass pipeline；
- Rust 可较自然地生成 CLI、LSP server、WASM playground、Python binding；
- 硬件 EDA 工具链已有越来越多 Rust 基础设施，便于后续生态接入。

推荐 workspace 划分：

```text
mico_ir        AST/IR、类型、诊断、语义检查
mico_frontend  lexer/parser、source map、error recovery
mico_codegen   SystemVerilog / JSON IR / future CIRCT emission
mico_cli       check、dump-ir、emit-sv、doctor、bench 命令
```

第一阶段不需要完整 MLIR binding。推荐先输出三类 artifact：

1. `*.sv`：保守 SystemVerilog wrapper/top；
2. `*.mico.json`：结构化 design graph，供 LLM repair 和 benchmark 使用；
3. `*.sva.sv`：接口 contract 派生断言。

第二阶段再接入 CIRCT：

```text
MICO AST
  -> MICO typed IR
  -> CIRCT HW module / extern
  -> ESI channels for stream protocols
  -> Verif/LTL for assertions
  -> SystemVerilog emission
```

---

## 6. LLM 协同流程

MICO 的 LLM 协同流程应为五步，而不是一次性生成代码：

```text
Step 1: module inventory
  从已有 RTL 或声明文件抽取模块、端口、参数、协议候选。

Step 2: interface schema proposal
  LLM 将低层 ports 归并为 Stream/Bus/Register/Interrupt/ClockReset 等接口。

Step 3: compose graph proposal
  LLM 输出实例图、连接图、必要 adapter plan。

Step 4: compiler validation
  Rust compiler 检查名字、类型、方向、domain、contract、adapter 合法性。

Step 5: repair loop
  结构化诊断返回 LLM；LLM 只修复局部 AST patch。
```

关键研究点是 **LLM 的输出空间设计**。建议默认用 JSON AST 而非自由文本：

```json
{
  "compose": "Top",
  "instances": [
    {"name": "p", "module": "Producer"},
    {"name": "f", "module": "Fifo"},
    {"name": "c", "module": "Consumer"}
  ],
  "connections": [
    {"from": "p.tx", "to": "f.input"},
    {"from": "f.output", "to": "c.rx"}
  ],
  "adapters": []
}
```

这使得错误可以定位到结构节点，而不是一段不可控 Verilog 文本。

---

## 7. Adapter 合成

MICO 不应把所有 adapter 都“自动魔法化”。建议区分三类：

| Adapter 类别 | 是否可自动插入 | 例子 | 编译器义务 |
|---|---:|---|---|
| 纯结构型 | 可以 | port rename、bundle flatten、constant tie-off | 证明字段一一映射 |
| 局部时序型 | 条件允许 | skid buffer、pipeline stage、valid-only to valid-ready | 报告新增 latency/backpressure 语义 |
| 跨域/协议型 | 默认要求显式确认 | CDC FIFO、AXI-Lite to APB、width converter | 生成 SVA/CDC collateral，记录 PPA 风险 |

研究创新点可以落在 **contract-guided adapter synthesis**：

```text
Given source interface S with guarantee GS,
and target interface T with assumption AT,
find adapter A such that:
  GS ; A |= AT
and A satisfies domain, latency, width, reset constraints.
```

MICO v0 可先不做完整自动搜索，只实现 adapter library + legality checker：

```mico
adapter AsyncFifo32 from StreamU32@Aclk to StreamU32@Bclk {
  kind cdc_fifo;
  depth 4;
  contract preserves_order;
  contract no_drop_if_not_full;
}

compose Top @Aclk {
  inst dma: Dma @Aclk;
  inst aes: Aes @Bclk;
  adapt dma.tx -> AsyncFifo32 -> aes.rx;
}
```

---

## 8. 实验设计：ModuleComposeBench

现有 Verilog 生成 benchmark 多聚焦单模块 RTL 或自然语言到 Verilog。MICO 需要一个新 benchmark：**ModuleComposeBench**。

### 8.1 任务构造

每个 task 包含：

```text
leaf RTL modules/blackbox declarations
自然语言集成需求
目标接口协议说明
golden compose graph
golden wrapper/top
testbench / formal properties
optional PPA reference
```

### 8.2 难度层级

| Level | 任务类型 | 例子 |
|---|---|---|
| L1 | 同域同协议直连 | stream producer -> fifo -> consumer |
| L2 | 宽度/参数适配 | u32 stream -> u64 packer |
| L3 | backpressure/latency adapter | valid-only -> valid-ready, skid buffer |
| L4 | CDC/RDC | async FIFO, reset synchronizer |
| L5 | bus bridge | AXI-Lite -> APB, register block wrapper |
| L6 | multi-IP subsystem | DMA + SRAM + accelerator + interrupt controller |

### 8.3 指标

| 指标 | 含义 |
|---|---|
| Compose-Pass@1 | 第一次生成的 compose graph 是否通过编译器检查 |
| Repair-Turns | 通过检查所需平均修复轮数 |
| Lint-Pass | 生成 SystemVerilog 是否通过 lint/elaboration |
| Sim-Pass | 是否通过 cocotb/Verilator 仿真 |
| Formal-Pass | 接口 SVA/性质是否通过 |
| Adapter-Correct | adapter 是否满足约束且没有隐式 CDC |
| QoR-Delta | 面积、时序、buffer 数、latency 相对手写基线差异 |
| Human-Fix-Minutes | 人类修复成本 |
| Connection-Entropy-Reduction | primitive edge 与 semantic edge 比值下降 |

### 8.4 Baselines

必须和强基线比较：

1. Direct Verilog prompting；
2. SystemVerilog interface prompting；
3. Chisel/Amaranth style prompting；
4. CPPL-style JSON IR prompting；
5. MICO text prompting；
6. MICO JSON AST prompting；
7. MICO JSON AST + compiler feedback repair；
8. Human-written wrapper/top。

---

## 9. CCF-A 论文创新点

推荐把 contribution 写成四条：

1. **问题定义**：首次系统定义 LLM-assisted RTL module composition，把模块连接从 free-form RTL generation 中剥离为 typed graph synthesis 问题。
2. **语言与类型系统**：提出 interface/clockdom/contract/adapter/compose 五对象核心语言，并定义连接合法性、跨域拒绝、adapter 显式化和 contract refinement。
3. **Rust 编译器与 LLM repair loop**：实现一个编译器验证的 LLM 协同系统，输出 SystemVerilog/SVA/CIRCT-ready IR，并提供结构化诊断反馈。
4. **Benchmark 与实证**：提出 ModuleComposeBench，在真实/半真实 RTL/IP 集成任务上证明 MICO 相比 direct Verilog/CPPL-style/Chisel-style prompting 提升 correctness、repair efficiency 和 maintainability。

最强利益点：

> MICO 把 LLM 硬件生成从“让模型写正确 RTL”转为“让模型提出可验证连接意图”，使复杂连线问题变成编译器可拒绝、可修复、可追踪的模块互连综合问题。

---

## 10. 可行性评分

| 维度 | 原始 idea | MICO 优化后 | 说明 |
|---|---:|---:|---|
| 创新性 | 6.5 | 8.4 | 关键在互连、契约、adapter、benchmark，而非又一门 HDL |
| 技术深度 | 6.8 | 8.2 | 需形式化连接合法性、contract refinement、CDC/adapters |
| 工程可实现性 | 7.2 | 8.0 | Rust frontend + conservative SV backend 可先落地 |
| 实验可验证性 | 6.5 | 8.3 | ModuleComposeBench 能形成清晰实证 |
| 与现有工作的区分度 | 5.8 | 8.0 | 必须明确避开 CPPL、Chisel、Amaranth 的已有覆盖 |
| 产业价值 | 7.0 | 8.5 | top-level/subsystem glue code 是真实痛点 |
| CCF-A 潜力 | 6.2 | 8.1 | 取决于 artifact、benchmark、强基线和形式化程度 |

**综合评分：8.1/10。** 这是有潜力的 CCF-A 方向，但前提是实现和评测必须真实扎实，不能只停留在语言语法设计。

---

## 11. 局限与风险

1. 若只实现漂亮语法，没有强 checker，会被认为是工程包装。
2. 若只对 toy examples 有效，无法说服 EDA/系统审稿人。
3. 若没有 CPPL/Chisel/Amaranth 等强基线，会被质疑新颖性。
4. 若 adapter 自动插入影响 PPA，必须透明报告 QoR delta。
5. 若 LLM 只是“锦上添花”，论文贡献会变成普通 DSL；必须通过实验展示 LLM 在 MICO 表示下更稳定。
6. 若过早追求完整 HDL，会扩大范围并稀释互连主线。

---

## 12. 结论

MICO 的最佳研究路线不是“用 Rust 重新实现 Verilog”，也不是“写一个方便 LLM 的玩具语言”。它应当是一个窄而硬的模块互连前端：

```text
existing RTL/IP modules
  -> interface/contract extraction
  -> LLM-proposed compose graph and adapter plan
  -> Rust compiler type/domain/protocol/contract checking
  -> SystemVerilog/SVA/CIRCT lowering
  -> lint/sim/formal/synthesis feedback
```

只要该路线能在 ModuleComposeBench 上显著降低连接错误、修复轮数和人类胶水代码量，同时保持可综合性和可验证性，就有较强的 CCF-A 投稿价值。
