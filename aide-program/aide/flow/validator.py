"""流程校验：环节合法性与跳转规则。"""

from __future__ import annotations

from aide.flow.errors import FlowError


class FlowValidator:
    def __init__(self, phases: list[str]):
        self.phases = _normalize_phases(phases)

    def validate_phase_exists(self, phase: str) -> None:
        if phase not in self.phases:
            raise FlowError(f"未知环节: {phase}（请检查 flow.phases 配置）")

    def validate_start(self, phase: str) -> None:
        self.validate_phase_exists(phase)

    def validate_next_part(self, from_phase: str, to_phase: str) -> None:
        self.validate_phase_exists(from_phase)
        self.validate_phase_exists(to_phase)
        from_index = self.phases.index(from_phase)
        to_index = self.phases.index(to_phase)
        if to_index != from_index + 1:
            raise FlowError(
                f"非法跳转: {from_phase} -> {to_phase}（next-part 只能前进到相邻环节）"
            )

    def validate_back_part(self, from_phase: str, to_phase: str) -> None:
        self.validate_phase_exists(from_phase)
        self.validate_phase_exists(to_phase)
        from_index = self.phases.index(from_phase)
        to_index = self.phases.index(to_phase)
        if to_index >= from_index:
            raise FlowError(
                f"非法回退: {from_phase} -> {to_phase}（back-part 只能回退到之前环节）"
            )


def _normalize_phases(phases: list[str]) -> list[str]:
    if not isinstance(phases, list) or not phases:
        raise FlowError("flow.phases 配置无效：必须为非空列表")
    normalized: list[str] = []
    seen: set[str] = set()
    for item in phases:
        if not isinstance(item, str) or not item.strip():
            raise FlowError("flow.phases 配置无效：环节名必须为非空字符串")
        name = item.strip()
        if name in seen:
            raise FlowError(f"flow.phases 配置无效：环节名重复 {name!r}")
        seen.add(name)
        normalized.append(name)
    return normalized

