"""decide 模块的数据结构与校验。"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from aide.decide.errors import DecideError


@dataclass(frozen=True)
class Location:
    file: str
    start: int
    end: int

    @staticmethod
    def from_dict(data: Any, path: str) -> "Location":
        if not isinstance(data, dict):
            raise DecideError(f"{path} 必须为对象")
        file = _require_str(data.get("file"), f"{path}.file")
        start = _require_int(data.get("start"), f"{path}.start")
        end = _require_int(data.get("end"), f"{path}.end")
        return Location(file=file, start=start, end=end)

    def to_dict(self) -> dict[str, Any]:
        return {"file": self.file, "start": self.start, "end": self.end}


@dataclass(frozen=True)
class Option:
    value: str
    label: str
    score: float | None = None
    pros: list[str] | None = None
    cons: list[str] | None = None

    @staticmethod
    def from_dict(data: Any, path: str, used_values: set[str]) -> "Option":
        if not isinstance(data, dict):
            raise DecideError(f"{path} 必须为对象")
        raw_value = data.get("value")
        value = _require_str(raw_value, f"{path}.value")
        if value in used_values:
            raise DecideError(f"{path}.value 在当前待定项中必须唯一，重复值: {value}")
        used_values.add(value)

        label = _require_str(data.get("label"), f"{path}.label")

        score = _optional_score(data.get("score"), f"{path}.score")
        pros = _optional_str_list(data.get("pros"), f"{path}.pros")
        cons = _optional_str_list(data.get("cons"), f"{path}.cons")
        return Option(value=value, label=label, score=score, pros=pros, cons=cons)

    def to_dict(self) -> dict[str, Any]:
        data: dict[str, Any] = {"value": self.value, "label": self.label}
        if self.score is not None:
            data["score"] = self.score
        if self.pros is not None:
            data["pros"] = self.pros
        if self.cons is not None:
            data["cons"] = self.cons
        return data


@dataclass(frozen=True)
class DecideItem:
    id: int
    title: str
    options: list[Option]
    location: Location | None = None
    context: str | None = None
    recommend: str | None = None

    @staticmethod
    def from_dict(data: Any, index: int) -> "DecideItem":
        path = f"items[{index}]"
        if not isinstance(data, dict):
            raise DecideError(f"{path} 必须为对象")
        item_id = _require_positive_int(data.get("id"), f"{path}.id")
        title = _require_str(data.get("title"), f"{path}.title")

        raw_options = data.get("options")
        if not isinstance(raw_options, list) or not raw_options:
            raise DecideError(f"{path}.options 必须为至少 1 个元素的数组")
        if len(raw_options) < 2:
            raise DecideError(f"{path}.options 至少需要 2 个选项，当前只有 {len(raw_options)} 个")
        options: list[Option] = []
        used_values: set[str] = set()
        for opt_index, raw_opt in enumerate(raw_options):
            options.append(
                Option.from_dict(raw_opt, f"{path}.options[{opt_index}]", used_values)
            )

        recommend = data.get("recommend")
        if recommend is not None:
            recommend = _require_str(recommend, f"{path}.recommend")
            if recommend not in {opt.value for opt in options}:
                raise DecideError(f'{path}.recommend 值 "{recommend}" 不在 options 中')

        location = None
        if "location" in data and data.get("location") is not None:
            location = Location.from_dict(data["location"], f"{path}.location")

        context = None
        if "context" in data and data.get("context") is not None:
            context = _require_str(data.get("context"), f"{path}.context", allow_empty=True)

        return DecideItem(
            id=item_id,
            title=title,
            options=options,
            location=location,
            context=context,
            recommend=recommend,
        )

    def to_dict(self) -> dict[str, Any]:
        data: dict[str, Any] = {
            "id": self.id,
            "title": self.title,
            "options": [opt.to_dict() for opt in self.options],
        }
        if self.location is not None:
            data["location"] = self.location.to_dict()
        if self.context is not None:
            data["context"] = self.context
        if self.recommend is not None:
            data["recommend"] = self.recommend
        return data


@dataclass(frozen=True)
class MetaInfo:
    created_at: str
    session_id: str

    @staticmethod
    def from_dict(data: Any) -> "MetaInfo":
        if not isinstance(data, dict):
            raise DecideError("_meta 必须为对象")
        created_at = _require_str(data.get("created_at"), "_meta.created_at")
        session_id = _require_str(data.get("session_id"), "_meta.session_id")
        return MetaInfo(created_at=created_at, session_id=session_id)

    def to_dict(self) -> dict[str, Any]:
        return {"created_at": self.created_at, "session_id": self.session_id}


@dataclass(frozen=True)
class DecideInput:
    task: str
    source: str
    items: list[DecideItem]
    meta: MetaInfo | None = None

    @staticmethod
    def from_dict(data: Any) -> "DecideInput":
        if not isinstance(data, dict):
            raise DecideError("输入数据必须为对象")
        task = _require_str(data.get("task"), "task")
        source = _require_str(data.get("source"), "source")

        raw_items = data.get("items")
        if not isinstance(raw_items, list) or not raw_items:
            raise DecideError("items 必须为至少 1 个元素的数组")

        items: list[DecideItem] = []
        used_ids: set[int] = set()
        for idx, raw_item in enumerate(raw_items):
            item = DecideItem.from_dict(raw_item, idx)
            if item.id in used_ids:
                raise DecideError(f"items[{idx}].id 与已有待定项重复: {item.id}")
            used_ids.add(item.id)
            items.append(item)

        meta = None
        if "_meta" in data and data.get("_meta") is not None:
            meta = MetaInfo.from_dict(data["_meta"])

        return DecideInput(task=task, source=source, items=items, meta=meta)

    def to_dict(self, include_meta: bool = True) -> dict[str, Any]:
        data: dict[str, Any] = {
            "task": self.task,
            "source": self.source,
            "items": [item.to_dict() for item in self.items],
        }
        if include_meta and self.meta is not None:
            data["_meta"] = self.meta.to_dict()
        return data

    def with_meta(self, meta: MetaInfo) -> "DecideInput":
        return DecideInput(
            task=self.task,
            source=self.source,
            items=self.items,
            meta=meta,
        )

    def without_meta(self) -> "DecideInput":
        return DecideInput(task=self.task, source=self.source, items=self.items, meta=None)


@dataclass(frozen=True)
class Decision:
    id: int
    chosen: str
    note: str | None = None

    @staticmethod
    def from_dict(data: Any, index: int) -> "Decision":
        path = f"decisions[{index}]"
        if not isinstance(data, dict):
            raise DecideError(f"{path} 必须为对象")
        item_id = _require_positive_int(data.get("id"), f"{path}.id")
        chosen = _require_str(data.get("chosen"), f"{path}.chosen")
        note = None
        if "note" in data and data.get("note") is not None:
            note = _require_str(data.get("note"), f"{path}.note", allow_empty=True)
        return Decision(id=item_id, chosen=chosen, note=note)

    def to_dict(self) -> dict[str, Any]:
        data: dict[str, Any] = {"id": self.id, "chosen": self.chosen}
        if self.note is not None:
            data["note"] = self.note
        return data


@dataclass(frozen=True)
class DecideOutput:
    decisions: list[Decision]

    @staticmethod
    def from_dict(data: Any) -> "DecideOutput":
        if not isinstance(data, dict):
            raise DecideError("输出数据必须为对象")
        raw_decisions = data.get("decisions")
        if not isinstance(raw_decisions, list) or not raw_decisions:
            raise DecideError("decisions 必须为至少 1 个元素的数组")
        decisions: list[Decision] = []
        for idx, raw in enumerate(raw_decisions):
            decisions.append(Decision.from_dict(raw, idx))
        return DecideOutput(decisions=decisions)

    def to_dict(self) -> dict[str, Any]:
        return {"decisions": [d.to_dict() for d in self.decisions]}


@dataclass(frozen=True)
class DecisionRecord:
    input: DecideInput
    output: DecideOutput
    completed_at: str

    @staticmethod
    def from_dict(data: Any) -> "DecisionRecord":
        if not isinstance(data, dict):
            raise DecideError("决策记录必须为对象")
        input_data = data.get("input")
        output_data = data.get("output")
        completed_at = _require_str(data.get("completed_at"), "completed_at")
        if input_data is None:
            raise DecideError("决策记录缺少 input")
        if output_data is None:
            raise DecideError("决策记录缺少 output")
        return DecisionRecord(
            input=DecideInput.from_dict(input_data),
            output=DecideOutput.from_dict(output_data),
            completed_at=completed_at,
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "input": self.input.to_dict(include_meta=False),
            "output": self.output.to_dict(),
            "completed_at": self.completed_at,
        }


def _require_str(value: Any, path: str, *, allow_empty: bool = False) -> str:
    if not isinstance(value, str):
        raise DecideError(f"{path} 必须为字符串")
    if not allow_empty and not value.strip():
        raise DecideError(f"{path} 不能为空")
    return value


def _require_int(value: Any, path: str) -> int:
    if isinstance(value, bool) or not isinstance(value, int):
        raise DecideError(f"{path} 必须为整数")
    return value


def _require_positive_int(value: Any, path: str) -> int:
    number = _require_int(value, path)
    if number <= 0:
        raise DecideError(f"{path} 必须为正整数")
    return number


def _optional_score(value: Any, path: str) -> float | None:
    if value is None:
        return None
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise DecideError(f"{path} 必须为数字")
    if value < 0 or value > 100:
        raise DecideError(f"{path} 必须在 0-100 范围内")
    return float(value)


def _optional_str_list(value: Any, path: str) -> list[str] | None:
    if value is None:
        return None
    if not isinstance(value, list):
        raise DecideError(f"{path} 必须为字符串数组")
    normalized: list[str] = []
    for idx, item in enumerate(value):
        if not isinstance(item, str):
            raise DecideError(f"{path}[{idx}] 必须为字符串")
        normalized.append(item)
    return normalized

