# Provider Migration Tracker Template

> 用途：按批次跟踪 Provider 收敛进度（配合 `provider-optimization-plan.md`）
> 日期：2026-02-26

---

## 1) 批次总览

| Batch | 范围 | 目标 | 状态 | 负责人 | PR |
|---|---|---|---|---|---|
| B1 | Tier 1（首批 8-15 个） | 统一宏路径 + 错误映射 | TODO |  |  |
| B2 | Tier 1（次批 8-15 个） | 统一配置 + 流式处理 | TODO |  |  |
| B3 | Tier 2（hooks） | 宏 + patch hooks | TODO |  |  |
| B4 | Tier 3（特例） | 手写实现规范化 | TODO |  |  |
| B5 | 收尾 | 删除兼容层 + CI 守卫 | TODO |  |  |

---

## 1.1) B1 候选 Provider（低风险高收益）

> 选择原则：优先 `registry/catalog` 中的 OpenAI-compatible、Bearer 认证、协议差异小、可复用统一宏路径。

| 候选 | 来源分组 | 选择理由 | 预估风险 |
|---|---|---|---|
| aiml_api | Group 1a | 标准 OpenAI-compatible，便于模板化 | 低 |
| aleph_alpha | Group 1a | 认证与端点模式标准 | 低 |
| anyscale | Group 1a | 典型云端 Bearer 路径 | 低 |
| bytez | Group 1a | 与统一请求转换高度一致 | 低 |
| comet_api | Group 1a | 配置/鉴权简单 | 低 |
| compactifai | Group 1a | 可直接套用统一错误映射 | 低 |
| maritalk | Group 1a | OpenAI-compatible，改造收益明显 | 低 |
| siliconflow | Group 1a | 历史上常见重复实现，收敛价值高 | 低 |
| yi | Group 1a | 协议简单，适合首批验证 | 低 |
| lambda_ai | Group 1a | 便于验证 catalog→factory 一致性 | 低 |
| ovhcloud | Group 1a | 标准 Bearer + base_url 模式 | 低 |
| lemonade | Group 1e | 作为跨分组样本验证泛化能力 | 低~中 |

### B1 目标（建议）
- 统一到“宏 + 默认错误映射 + 统一配置映射”主路径。
- 本批完成后验证：catalog / factory / dispatch 列表一致。
- 不触碰 Anthropic / Bedrock / Vertex 等重协议特例。

### B1 验收附加项
- [ ] 上述 12 个 provider 可通过统一工厂创建
- [ ] 12 个 provider 的错误映射全部走默认 mapper（有差异者仅 patch）
- [ ] 12 个 provider 至少完成基础 chat 回归（非流式）

---

### B1 执行顺序（按改动最小优先）

> 目标：先打通模板与流水线，再逐步扩展覆盖，避免首批就引入高耦合变更。

#### PR-B1-1（模板打通，4个）
1. `aiml_api`
2. `anyscale`
3. `bytez`
4. `comet_api`

验收重点：
- 统一宏路径可稳定创建 provider
- 默认错误映射与基础 chat 回归通过
- catalog/factory/dispatch 三处覆盖对齐

#### PR-B1-2（扩展覆盖，4个）
5. `compactifai`
6. `aleph_alpha`
7. `yi`
8. `lambda_ai`

验收重点：
- 配置映射一致（BaseConfig + provider-specific）
- 继续复用默认 mapper，避免新增自定义分叉
- 与 PR-B1-1 合并后全量回归保持通过

#### PR-B1-3（跨分组验证，4个）
9. `ovhcloud`
10. `maritalk`
11. `siliconflow`
12. `lemonade`（跨分组样本）

验收重点：
- 验证统一路径在不同分组下仍成立
- 检查是否存在隐藏差异（endpoint/header/body 字段）
- 形成 B2 输入清单（哪些 provider 需要 hooks）

### B1 每个 PR 的固定检查
- [ ] `cargo check --all-features`
- [ ] `cargo test --all-features`
- [ ] 本 PR provider 清单可通过工厂创建
- [ ] 本 PR provider 基础 chat 回归通过
- [ ] 未新增重复 `error.rs` / 重复配置 schema

---

## 2) Provider 级跟踪表

> 状态建议：`TODO` / `IN_PROGRESS` / `DONE` / `BLOCKED`

| Provider | Tier | 当前实现模式 | 目标实现模式 | Config Canonical | Error Canonical | Streaming Canonical | Registry/Factory 对齐 | 测试通过 | 状态 | 备注 |
|---|---|---|---|---|---|---|---|---|---|---|
| openai | 1 | 手写 | 统一（保留手写/或宏） | ☐ | ☐ | ☐ | ☐ | ☐ | TODO |  |
| anthropic | 3 | 手写 | 手写规范化 | ☐ | ☐ | ☐ | ☐ | ☐ | TODO | 非 OpenAI 协议 |
| azure | 2 | 混合 | 宏+hooks / 手写规范化 | ☐ | ☐ | ☐ | ☐ | ☐ | TODO |  |
| bedrock | 3 | 手写 | 手写规范化 | ☐ | ☐ | ☐ | ☐ | ☐ | TODO | SigV4 |
| groq | 1 | 混合 | 宏或统一骨架 | ☐ | ☐ | ☐ | ☐ | ☐ | TODO |  |

> 复制上面行继续扩展全部 provider。

---

## 3) 每批执行清单（Checklist）

### Batch 准入条件
- [ ] 已确认本批 provider 列表（8~15 个）
- [ ] 已有基线测试（chat/stream/error）
- [ ] 已确认不会跨批修改公共 schema（若需要，先独立 PR）

### Batch 内实施
- [ ] 统一配置映射（对齐 canonical schema）
- [ ] 统一错误映射（默认 mapper + provider patch）
- [ ] 统一流式处理（可用时走 base/sse）
- [ ] 对齐 registry/factory/dispatch 覆盖
- [ ] 清理无价值 `error.rs`/重复 glue 代码

### Batch 验收
- [ ] `cargo check --all-features` 通过
- [ ] `cargo test --all-features` 通过
- [ ] 本批 provider 回归测试通过
- [ ] 迁移台账更新为 DONE

---

## 4) 风险记录

| 日期 | Provider/批次 | 风险描述 | 影响 | 缓解措施 | 状态 |
|---|---|---|---|---|---|
|  |  |  |  |  | OPEN |

---

## 5) 决策日志（ADR-lite）

| 日期 | 决策 | 备选方案 | 结论 | 关联 PR |
|---|---|---|---|---|
|  |  |  |  |  |

---

## 6) CI 守卫项（最终收尾使用）

- [ ] catalog/factory/dispatch provider 列表一致性检查
- [ ] 禁止新增重复 ProviderType/ProviderConfig schema（脚本或 lint）
- [ ] 新 provider 必须声明 Tier 和实现模式
- [ ] 新 provider 必须含错误映射与流式策略说明

---

## 7) 快速填报模板（复制即用）

```markdown
### Batch Bx - <name>
- Providers: <p1, p2, ...>
- Goal: <本批目标>
- PR: <link>
- Result:
  - check: PASS/FAIL
  - test: PASS/FAIL
  - coverage notes: <可选>
- Follow-up:
  - <下一批或阻塞项>
```
