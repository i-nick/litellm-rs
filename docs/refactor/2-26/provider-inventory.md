# Provider Inventory（P0 交付）

> 日期：2026-02-28
> 范围：`src/core/providers/**`
> 目的：给 `provider-optimization-plan.md` 提供当前可执行基线（实现路径、覆盖状态、测试护栏）

---

## 1) 当前实现路径总览

### 1.1 Tier 1（catalog-first）

- 单一入口：`src/core/providers/registry/catalog.rs`
- 运行时实例：`Provider::OpenAILike(openai_like::OpenAILikeProvider)`
- 工厂优先级：`create_provider()` 先查 catalog，再走 `from_config_async()`
  - 参考：`src/core/providers/mod.rs:694-710`

### 1.2 Tier 2/3（手写/特例）

- 当前 `from_config_async()` 直接分支支持：
  - `OpenAI`
  - `Anthropic`
  - `Mistral`
  - `Cloudflare`
  - `OpenAICompatible`
  - 参考：`src/core/providers/mod.rs:775-885`

### 1.3 Provider Enum 统一出口

- 当前统一枚举：
  - `OpenAI` / `Anthropic` / `Azure` / `Bedrock` / `Mistral` / `MetaLlama` / `VertexAI` / `V0` / `AzureAI` / `Cloudflare` / `OpenAILike`
  - 参考：`src/core/providers/mod.rs:443-456`

---

## 2) Catalog 现状统计

> 基于 `src/core/providers/registry/catalog.rs` 统计

- Tier 1 总条目：**51**
  - `def(...)`（云端 Bearer）：43
  - `def_local(...)`（本地无 key）：8
- 代表性已 catalog 化 provider：
  - `groq`, `xai`, `openrouter`, `deepseek`, `moonshot`, `ovhcloud`, `aiml_api`, `anyscale` 等

---

## 3) 基础设施收敛快照（P1 对应）

- Header 构造统一类型：`HeaderPair`
- HTTP 错误统一 mapper：`HttpErrorMapper`
- OpenAI-compatible 主路径：`openai_like::OpenAILikeProvider`
- 宏化骨架（仍用于部分 provider）：`define_pooled_http_provider_with_hooks!`

---

## 4) 已落地护栏测试（P0 对应）

### 4.1 单测（providers 模块）

- `test_provider_type_from_display_consistency`
- `test_provider_type_all_variants_covered`
- `test_create_provider_tier1_catalog_creates_openai_like`
- `test_b1_first_batch_create_provider_from_name`
- `test_b2_second_batch_create_provider_from_name`
- `test_b3_third_batch_create_provider_from_name`

### 4.2 集成测试

- `tests/integration/provider_factory_tests.rs`
- 关键验证：`create_provider()` 可创建 catalog provider，并返回 `Provider::OpenAILike(_)`

---

## 5) 本次验证命令与结果（2026-02-28）

- `cargo test test_provider_type_from_display_consistency --lib` ✅
- `cargo test test_b1_first_batch_create_provider_from_name --lib` ✅
- `cargo test test_b2_second_batch_create_provider_from_name --lib` ✅
- `cargo test test_b3_third_batch_create_provider_from_name --lib` ✅
- `cargo test test_create_provider_tier1_catalog_creates_openai_like --lib` ✅
- `cargo test --test lib integration::provider_factory_tests` ✅（14 passed）

---

## 6) 结论（对应 P0 验收）

- ✅ 已可回答“某 provider 走哪条路径”（catalog-first / from_config_async / enum 变体）
- ✅ 基线护栏测试可运行并通过
- ✅ P0 交付物已补齐（本文件 + migration tracker）
