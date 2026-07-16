/**
 * 核心场景验收的机器可读 authority。
 *
 * 本模块只定义稳定场景边界和 acceptance plan，不运行 Runtime、读取 Wiki 或聚合 gate。
 * 行为证据由 Rust/workspace tests 提供，gate decision 由 `quality-gates.mjs` 负责。
 */

import {
  FORMAL_QUALITY_GATES,
  QUALITY_GATE_REGISTRY,
} from "./quality-gates.mjs";

/** 场景矩阵结构与语义的版本；它不表示产品 release 或 artifact 版本。 */
export const CORE_SCENARIO_CONTRACT_VERSION = "core-scenario-acceptance.v1";

/** 当前允许写入 matrix 的支持等级闭集。 */
export const CORE_SCENARIO_SUPPORT_LEVELS = Object.freeze([
  "supported",
  "degraded",
]);

function requireStringArray(value, field, { allowEmpty = false } = {}) {
  if (!Array.isArray(value)
    || (!allowEmpty && value.length === 0)
    || value.some(item => typeof item !== "string" || item.trim() === "")) {
    throw new TypeError(`${field} must be ${allowEmpty ? "a" : "a non-empty"} string array`);
  }
  return Object.freeze([...value]);
}

function requireThresholds(value) {
  if (value === undefined)
    return Object.freeze({});
  if (!value || typeof value !== "object" || Array.isArray(value))
    throw new TypeError("acceptance plan thresholds must be an object");
  const entries = Object.entries(value);
  for (const [key, threshold] of entries) {
    if (key.trim() === ""
      || (typeof threshold !== "boolean"
        && (typeof threshold !== "number" || !Number.isFinite(threshold)))) {
      throw new TypeError(`invalid acceptance plan threshold: ${key}`);
    }
  }
  return Object.freeze(Object.fromEntries(entries));
}

/**
 * 创建不可变的验收计划；所有 primary fixtures 和 required gates 必须由调用方显式提供。
 *
 * @param {Record<string, unknown>} input 验收范围和执行策略。
 * @returns {Readonly<Record<string, unknown>>} 经过闭集校验的不可变计划。
 */
export function createAcceptancePlan(input) {
  if (!input || typeof input !== "object")
    throw new TypeError("acceptance plan must be an object");
  if (typeof input.plan_id !== "string" || input.plan_id.trim() === "")
    throw new TypeError("acceptance plan requires plan_id");
  if (typeof input.report_only !== "boolean")
    throw new TypeError("acceptance plan requires boolean report_only");

  const primaryFixtures = requireStringArray(input.primary_fixtures, "primary_fixtures");
  const requiredPrimaryGates = requireStringArray(
    input.required_primary_gates ?? [],
    "required_primary_gates",
    { allowEmpty: true },
  );
  const requiredGates = requireStringArray(input.required_gates, "required_gates", { allowEmpty: true });
  const requiredGuards = requireStringArray(input.required_guards, "required_guards", { allowEmpty: true });
  for (const gate of requiredGates) {
    if (!FORMAL_QUALITY_GATES.includes(gate))
      throw new TypeError(`unknown gate: ${gate}`);
  }
  for (const gate of requiredPrimaryGates) {
    if (QUALITY_GATE_REGISTRY[gate]?.gate_level !== "primary_gate")
      throw new TypeError(`required_primary_gates must reference a primary gate: ${gate}`);
    const missingCompanions = QUALITY_GATE_REGISTRY[gate].required_companion_gates
      .filter(companion => !requiredGates.includes(companion));
    if (missingCompanions.length > 0)
      throw new TypeError(`primary gate ${gate} requires companion gates: ${missingCompanions.join(", ")}`);
  }
  for (const gate of requiredGuards) {
    if (QUALITY_GATE_REGISTRY[gate]?.gate_level !== "baseline_guard")
      throw new TypeError(`required_guards must reference a baseline guard: ${gate}`);
  }

  return Object.freeze({
    plan_id: input.plan_id,
    primary_fixtures: primaryFixtures,
    required_primary_gates: requiredPrimaryGates,
    required_gates: requiredGates,
    required_guards: requiredGuards,
    thresholds: requireThresholds(input.thresholds),
    report_only: input.report_only,
  });
}

const REQUIRED_ARRAY_FIELDS = Object.freeze([
  "public_entrypoints",
  "formal_artifacts",
  "success",
  "failure_or_degraded",
  "recovery",
  "verification_fixtures",
  "evidence_refs",
  "owning_gates",
  "deferred_capabilities",
]);

function freezeScenario(scenario) {
  return Object.freeze(Object.fromEntries(
    Object.entries(scenario).map(([key, value]) => {
      if (Array.isArray(value))
        return [key, Object.freeze([...value])];
      if (value && typeof value === "object")
        return [key, Object.freeze({ ...value })];
      return [key, value];
    }),
  ));
}

/**
 * 校验核心场景矩阵的闭集、稳定身份和必需证据字段。
 *
 * @param {Array<Record<string, unknown>>} matrix 待验证的场景记录。
 * @returns {Array<Record<string, unknown>>} 原矩阵，方便调用者在校验后继续消费。
 */
export function validateCoreScenarioMatrix(matrix) {
  if (!Array.isArray(matrix) || matrix.length !== 9)
    throw new TypeError("core scenario matrix must contain exactly 9 records");

  const seenIds = new Set();
  for (const [index, scenario] of matrix.entries()) {
    if (!scenario || typeof scenario !== "object")
      throw new TypeError(`scenario at index ${index} must be an object`);

    const expectedId = `CS-${String(index + 1).padStart(2, "0")}`;
    if (seenIds.has(scenario.scenario_id))
      throw new TypeError(`duplicate scenario_id: ${scenario.scenario_id}`);
    seenIds.add(scenario.scenario_id);
    if (scenario.scenario_id !== expectedId)
      throw new TypeError(`scenario_id must be ${expectedId}`);

    if (!CORE_SCENARIO_SUPPORT_LEVELS.includes(scenario.support_level))
      throw new TypeError(`invalid support_level for ${scenario.scenario_id}`);
    for (const field of ["title", "actor", "trigger"]) {
      if (typeof scenario[field] !== "string" || scenario[field].trim() === "")
        throw new TypeError(`${scenario.scenario_id} requires ${field}`);
    }
    if (!scenario.state_readiness || typeof scenario.state_readiness !== "object")
      throw new TypeError(`${scenario.scenario_id} requires state_readiness`);
    for (const field of REQUIRED_ARRAY_FIELDS) {
      if (!Array.isArray(scenario[field]) || scenario[field].some(item => typeof item !== "string" || item === ""))
        throw new TypeError(`${scenario.scenario_id} requires string array ${field}`);
    }
    for (const field of REQUIRED_ARRAY_FIELDS.filter(field => field !== "deferred_capabilities")) {
      if (scenario[field].length === 0)
        throw new TypeError(`${scenario.scenario_id} requires non-empty ${field}`);
    }
    if (scenario.owning_gates.some(gate =>
      QUALITY_GATE_REGISTRY[gate]?.gate_level !== "formal_quality_gate")) {
      throw new TypeError(`${scenario.scenario_id} owning_gates must reference formal quality gates`);
    }
  }

  return matrix;
}

const matrix = [
  {
    scenario_id: "CS-01",
    title: "init 后理解项目概览",
    support_level: "supported",
    actor: "首次进入仓库的用户",
    trigger: "对可扫描仓库执行 init",
    public_entrypoints: ["init", "status"],
    formal_artifacts: [".wiki/.knowledge/**", ".wiki/wiki.metadata.json", "metadata-declared pages"],
    state_readiness: { success: "fresh/ready", degraded: "runtime_incomplete or blocker with action" },
    success: ["概览、模块、入口和技术栈具有正式产物与源码证据"],
    failure_or_degraded: ["provider 或扫描阻塞不得伪装为 fresh"],
    recovery: ["按 Runtime recommended_action 执行 init/rebuild/update"],
    verification_fixtures: ["core_scenario_acceptance::canonical_core_scenarios_stay_within_public_contract"],
    evidence_refs: [
      ".wiki/06-设计文档/03-核心场景.md",
      "crates/wiki-runtime/tests/acceptance/init_generates_wiki.rs",
    ],
    owning_gates: ["artifact_validity", "status_recommended_action_stability"],
    deferred_capabilities: ["完整架构真相和用户研究"],
  },
  {
    scenario_id: "CS-02",
    title: "Agent 开工前读取项目规则",
    support_level: "degraded",
    actor: "准备修改代码的 Agent",
    trigger: "用户要求实现、修改、重构或修复",
    public_entrypoints: ["managed declared edit", "sync", "update", "query <term>"],
    formal_artifacts: ["declared knowledge records", "health signals"],
    state_readiness: { success: "knowledge ready", degraded: "conflict or stale with review/update action" },
    success: ["repo/module scoped policy 或 convention 可由 knowledge_declared_hit 命中"],
    failure_or_degraded: ["宿主必须显式 query，当前不自动匹配任务 scope"],
    recovery: ["sync/update 或 review_governance"],
    verification_fixtures: ["core_scenario_acceptance::canonical_core_scenarios_stay_within_public_contract"],
    evidence_refs: [
      "crates/wiki-runtime/tests/runtime/editable_runtime.rs",
      "crates/wiki-runtime/tests/runtime/query_sync_rebuild.rs",
    ],
    owning_gates: ["artifact_validity", "query_route_contract"],
    deferred_capabilities: ["task-aware 自动 scope 匹配", "宿主 trigger parity"],
  },
  {
    scenario_id: "CS-03",
    title: "找到正确改动候选",
    support_level: "degraded",
    actor: "准备定位改动位置的 Agent",
    trigger: "使用已知 term 查询候选源码",
    public_entrypoints: ["query <term>", "route_groups", "answer"],
    formal_artifacts: ["facts snapshot", "SQLite FTS and graph cache"],
    state_readiness: { success: "index ready", degraded: "empty success or index_not_ready/init" },
    success: ["symbol/path/module 候选携带 rank、provenance 和 source refs"],
    failure_or_degraded: ["term-only 不证明自然语言任务到真实入口"],
    recovery: ["init/rebuild 或核验 source refs"],
    verification_fixtures: ["core_scenario_acceptance::canonical_core_scenarios_stay_within_public_contract"],
    evidence_refs: [
      ".wiki/06-设计文档/06-Runtime查询合同.md",
      "crates/wiki-runtime/tests/acceptance/command_contract.rs",
    ],
    owning_gates: ["query_route_contract", "status_recommended_action_stability"],
    deferred_capabilities: ["intent-aware query", "owner projection", "独立 entrypoint projection"],
  },
  {
    scenario_id: "CS-04",
    title: "改动前查看局部影响上下文",
    support_level: "degraded",
    actor: "修改共享代码的 Agent",
    trigger: "使用已知 symbol term 查询 graph context",
    public_entrypoints: ["query <term>", "index_graph_hit", "source_refs"],
    formal_artifacts: ["graph facts", "phase diagnostics"],
    state_readiness: { success: "graph ready", degraded: "graph stale/missing with rebuild action" },
    success: ["局部 graph route 携带真实 provenance、confidence 和 refs"],
    failure_or_degraded: ["不声称完整遍历或穷尽影响"],
    recovery: ["rebuild graph 或人工核验 source refs"],
    verification_fixtures: ["core_scenario_acceptance::canonical_core_scenarios_stay_within_public_contract"],
    evidence_refs: [
      ".wiki/06-设计文档/06-Runtime查询合同.md",
      "crates/wiki-runtime/tests/runtime/query_sync_rebuild.rs",
    ],
    owning_gates: ["query_route_contract", "status_recommended_action_stability"],
    deferred_capabilities: ["callers/callees schema", "完整 impact traversal"],
  },
  {
    scenario_id: "CS-05",
    title: "代码变化后更新相关知识",
    support_level: "supported",
    actor: "完成代码变更的用户",
    trigger: "源码变化后执行 status/update",
    public_entrypoints: ["status", "update", "query <term>"],
    formal_artifacts: ["updated knowledge records", "page projections", "runtime metadata"],
    state_readiness: { success: "needs_update -> fresh", degraded: "failed update preserves prior formal snapshot" },
    success: ["只刷新关联 unit/page，未影响 identity 保持稳定"],
    failure_or_degraded: ["更新失败不得破坏上一份正式快照"],
    recovery: ["retry update or rebuild according to status"],
    verification_fixtures: ["core_scenario_acceptance::canonical_core_scenarios_stay_within_public_contract"],
    evidence_refs: [
      "crates/wiki-runtime/tests/runtime/status_and_update.rs",
      "crates/wiki-runtime/tests/runtime/progress_streaming.rs",
    ],
    owning_gates: ["artifact_validity", "status_recommended_action_stability"],
    deferred_capabilities: ["自动 PR/branch trigger policy"],
  },
  {
    scenario_id: "CS-06",
    title: "将 bug 经验沉淀为避坑记录",
    support_level: "degraded",
    actor: "完成 bug 修复的用户",
    trigger: "在 declared-owned section 写入 pitfall marker",
    public_entrypoints: ["managed declared edit", "sync", "update", "query <term>"],
    formal_artifacts: ["declared pitfall record", "projection and health artifacts"],
    state_readiness: { success: "declared active/queryable", degraded: "invalid marker rejected atomically" },
    success: ["pitfall 的 scope、现象、根因、修复和避免信息可追溯"],
    failure_or_degraded: ["非法或重复 ID 不污染旧 snapshot"],
    recovery: ["修正 marker 后重新 sync/update"],
    verification_fixtures: ["core_scenario_acceptance::declared_knowledge_and_structured_conflict_are_traceable"],
    evidence_refs: [
      "crates/wiki-model/src/domain/knowledge_artifact.rs",
      "crates/wiki-runtime/src/workflows/sync.rs",
    ],
    owning_gates: ["artifact_validity", "query_route_contract"],
    deferred_capabilities: ["专用 bug authoring UX", "自动升级为 policy"],
  },
  {
    scenario_id: "CS-07",
    title: "显式处理结构化规范冲突",
    support_level: "degraded",
    actor: "依赖 declared knowledge 的用户或 Agent",
    trigger: "parallel active declared records 发生结构化冲突",
    public_entrypoints: ["sync", "status", "query <term>"],
    formal_artifacts: ["conflict records", "health signals", "governance evidence refs"],
    state_readiness: { success: "conflict visible with review action", degraded: "query answer constrained" },
    success: ["双方 refs、scope、state 和 review action 可追溯且可清理"],
    failure_or_degraded: ["不检测任意自然语言规则与源码语义冲突"],
    recovery: ["review_governance and resolve declared records"],
    verification_fixtures: ["core_scenario_acceptance::declared_knowledge_and_structured_conflict_are_traceable"],
    evidence_refs: [
      "crates/wiki-runtime/tests/runtime/editable_runtime.rs",
      "crates/wiki-runtime/tests/runtime/status_and_update.rs",
    ],
    owning_gates: ["artifact_validity", "status_recommended_action_stability"],
    deferred_capabilities: ["自然语言规则-vs-code 语义检测"],
  },
  {
    scenario_id: "CS-08",
    title: "主动声明项目规范",
    support_level: "degraded",
    actor: "维护项目规则的用户",
    trigger: "在 declared-owned section 写入 policy/convention marker",
    public_entrypoints: ["managed declared edit", "sync", "update", "query <term>"],
    formal_artifacts: ["declared policy/convention record", "projection index"],
    state_readiness: { success: "declared active/queryable", degraded: "derived-owned edit rejected" },
    success: ["规范被原子写入 formal records 并可由用户和 Agent query"],
    failure_or_degraded: ["没有专用 authoring CLI/API"],
    recovery: ["修正 ownership/marker 后 sync/update"],
    verification_fixtures: ["core_scenario_acceptance::declared_knowledge_and_structured_conflict_are_traceable"],
    evidence_refs: [
      "crates/wiki-runtime/src/workflows/sync.rs",
      "crates/wiki-runtime/tests/runtime/editable_runtime.rs",
    ],
    owning_gates: ["artifact_validity", "query_route_contract"],
    deferred_capabilities: ["专用 authoring CLI/API", "完整 deprecated/superseded lifecycle UX"],
  },
  {
    scenario_id: "CS-09",
    title: "从已提交正式产物恢复第二工作副本",
    support_level: "supported",
    actor: "拉取已提交 Wiki 的协作用户",
    trigger: "B 有正式 Wiki 产物但无本地 cache",
    public_entrypoints: ["status", "query <term>", "update", "rebuild"],
    formal_artifacts: [".wiki/.knowledge/**", ".wiki/wiki.metadata.json", "metadata-declared pages"],
    state_readiness: { success: "restored level1", degraded: "index missing/fusion degraded with rebuild action" },
    success: ["B 不复制 cache 即可恢复 knowledge/projection query substrate"],
    failure_or_degraded: ["page/hash drift fail closed，source drift needs_update"],
    recovery: ["rebuild for index or update for source drift"],
    verification_fixtures: [
      "core_scenario_acceptance::formal_artifacts_restore_second_worktree_to_level1",
      "core_scenario_acceptance::formal_restore_rejects_page_drift_and_reports_source_drift",
    ],
    evidence_refs: [
      "crates/wiki-runtime/tests/runtime/status_and_update.rs",
      "crates/wiki-runtime/tests/runtime/query_sync_rebuild.rs",
    ],
    owning_gates: ["restore_validity", "status_recommended_action_stability"],
    deferred_capabilities: ["直接恢复 graph/index 到 ready"],
  },
];

validateCoreScenarioMatrix(matrix);
/** 已验证并冻结的 9 个核心场景 authority。 */
export const CORE_SCENARIO_ACCEPTANCE_MATRIX = Object.freeze(matrix.map(freezeScenario));
