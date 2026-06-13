# 面向模块自动连线与 LLM 协同设计的新式硬件描述语言研究报告

## 执行摘要

本报告围绕“是否应设计一种以模块为核心、便于大语言模型自动完成模块间连线的新式 HDL，用以替代或补充 RTL/Verilog”这一问题，给出面向学术与工程评审的系统论证。核心结论是：**应当推进，但不宜以“全面替代 Verilog”作为短期目标，而应以“模块接口、连接、契约、时序域”四个要素为核心，先构建一个可降级到保守 Verilog/SystemVerilog 子集的上层语言或前端**。现有 RTL/SystemVerilog 已经通过 interfaces、implicit ports 等机制部分缓解连线负担，但其语言历史包袱、设计/仿真语义混杂、工具支持不均，以及顶层互连的文本噪声，仍然使其对 LLM 不够友好。近年来 Chisel、Amaranth、SpinalHDL、Bluespec、Clash、Veryl、Spade、Anvil、Calyx/CIRCT 等语言或 IR 已分别证明：**接口抽象、类型系统、时序域、契约式验证、可降级后端**都是可行方向。与此同时，Verilog 生成基准表明，LLM 在较真实的 RTL 生成、调试与验证任务上仍远未饱和，且静态基准存在数据污染与高估风险，因此“减少连线熵、提高中间表示可检查性”本身就是更稳妥的研究方向。综合理论、工程、验证、产业采纳等维度，本报告对“独立新语言 + 降级后端”的总体可行性评分为 **7.4/10**：**值得立项，但建议采用“接口优先、互连优先、验证优先”的渐进式路线**，而不是一步到位重写整个数字设计生态。 citeturn22view4turn22view0turn16view2turn21view3turn24view0turn24view3turn15view0turn15view2turn25view3turn39view6

## 研究目标与问题陈述

现有 Verilog/SystemVerilog 的根本优势在于与 EDA 工具链深度耦合、工程生态成熟，但其顶层或中层集成代码往往充满显式端口声明、宽度/方向对齐、时钟复位传递、握手信号散布与命名约定依赖。SystemVerilog interfaces 的出现，本身就说明“连接”在 RTL 工程里是需要专门语言构造来封装的问题；Doulos 的教程将接口定义为“专门用于封装模块间通信的构造”，Oracle Labs 在 DVCon 经验中甚至报告 **约 25% 的代码行只用于互连，端口连接可减少最多 90%**。Chisel 官方文档也明确指出，接口与 bulk connect 的价值正在于显著减少 wiring，并将大接口的修改局部化。换言之，**连线复杂性不是错觉，而是被既有社区反复识别并持续修补的问题**。 citeturn22view4turn22view0turn16view2turn14search19

从 LLM 视角看，RTL/Verilog 不友好，主要不在于“语法难”，而在于它把太多**低层、分散、跨文件、跨层次、跨时序域**的信息暴露在最终文本中。CIRCT 文档指出，生成“被大量工具接受的、语法正确且风格良好”的 Verilog/SystemVerilog 本身就很难，原因包括语言特性丰富、工具支持差异大、lint 规范复杂；Veryl 论文则进一步强调，SystemVerilog 混合了逻辑设计与仿真相关构造，复杂性抬高了工具完整支持难度，也限制了可用性。对于 LLM，这直接转化为三个困难：其一，**上下文分散**，模型必须同时记忆局部逻辑与远处连接约束；其二，**错误局部不可判定**，很多连线问题只有 elaboration、lint、仿真或形式验证后才暴露；其三，**语义噪声过高**，最终文本中有大量对人类和工具必要、但对功能理解价值不高的“胶水代码”。 citeturn23view3turn21view3turn14search9

硬件版 LLM 基准进一步说明，问题不是“模型还不够大”那么简单。VerilogEval 以 156 个 HDLBits 任务为起点，但更接近真实工作流的新基准已经明显更难：一个 2025–2026 的更大规模 RTL 设计/验证基准报告称，最强模型在代码生成任务上 **pass@1 不超过 34%**；DATE 2026 的 LiveVerilogEval 又指出，传统静态基准易受训练集泄漏影响，在自动生成的、抗污染的新基准上，多种 SOTA 模型都出现显著性能下降。DeepRTL 也直接指出，现有方法偏重 Verilog 生成，却忽略同样关键的 Verilog 理解，因此自然语言到 RTL 的对齐仍然较弱。**这意味着，与其继续让模型直接在原始 Verilog 上“硬写顶层互连”，不如先改变语言表示，把模块、接口、连接和契约抽成更低熵、更可检、更可组合的中间设计对象。** citeturn15view0turn15view2turn25view3turn25view4turn27view3

因此，本报告对研究目标作如下严格表述：**设计一种面向通用数字电路与 SoC 模块互连的模块中心型 HDL 前端，使设计者与 LLM 优先编写“模块接口、时序域、连接拓扑、契约与适配器”，而不是逐信号胶水连线；再由编译器将其确定性降低为可综合 RTL/SystemVerilog。** 它的短期定位是“替代顶层/子系统级互连描述层”，中期定位是“补充手写 RTL datpath/control），长期才可能竞争更广义的 RTL 前端。这个定位既符合 SystemC/TLM、Chisel/FIRRTL、Calyx/CIRCT 等工具链显示出的“上层抽象 + 下层 RTL 降级”实践，也能规避 HLS 仅擅长函数/算法内核而未直接解决大规模模块互连规范化的问题。 citeturn16view0turn16view1turn16view9turn16view8turn39view6

## 背景与相关工作综述

过去十余年，硬件语言研究已经从“是否要高于 RTL”转向“**以何种抽象高于 RTL，同时不失去可综合性、时序可控性与验证能力**”。SNAPL 2019 将此趋势概括为“硬件描述语言的黄金时代”，并明确把编程语言技术、硬件 DSL、编译器与形式化方法视为提高硬件生产率的关键结合点。围绕你的设想，现有工作大致可分为四类：系统级/HLS、宿主语言嵌入式 RTL DSL、规则/消息传递型 HDL、以及新一代独立现代 HDL/IR。 citeturn21view0

| 体系 | 代表 | 核心抽象 | 连接/接口机制 | 对本课题的启发 | 主要局限 |
|---|---|---|---|---|---|
| 传统 RTL 基线 | Verilog/SystemVerilog | 事件驱动 + RTL + 验证混合 | ports、interfaces、implicit ports | 工具生态最成熟；interfaces 证明“连接值得被封装” | 语言包袱重、工具支持不均、顶层胶水噪声高。citeturn22view4turn22view0turn14search9turn23view3 |
| 系统级/HLS | SystemC/TLM、Vitis HLS | 系统级模型、函数/任务、事务级 | LT/AT socket、函数参数映射端口、stream/task | 证明高抽象与 RTL 降级可行，且仿真可显著快于 pin-accurate RTL | 更偏系统建模或算法核，未直接把“模块互连语言化”为一等公民。citeturn16view0turn16view1turn39view6 |
| 嵌入式 RTL DSL | Chisel、PyRTL、PyMTL3、Amaranth、SpinalHDL | 在 Scala/Python 中构造硬件 | Bundle/Decoupled、Signature/connect、Stream/Flow | 证明“强接口 + 生成式编程 + 降到 Verilog”非常有效；尤其 Chisel/Amaranth 在接口与 bulk connect 上最接近你的目标 | 依赖宿主语言生态与 elaboration 心智模型；对 LLM 而言，仓库级上下文负担可能更大。citeturn16view2turn23view0turn16view4turn16view5turn16view7turn16view6turn6search5turn17search12turn36search20 |
| 函数式/规则式 HDL | Clash、Bluespec | 类型化信号域、规则/方法、原子动作 | `Signal dom a`、接口方法、规则调度 | 提供强类型时序域与形式化友好的并发模型，是“时序契约”设计的重要参考 | Haskell/规则调度门槛高，成本模型与团队学习曲线较陡。citeturn17search0turn17search16turn31view3turn18search0turn18search17turn18search21 |
| 编译器 IR / 生成基础设施 | FIRRTL、Calyx、CIRCT、ESI、Handshake | IR 分层、结构+控制、通道/服务/验证方言 | HW module、ESI channels、Handshake FIFO、Verif/LTL | 极适合做新语言后端：模块、通道、契约、验证与 SV 导出都已有基础设施 | 主要是 IR，不直接解决“人类/LLM 前端的语言设计”问题。citeturn23view1turn39view4turn16view9turn34view0turn34view1turn34view2turn34view3turn39view5turn13search12 |
| 现代独立 HDL | Veryl、Spade、Anvil | 现代语法、强类型、时序安全/流水线 | interface、clock/reset 抽象、pipelines、timing contracts | 最接近“重新设计 HDL 表面语言”的方向，说明产业与学界都在重新发明 HDL | 新生态尚小，采用成本与兼容路径仍在形成期。citeturn21view3turn24view4turn21view1turn24view3turn24view0turn20search3 |

如果把这些路径放在一起看，可以得到一个非常重要的判断：**没有任何一个现有体系同时把“模块互连自动化”“时序域显式化”“契约验证”“LLM 友好输出”“工业 RTL 后端兼容”五件事情做到位**。SystemC/HLS 主要解决抽象层次；Chisel/Amaranth/Spinal 解决了一部分接口与生成；Bluespec/Clash/Anvil/Spade 解决更强的时序与类型安全；CIRCT/ESI/Verif/LTL 则提供极强的后端与验证支撑。你的设想真正的创新点，不在“又做一门 HDL”，而在于**把这些已经被验证有效的思想，围绕“模块连接”和“LLM 分步生成”重组为一个更窄、更可落地的前端语言目标。** citeturn16view2turn16view5turn18search0turn24view0turn34view0turn34view3turn39view5

在 LLM 方向，现有成果同样给出清晰信号。VerilogEval 证明了可以用仿真输出对 RTL 生成进行自动函数正确性评估；RTLCoder/OpenLLM-RTL 展示了开放数据集、领域微调与代码质量反馈对性能的重要性，并凸显了本地部署与数据隐私在芯片设计中的现实意义；ChipNeMo 则表明，行业场景中定制 tokenizer、继续预训练、SFT 与专用检索模型可以显著提升芯片设计任务效果；AutoChip/EDA-feedback 路线说明，把编译器、仿真器和 EDA 日志纳入迭代闭环比单轮生成更贴近真实设计流程；VRank 进一步说明，多候选自一致性排序可提升功能正确率。总体上，这一波研究几乎一致地指向同一件事：**给 LLM 一个结构良好、反馈闭环明确、局部可修复的设计表示，往往比硬逼它直接写最终 Verilog 更有效。** citeturn15view0turn27view2turn21view5turn15view4turn38search0turn26search2

## 技术要求与设计原则

基于上述相关工作，本报告建议把新语言的验收标准定义为“**八项硬属性 + 一项 LLM 协同属性**”。这些属性不应停留在愿景层面，而应对应明确的编译期检查与基准指标。现有语言和工具已经分别证明：接口签名、域类型、结构化 IR、契约化验证与可降级后端，是足以落地的技术支点。 citeturn16view2turn16view5turn31view3turn24view0turn34view1turn34view3turn39view5

| 属性 | 严格定义 | 最低要求 | 建议量化指标 |
|---|---|---|---|
| 模块接口抽象 | 模块边界不得默认暴露为原始散乱位信号；边界必须首先是**具名接口类型** | 新设计中 ≥80% 跨模块边界使用接口类型而非裸 ports | **IAC**：Interface Abstraction Coverage = 接口化边界数 / 全部边界数 |
| 类型系统 | 编译期检查方向、位宽、枚举、参数、接口角色、时钟域、复位极性、延迟兼容性 | 宽度/方向/域不匹配必须在 elaboration 前报错 | **TSC**：Type Safety Coverage = 可静态检查约束项 / 总约束项 |
| 时序/同步语义 | 时钟域、复位、流水级、握手协议、跨域适配都必须显式化 | 任一 CDC/RDC 连接都必须通过显式 adapter 或 contract | **TEI**：Temporal Explicitness Index = 显式标注的时序边界 / 全部时序边界 |
| 可组合性 | 模块组合由签名与契约驱动，而非命名偶合 | 相同签名接口可直接组合；多重匹配必须报歧义，不可静默猜测 | **CSR**：Composition Success Rate = 自动组合成功样例 / 总组合样例 |
| 可验证性 | 规格断言可自动降级为 SVA/LTL/形式约束 | 每个接口类型至少附带一条 safety 与一条 liveness 模板 | **FVS**：Formal Verifiability Score = 自动生成并成功检查的性质数 / 总性质数 |
| 可综合性 | 语言核心语义必须可确定性降级到综合可接受的 RTL | 默认后端支持保守 Verilog-2005/SystemVerilog 子集 | **SPR**：Synthesis Pass Rate = 综合通过设计数 / 全部设计数 |
| 可测试性 | 自动生成仿真/协同仿真测试夹具与覆盖钩子 | 每个模块都可自动生成最小 smoke test 与接口约束测试 | **TY**：Testability Yield = 自动测试通过率；覆盖达标率 |
| 可解释性/可追溯 | 每条连接、每个 adapter、每个端口展开都可追踪回源语句 | 降级 RTL 需保留源位置信息与接口来源 | **TS**：Traceability Score = 可回溯对象数 / 总对象数 |
| LLM 友好性 | 设计任务可被拆解为接口、模块、拓扑、契约、修复五类局部子任务 | 模型默认输出 AST/JSON/受约束 DSL，而非直接自由文本 Verilog | **LLM-Pass@1**、**Type-Pass@1**、**Compose-Pass@1**、平均修复轮数、平均上下文 token |

其中最关键的不是“接口抽象”本身，而是**接口抽象必须伴随时序与契约抽象**。仅把若干信号打成 bundle，最多只能降低样板代码；若不把“这个 bundle 在什么域中有效、何时可以消费、是否允许零拍组合穿透、跨域是否需要同步器、是否允许背压”等语义一并上升为类型或契约，那么编译器和 LLM 最终仍要回到手工揣测。Clash 的 synthesis domain、Anvil 的 timing safety、Spade 的 first-class pipelines、Bluespec 的规则/方法语义，以及 CIRCT 的 Verif/LTL 方言都指向同一设计原则：**对硬件而言，空间与时间必须同时进入类型系统或契约系统。** citeturn31view3turn24view0turn24view3turn18search0turn18search17turn39view5turn34view3

与此相配套，语言还应坚持三个审稿级原则。第一，**保守默认**：对任何可能引入时序歧义、CDC 风险或多重匹配歧义的自动连线，一律默认拒绝，而不是猜。第二，**双后端策略**：一条路径输出保守 Verilog-2005/朴素 SystemVerilog 以求工具兼容，另一条路径输出 richer 的 interface/ESI/SVA 版本以利验证与调试；CIRCT 已明确指出，不同工具对特性的支持差异是后端设计的现实约束。第三，**先图后文**：优先让编译器和 LLM 在“模块图/接口图/约束 AST”上协作，再 pretty-print 为源码，因为 grammar-constrained decoding 已被证明能显著提高结构输出可靠性。 citeturn23view3turn28view0turn29search2

## 语言设计草案

本报告建议采用一个**窄而强**的语言核心，而不是一开始覆盖 Verilog 的全部表达能力。临时命名可用 **MC2-HDL**，意为 *Module–Contract–Connectivity HDL*。它的核心对象只有五种：`interface`、`module`、`adapter`、`compose`、`contract`。其中 `module` 负责局部实现，`interface` 规定边界类型与协议，`compose` 只负责拓扑连接，`adapter` 专门承载宽度变化、协议转换、CDC/RDC、节拍插入等“非直连”情况，`contract` 负责把可验证语义绑定到接口与模块。这样的切分和 Calyx 的“结构 + 控制分离”、CIRCT/ESI 的模块/通道建模非常一致，也比让 LLM 直接在最终 Verilog 中同时处理实现与胶水更适合局部化生成。 citeturn16view9turn34view0turn34view1

下面给出一个最小草案。其目标不是展示语法美感，而是展示“**接口为一等公民、连接为显式语句、时序域为类型属性、歧义必须显式消解**”这四点。

```text
clockdom Sys(clk: clock, rst: reset, reset_kind: sync, active: high)

interface Stream<T> @Sys {
    role producer {
        payload : T
        valid   : Bool
    }
    role consumer {
        ready   : Bool
    }

    contract {
        safety  : valid -> stable(payload) until ready
        event   : fire := valid & ready
    }
}

module Producer(out tx: Stream<u32>) @Sys {
    state cnt: u32 = 0
    logic {
        tx.payload = cnt
        tx.valid   = 1
        when tx.fire { cnt = cnt + 1 }
    }
}

module Consumer(in rx: Stream<u32>) @Sys {
    logic {
        rx.ready = 1
        // consume rx.payload on rx.fire
    }
}

compose Top @Sys {
    inst p = Producer()
    inst q = Fifo<u32>(depth = 4)
    inst c = Consumer()

    connect p.tx -> q.in
    connect q.out -> c.rx
}
```

与之对比，下面是 Verilog/SystemVerilog 风格的典型写法，它并不“错误”，但把大部分篇幅都花在端口与胶水上：

```verilog
module Top(
  input  logic        clk,
  input  logic        rst
);
  logic [31:0] p_tx_payload;
  logic        p_tx_valid;
  logic        p_tx_ready;

  logic [31:0] q_out_payload;
  logic        q_out_valid;
  logic        q_out_ready;

  Producer u_p(
    .clk(clk), .rst(rst),
    .tx_payload(p_tx_payload),
    .tx_valid(p_tx_valid),
    .tx_ready(p_tx_ready)
  );

  Fifo #(.DEPTH(4)) u_q(
    .clk(clk), .rst(rst),
    .in_payload(p_tx_payload),
    .in_valid(p_tx_valid),
    .in_ready(p_tx_ready),
    .out_payload(q_out_payload),
    .out_valid(q_out_valid),
    .out_ready(q_out_ready)
  );

  Consumer u_c(
    .clk(clk), .rst(rst),
    .rx_payload(q_out_payload),
    .rx_valid(q_out_valid),
    .rx_ready(q_out_ready)
  );
endmodule
```

这个对比并不是要否定 Verilog，而是强调一个设计事实：**在模块组合这件事上，Verilog 的主工作单元是“信号”，而 MC2-HDL 的主工作单元应是“接口实例”与“连接关系”**。SystemVerilog interfaces、Chisel Bundle/Decoupled、Amaranth Signature/connect 都已经证明，把边界提升为接口对象能显著减少 wiring；你的语言应当把这一思想再推进一步——不仅“封装端口”，还要“封装协议、时序域与验证契约”。 citeturn22view4turn16view2turn16view5

在语义上，本报告建议采用以下四条硬规则。其一，`connect A -> B` 仅能连接**互补角色**的同签名接口；其二，若存在多个潜在接收方，编译器返回 `AmbiguousConnectError`，要求设计者增加作用域或显式 `route`; 其三，跨时钟/复位域的连接禁止隐式通过，必须写成 `adapt A -> SyncFIFO -> B` 或等价适配器；其四，所有自动生成的 adapter 都必须出现在 elaboration report 中并可在 RTL 中追踪。这样做的目的是把“自动连线”从“猜测式魔法”变成“受限搜索 + 静态证明 + 可解释降级”。这一点非常重要，因为 Anvil、Clash、Spade 都说明：**时序与域兼容不应依靠设计者和模型暗自记忆，而应受语言与编译器约束。** citeturn24view0turn31view3turn24view3

围绕 LLM，本报告不建议让模型直接输出最终 DSL 文本，更不建议直接输出最终 Verilog；建议采用“**可见中间工件链**”策略：先输出接口 schema，再输出模块骨架，再输出连接图，再输出契约与测试模板，最后才生成 DSL 文本与降级 RTL。Plan-and-Solve prompting 说明，先规划再求解可以减少缺步错误；Self-Refine 表明，初稿—反馈—修正的迭代明显优于单轮输出；Grammar-Constrained / Grammar-Aligned Decoding 说明，受语法约束的生成能稳定保证结构合法；RepoCoder 与检索增强代码生成实证工作则表明，仓库级上下文最好通过检索与局部拼接提供，而不是整仓库硬塞进上下文窗口。对硬件设计来说，这意味着应把链式推理显式外化为**设计计划、接口图、修复补丁**这些可审计对象，而不是依赖不可检验的自由文本思维过程。 citeturn28view1turn28view2turn28view0turn29search2turn30search7turn30search4

```mermaid
flowchart LR
    A[自然语言需求/现有模块库] --> B[任务分解<br/>接口→模块→拓扑→契约]
    B --> C[受约束 AST / JSON]
    C --> D[语法与类型检查]
    D --> E[生成 MC2-HDL 源码]
    E --> F[Lower 到 CIRCT/HW/ESI/Verif/LTL]
    F --> G[导出 Verilog/SystemVerilog]
    G --> H[lint/仿真/形式验证/综合]
    H --> I[结构化诊断]
    I --> B
```

## 工具链、实现路线与实验建议

从工程角度看，最可行的路线不是自建完整后端，而是**前端自研、后端复用 CIRCT 与现有开源 EDA**。推荐的编译流是：MC2-HDL 前端完成解析、名字解析、参数展开、接口匹配、时序域检查、适配器插入与 contract elaboration；然后降低到 CIRCT 的 `hw`/`esi`/`verif`/`ltl`/必要时 `firrtl` 方言；最后使用 CIRCT 的 Verilog/SystemVerilog emission 输出目标 RTL。CIRCT 已提供 `hw.module`、ESI channel/service、Verif、LTL 等关键构件，且其目标之一正是把前端作者从“如何生成可接受 Verilog”这一复杂问题中隔离出来。 citeturn34view1turn34view0turn34view3turn39view5turn23view3

与现有 EDA 集成时，建议把“保守兼容”放在首位：生成后的 RTL 先经 `slang` 或 `Surelog/UHDM` 进行编译/静态分析/elaboration，再经 Verilator 做 lint、可选断言与覆盖插桩，再进入 Yosys/SymbiYosys 的综合、等价与形式验证流。`slang` 明确支持解析、类型检查和 elaboration；Surelog 目标是完整 SystemVerilog 2017 前端，并已在多个开源核上走通综合与硬件运行；Verilator 可做 lint、断言与覆盖；Yosys/SBY 则能提供综合、BMC、prove、cover 与等价检查。由此，MC2-HDL 完全可以不试图取代仿真器、综合器或形式工具，而是成为这些工具之前的**结构化设计入口层**。 citeturn15view5turn39view2turn39view1turn39view3turn15view7turn32search17turn32search1

在验证层面，建议把接口契约一部分直接降成 SVA 风格性质，一部分降成 CIRCT 的 `verif`/`ltl` IR。CIRCT 的 Verif dialect 已支持 assert/assume/cover、logic equivalence checking、contracts；LTL dialect 的目标是抽取 SystemVerilog Assertions 背后的核心时间逻辑，并为 Verilog 输出与形式工具提供基础。对于以模块和互连为中心的语言，这意味着你可以天然支持三类验证：**接口协议正确性、适配器保真性、与基线 RTL 的行为等价性**。此外，若设计目标包含多时钟域 SoC 互连，后续还应考虑生成符合 Accellera CDC/RDC 语义的 collateral；Accellera 在 2026 年刚发布了 IP 层 CDC/RDC 集成抽象标准，这对你的语言是非常现实的产业对接点。 citeturn34view3turn39view5turn31view0

在自动化测试/仿真层面，建议双轨并行。第一轨是硬件传统流：随机/定向 testbench、接口断言、cover 收敛。第二轨是软件化协同仿真流：用 cocotb 生成 Python 驱动的白盒测试，重用 Python 生态做参考模型、数据生成、差分比对与性能脚本。cocotb 的优势在于验证生产率、白盒访问与 Python 包生态；对于你这类“语言与工具链共演化”的研究，自动化评测基础设施比单一语言语法更重要。 citeturn39view0turn39view1

为了降低首次立项风险，本报告建议采用“三阶段实现路线”。第一阶段只做**互连层 DSL**：要求已有 leaf RTL 模块，只重新描述接口、拓扑、契约与 adapter，编译到 RTL wrapper/top；第二阶段扩展到**模块体 DSL**：允许写简单状态机与流水线控制，但 datapath 仍可嵌入现有 Verilog/Chisel/Veryl；第三阶段才尝试更完整的独立 HDL。这个顺序的优点在于：第一阶段就能直接验证“LLM 自动连线是否显著优于直接写 Verilog 顶层”，且完全保留现有 IP。考虑到 HLS、SpinalHDL、Anvil、Veryl 都选择“编译到可读 RTL 并保留互操作性”，这是更符合工程现实的切入方式。 citeturn39view6turn16view6turn24view0turn21view3

建议的里程碑与时间估计如下。

| 阶段 | 时间估计 | 主要交付 | 验收标准 |
|---|---:|---|---|
| 需求冻结与形式语义草案 | 1–2 个月 | 语言核心语法、接口模型、错误模型、评价指标 | 形成公开 spec v0.1；接口与时序语义可被测试化 |
| 前端与互连原型 | 2–4 个月 | parser/elaborator、接口匹配、歧义报错、adapter 插入 | 能把 20–30 个组合/握手/CDC 小样例降级到可仿真 RTL |
| CIRCT 后端与验证原型 | 4–6 个月 | HW/ESI/Verif/LTL lowering、SVA 生成、Verilator/Yosys 接入 | lint 通过率、综合通过率、形式验证通过率达到预设阈值 |
| LLM 协同生成链 | 6–8 个月 | AST 约束生成、检索、EDA 反馈、自修复循环 | 在自建互连任务集上显著优于直接生成 Verilog 顶层 |
| 基准、对比与论文化 | 8–12 个月 | ModuleComposeBench、对比实验、开源实现、论文初稿 | 形成可复现脚本与 CCF-A 级投稿材料 |

实验设计上，建议至少包含四类基准。其一，**已有 RTL 生成基准**：VerilogEval、RTLLM 2.0、LiveVerilogEval，主要检验“是否更利于 LLM 生成正确实现”；其二，**接口/拓扑组合基准**：从公开模块中抽取 leaf 模块，只让模型负责接口匹配与 compose；其三，**适配器基准**：宽度调整、背压协议转换、CDC/RDC、节拍插入；其四，**验证基准**：自动生成的接口性质、等价检查与 cocotb 测试。特别建议你新建一个 **ModuleComposeBench**：选择一批公开设计中的 leaf module，剥离手写顶层，为每个任务给出模块接口图与目标拓扑，让模型只完成“接口定义 + compose + adapter + contract”。这比直接在 VerilogEval 上刷 pass@1 更能真实检验“新语言是否解决了连线痛点”。已有研究已证明，仓库级上下文、局部检索、多候选排序与 EDA 反馈都很重要，因此评测必须覆盖单轮生成与闭环修复两种模式。 citeturn15view0turn21view5turn15view2turn25view3turn38search0turn26search2turn30search7turn30search4

## 可行性评估

综合现有证据，**“设计一门模块中心、接口契约驱动、可供 LLM 自动连线的 HDL 前端”在理论上完全可行，在工程上具有中高可行性，在产业上则需要谨慎的渐进式采纳策略**。原因是：接口抽象已在 SystemVerilog、Chisel、Amaranth、SpinalHDL 中被证明有效；强类型与时序安全在 Clash、Bluespec、Spade、Anvil 中得到证明；结构化 lowering 与可读 RTL 导出在 FIRRTL/CIRCT/Calyx/Veryl/Anvil 中已有现成路径；LLM 侧则已有检索、约束解码、EDA 反馈、自一致性排序等成熟拼装件。换言之，真正缺的不是“单项技术”，而是**面向模块连接这一窄问题的一体化设计**。 citeturn16view2turn16view5turn16view6turn31view3turn18search0turn24view0turn21view1turn21view3turn34view0turn34view3turn38search0turn26search2turn28view0

下面给出本报告的量化评分。分值不是“是否存在论文”的简单函数，而是结合可实现性、外部依赖与产业阻力后的综合判断。

| 维度 | 评分 | 评分依据 |
|---|---:|---|
| 理论可行性 | 8.5 | 接口、域类型、契约、结构化 IR、形式验证方言都已有成熟先例；不存在已知理论障碍。citeturn16view2turn31view3turn34view3turn39view5 |
| 工程可行性 | 7.5 | 可大量复用 CIRCT、slang、Surelog、Verilator、Yosys/SBY；关键工作在前端与 lowering 规则。citeturn15view5turn39view2turn39view1turn39view3turn15view7 |
| 性能与 QoR 风险 | 6.5 | 若 auto-wiring 仅做语义保真展开，额外开销可控；但自动插入 adapter/buffer/CDC 单元可能伤及 PPA。现代 HDL 已展示“零/低开销抽象”可能性，但不能默认成立。citeturn16view6turn24view0turn24view3turn21view1 |
| 验证可行性 | 8.0 | 接口契约可系统化降为 SVA/LTL/Verif；等价、BMC、cover、协同仿真路径明确。citeturn34view3turn39view5turn32search17turn15view7turn39view0 |
| LLM 适配收益 | 8.0 | 现有基准显示直接 Verilog 生成仍难；结构约束、EDA 反馈、局部检索与自一致性已知有效。citeturn15view2turn25view3turn38search0turn26search2turn30search4turn28view0 |
| 安全与 IP 治理 | 6.5 | 本地模型/开源模型能减轻隐私风险，但训练污染、提示注入、恶意逻辑与第三方模型合规性仍需制度化控制。citeturn27view2turn15view4turn25view4 |
| 可维护性 | 8.5 | 若坚持“互连层优先、后端降级、源位置信息保留”，维护成本优于直接维护大量顶层胶水代码。citeturn23view1turn23view3turn22view4 |
| 产业采纳 | 5.5 | 工具与团队习惯强绑定 Verilog/SystemVerilog；最可行路径是先替代顶层互连层，而不是一次性取代全 RTL 设计流。citeturn23view3turn21view3turn39view6 |

| 方案 | 核心思路 | 综合评分 | 优点 | 缺点 | 建议 |
|---|---|---:|---|---|---|
| 原方案 | **独立 MC2-HDL 前端**，模块/接口/契约/compose 一等公民，降级到 Verilog/SystemVerilog | 7.4 | 学术创新最强；最适合做 LLM 协同；语义最干净 | 语言设计、生态建设与 IDE 成本最高 | **推荐作为主线** |
| 替代方案 | **在 Chisel/Amaranth/Veryl/SV 上增加 interface schema + auto-wiring 插件** | 8.0 | 工程见效快；更易融入现有团队；可先做实验验证价值 | 受宿主语言限制，难形成统一前端语义 | **推荐作为保守落地线** |
| 优化方案 | **先做图式 IR / 连接编排器，而非完整新语言** | 8.2 | 最快验证“连线自动化”是否有价值；可直接服务 IDE/Agent | 研究创新点偏工具，不足以彻底改善前端表达 | **适合作为最小可行产品** |
| 激进方案 | **走更强的 timing-safe / message-passing 语言路线**，类似 Anvil/Spade/ESI/Bluespec 思想融合 | 6.8 | 安全性、时序显式性最强；长期上限高 | 学习曲线和产业阻力最大；短期不利于采纳 | **不建议作为第一阶段主线** |

因此，如果从“是否值得做”回答，是 **值得**；如果从“是否应完全替代 Verilog/RTL”回答，则是 **短中期不建议**。更准确的策略是：**把它定位为一种面向模块互连、接口契约和 LLM 协同的“上层 RTL 结构语言”**。这样既承认 Verilog 作为最终交付格式的现实地位，也能把最痛、最适合语言创新的一层先剥离出来。 citeturn22view4turn16view2turn24view0turn23view3

## 结论与建议

最终结论是：**推荐推进，但应以“补充并局部替代 RTL/Verilog”而非“短期完全替代”作为项目目标；优先级为高。** 就学术价值而言，这个方向踩中了当前 HDL 研究与 LLM-for-hardware 两个热点的交叉点，而且问题足够具体：不是泛泛地“让 LLM 写硬件”，而是让它在一个**模块图、接口签名、时序域与契约可静态检查**的语言里完成最耗人工、最不稳定的互连部分。就工程价值而言，顶层/子系统级胶水代码长期是缺乏审美也缺乏鲁棒性的痛点，而现有工具链与现代 HDL 已足以支撑一个可落地的前端试验。就产业路径而言，最现实的入口是 **top-level / subsystem composition、NoC/bus/stream 互连、wrapper/adapter 自动生成、CDC 边界显式化**，而不是立刻挑战手写 datapath 与全部验证生态。 citeturn22view0turn22view4turn16view2turn31view0turn15view2

建议的下一步行动顺序也应当非常明确。第一，先冻结一个极小语言核心：`interface`、`compose`、`adapter`、`contract`、`clockdom`；第二，只服务“已有 leaf RTL 的集成场景”，不急于支持复杂模块体；第三，建立 ModuleComposeBench，把你的语言与 Verilog 顶层、Chisel/Amaranth/Veryl 风格顶层做**人工工作量、LLM pass@1、lint/综合/形式通过率、PPA 偏差**四维对比；第四，在闭环中强制使用“AST 约束生成 + 类型检查 + EDA 反馈 + 局部修复”，绝不批准“自由文本一步到位写最终 Verilog”的研发路径。这样做，既能满足 CCF-A 级论文对**清晰问题定义、创新点收敛、可复现实验、与强基线对比**的要求，也能最大化减少工程试错。 citeturn28view0turn28view1turn38search0turn26search2turn39view3turn39view1

如果必须给出一句最简洁的项目判断，那就是：**不要先发明“更高层的算法 HDL”，而要先发明“更低熵、更可解释、更易被 LLM 正确连接的模块互连 HDL”**。这条路线比全面重做 RTL 语言更窄，却更有可能在一年内做出可发表、可开源、可验证、可被团队试用的成果。关键参考来源包括 IEEE/SystemVerilog 与 Accellera/SystemC/CDC 标准资料、DAC/ASPLOS/PLDI/ICCAD/DATE 等论文与 CIRCT/CHIPS Alliance 官方文档，以及 VerilogEval、RTLLM、LiveVerilogEval、RTLCoder、DeepRTL、ChipNeMo、AutoChip 等近年的硬件-LMM 原始研究。 citeturn14search9turn16view0turn31view0turn8search8turn16view9turn18search17turn15view0turn15view3turn27view2turn27view3turn15view4turn38search0

开放问题与局限性也需明确：其一，本文提出的指标体系与语言草案属于**研究性规范**，尚需通过真实 benchmark 校准；其二，跨时钟域、低功耗/时钟门控、复杂总线协议（AXI/CHI 等）上的契约设计，尚未在本报告中展开到工业细则；其三，LLM 结果具有显著模型依赖与时间依赖，基准也在快速演化，尤其污染问题不容忽视；其四，若未来目标转向完整替代 RTL，则需进一步处理模拟/混合信号、DFT、物理约束、IP 交付格式等超出本报告主战场的议题。就当前范围而言，这些不是阻止立项的理由，但它们决定了项目必须以**渐进式、可降级、强验证**为研究纪律。 citeturn25view4turn15view2turn31view0turn23view3