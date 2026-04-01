use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::llm::LlmUsageSnapshot;

/// `WorkflowProgressEvent` 是 workflow 对外可观测的最小进度事实。
/// transport 只负责把它编码成协议，不在这里附带宿主语义。
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct WorkflowProgressEvent {
    /// 当前正在执行的 workflow action。
    pub action: String,
    /// 稳定阶段名，供 Agent 和测试做精确断言。
    pub phase: String,
    /// 当前阶段的简洁描述。
    pub message: String,
    /// 从当前 workflow 开始到本事件的累计耗时。
    pub elapsed_ms: u64,
    /// 已完成的工作量；未知时显式为 `None`。
    pub processed: Option<usize>,
    /// 当前阶段总工作量；未知时显式为 `None`。
    pub total: Option<usize>,
    /// 当前累计的 LLM usage 快照。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<LlmUsageSnapshot>,
}

/// workflow 只依赖 `ProgressSink` 这层抽象，不直接触碰 stdout。
pub trait ProgressSink {
    fn report(&mut self, event: WorkflowProgressEvent);
}

/// 默认 no-op sink 用于兼容旧的非流式执行路径。
#[derive(Default)]
pub struct NoopProgressSink;

impl ProgressSink for NoopProgressSink {
    fn report(&mut self, _event: WorkflowProgressEvent) {}
}

/// `SharedProgressSink` 让同一条 workflow 能把普通阶段和实时 usage 事件写到同一个下游。
pub struct SharedProgressSink<'a> {
    inner: Rc<RefCell<&'a mut dyn ProgressSink>>,
}

impl<'a> SharedProgressSink<'a> {
    pub fn new(inner: Rc<RefCell<&'a mut dyn ProgressSink>>) -> Self {
        Self { inner }
    }
}

impl ProgressSink for SharedProgressSink<'_> {
    fn report(&mut self, event: WorkflowProgressEvent) {
        self.inner.borrow_mut().report(event);
    }
}

/// `WorkflowReporter` 统一负责稳定阶段命名和耗时计算。
pub struct WorkflowReporter<'a> {
    action: &'static str,
    started_at: Instant,
    sink: &'a mut dyn ProgressSink,
}

impl<'a> WorkflowReporter<'a> {
    /// 为一个新的 workflow 执行构造 reporter。
    ///
    /// # 参数
    /// - `action`：当前 workflow 的稳定动作名。
    /// - `sink`：接收进度事件的下游。
    ///
    /// # 返回
    /// - 返回从当前时刻开始累计耗时的 reporter。
    pub fn new(action: &'static str, sink: &'a mut dyn ProgressSink) -> Self {
        Self {
            action,
            started_at: Instant::now(),
            sink,
        }
    }

    /// 基于已有起始时间构造 reporter。
    ///
    /// # 参数
    /// - `action`：当前 workflow 的稳定动作名。
    /// - `sink`：接收进度事件的下游。
    /// - `started_at`：workflow 的真实起始时间。
    ///
    /// # 返回
    /// - 返回沿用既有耗时起点的 reporter。
    pub fn from_started_at(
        action: &'static str,
        sink: &'a mut dyn ProgressSink,
        started_at: Instant,
    ) -> Self {
        Self {
            action,
            started_at,
            sink,
        }
    }

    /// 在阶段边界上报不带计数的事件。
    ///
    /// # 参数
    /// - `phase`：稳定阶段名。
    /// - `message`：当前阶段的简洁说明。
    pub fn phase(&mut self, phase: &str, message: impl Into<String>) {
        self.emit(phase, message.into(), None, None, None);
    }

    /// 在可计数阶段上报当前完成度。
    ///
    /// # 参数
    /// - `phase`：稳定阶段名。
    /// - `message`：当前阶段的简洁说明。
    /// - `processed`：已完成工作量。
    /// - `total`：当前阶段总工作量。
    pub fn counted(
        &mut self,
        phase: &str,
        message: impl Into<String>,
        processed: usize,
        total: usize,
    ) {
        self.emit(phase, message.into(), Some(processed), Some(total), None);
    }

    /// 输出带 usage 快照的进度事件。
    pub fn usage(&mut self, phase: &str, message: impl Into<String>, usage: LlmUsageSnapshot) {
        self.emit(phase, message.into(), None, None, Some(usage));
    }

    fn emit(
        &mut self,
        phase: &str,
        message: String,
        processed: Option<usize>,
        total: Option<usize>,
        usage: Option<LlmUsageSnapshot>,
    ) {
        self.sink.report(WorkflowProgressEvent {
            action: self.action.to_string(),
            phase: phase.to_string(),
            message,
            elapsed_ms: self.started_at.elapsed().as_millis() as u64,
            processed,
            total,
            usage,
        });
    }
}
