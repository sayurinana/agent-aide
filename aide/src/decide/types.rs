use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub file: String,
    pub start: i64,
    pub end: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecideOption {
    pub value: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pros: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cons: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecideItem {
    pub id: i64,
    pub title: String,
    pub options: Vec<DecideOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommend: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaInfo {
    pub created_at: String,
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecideInput {
    pub task: String,
    pub source: String,
    pub items: Vec<DecideItem>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<MetaInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub id: i64,
    pub chosen: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecideOutput {
    pub decisions: Vec<Decision>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionRecord {
    pub input: DecideInput,
    pub output: DecideOutput,
    pub completed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub pid: u32,
    pub port: u16,
    pub url: String,
    pub started_at: String,
}

// === 验证函数 ===

pub fn validate_input(input: &DecideInput) -> Result<(), String> {
    if input.task.trim().is_empty() {
        return Err("task 不能为空".into());
    }
    if input.source.trim().is_empty() {
        return Err("source 不能为空".into());
    }
    if input.items.is_empty() {
        return Err("items 必须为至少 1 个元素的数组".into());
    }

    let mut used_ids = std::collections::HashSet::new();
    for (idx, item) in input.items.iter().enumerate() {
        if item.id <= 0 {
            return Err(format!("items[{idx}].id 必须为正整数"));
        }
        if used_ids.contains(&item.id) {
            return Err(format!("items[{idx}].id 与已有待定项重复: {}", item.id));
        }
        used_ids.insert(item.id);

        if item.title.trim().is_empty() {
            return Err(format!("items[{idx}].title 不能为空"));
        }
        if item.options.len() < 2 {
            return Err(format!(
                "items[{idx}].options 至少需要 2 个选项，当前只有 {} 个",
                item.options.len()
            ));
        }

        let mut used_values = std::collections::HashSet::new();
        for (oi, opt) in item.options.iter().enumerate() {
            if opt.value.trim().is_empty() {
                return Err(format!("items[{idx}].options[{oi}].value 不能为空"));
            }
            if used_values.contains(&opt.value) {
                return Err(format!(
                    "items[{idx}].options[{oi}].value 在当前待定项中必须唯一，重复值: {}",
                    opt.value
                ));
            }
            used_values.insert(opt.value.clone());

            if opt.label.trim().is_empty() {
                return Err(format!("items[{idx}].options[{oi}].label 不能为空"));
            }
            if let Some(score) = opt.score {
                if !(0.0..=100.0).contains(&score) {
                    return Err(format!(
                        "items[{idx}].options[{oi}].score 必须在 0-100 范围内"
                    ));
                }
            }
        }

        if let Some(ref recommend) = item.recommend {
            if !item.options.iter().any(|o| o.value == *recommend) {
                return Err(format!(
                    "items[{idx}].recommend 值 \"{recommend}\" 不在 options 中"
                ));
            }
        }

        if let Some(ref loc) = item.location {
            if loc.file.trim().is_empty() {
                return Err(format!("items[{idx}].location.file 不能为空"));
            }
        }
    }

    Ok(())
}

pub fn validate_output(output: &DecideOutput, input: &DecideInput) -> Result<(), String> {
    if output.decisions.len() != input.items.len() {
        return Err(format!(
            "decisions 数量 ({}) 与 items 数量 ({}) 不一致",
            output.decisions.len(),
            input.items.len()
        ));
    }

    let items_by_id: std::collections::HashMap<i64, &DecideItem> =
        input.items.iter().map(|i| (i.id, i)).collect();

    let mut seen = std::collections::HashSet::new();
    for decision in &output.decisions {
        if seen.contains(&decision.id) {
            return Err(format!("待定项 {} 的决策重复", decision.id));
        }
        seen.insert(decision.id);

        let item = items_by_id
            .get(&decision.id)
            .ok_or_else(|| format!("存在未知的待定项 {}", decision.id))?;

        if !item.options.iter().any(|o| o.value == decision.chosen) {
            return Err(format!(
                "待定项 {} 的决策值无效: {}",
                decision.id, decision.chosen
            ));
        }
    }

    let missing: Vec<String> = items_by_id
        .keys()
        .filter(|id| !seen.contains(id))
        .map(|id| id.to_string())
        .collect();
    if !missing.is_empty() {
        return Err(format!("缺少待定项 {} 的决策", missing.join(", ")));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_option(value: &str, label: &str) -> DecideOption {
        DecideOption {
            value: value.into(),
            label: label.into(),
            score: None,
            pros: None,
            cons: None,
        }
    }

    fn make_item(id: i64, title: &str, options: Vec<DecideOption>) -> DecideItem {
        DecideItem {
            id,
            title: title.into(),
            options,
            location: None,
            context: None,
            recommend: None,
        }
    }

    fn make_input(items: Vec<DecideItem>) -> DecideInput {
        DecideInput {
            task: "测试任务".into(),
            source: "test.md".into(),
            items,
            meta: None,
        }
    }

    // === validate_input 测试 ===

    #[test]
    fn test_validate_input_valid() {
        let input = make_input(vec![make_item(
            1,
            "选择",
            vec![make_option("a", "A"), make_option("b", "B")],
        )]);
        assert!(validate_input(&input).is_ok());
    }

    #[test]
    fn test_validate_input_empty_task() {
        let mut input = make_input(vec![make_item(
            1,
            "选择",
            vec![make_option("a", "A"), make_option("b", "B")],
        )]);
        input.task = "  ".into();
        assert!(validate_input(&input).is_err());
    }

    #[test]
    fn test_validate_input_empty_source() {
        let mut input = make_input(vec![make_item(
            1,
            "选择",
            vec![make_option("a", "A"), make_option("b", "B")],
        )]);
        input.source = "".into();
        assert!(validate_input(&input).is_err());
    }

    #[test]
    fn test_validate_input_empty_items() {
        let input = make_input(vec![]);
        let err = validate_input(&input).unwrap_err();
        assert!(err.contains("至少 1 个"));
    }

    #[test]
    fn test_validate_input_zero_id() {
        let input = make_input(vec![make_item(
            0,
            "选择",
            vec![make_option("a", "A"), make_option("b", "B")],
        )]);
        let err = validate_input(&input).unwrap_err();
        assert!(err.contains("正整数"));
    }

    #[test]
    fn test_validate_input_duplicate_id() {
        let input = make_input(vec![
            make_item(
                1,
                "选择1",
                vec![make_option("a", "A"), make_option("b", "B")],
            ),
            make_item(
                1,
                "选择2",
                vec![make_option("c", "C"), make_option("d", "D")],
            ),
        ]);
        let err = validate_input(&input).unwrap_err();
        assert!(err.contains("重复"));
    }

    #[test]
    fn test_validate_input_empty_title() {
        let input = make_input(vec![make_item(
            1,
            "  ",
            vec![make_option("a", "A"), make_option("b", "B")],
        )]);
        let err = validate_input(&input).unwrap_err();
        assert!(err.contains("title"));
    }

    #[test]
    fn test_validate_input_single_option() {
        let input = make_input(vec![make_item(1, "选择", vec![make_option("a", "A")])]);
        let err = validate_input(&input).unwrap_err();
        assert!(err.contains("至少需要 2 个"));
    }

    #[test]
    fn test_validate_input_empty_option_value() {
        let input = make_input(vec![make_item(
            1,
            "选择",
            vec![make_option("", "A"), make_option("b", "B")],
        )]);
        let err = validate_input(&input).unwrap_err();
        assert!(err.contains("value"));
    }

    #[test]
    fn test_validate_input_duplicate_option_value() {
        let input = make_input(vec![make_item(
            1,
            "选择",
            vec![make_option("a", "A"), make_option("a", "B")],
        )]);
        let err = validate_input(&input).unwrap_err();
        assert!(err.contains("唯一"));
    }

    #[test]
    fn test_validate_input_empty_option_label() {
        let input = make_input(vec![make_item(
            1,
            "选择",
            vec![make_option("a", ""), make_option("b", "B")],
        )]);
        let err = validate_input(&input).unwrap_err();
        assert!(err.contains("label"));
    }

    #[test]
    fn test_validate_input_score_out_of_range() {
        let mut opt = make_option("a", "A");
        opt.score = Some(101.0);
        let input = make_input(vec![make_item(1, "选择", vec![opt, make_option("b", "B")])]);
        let err = validate_input(&input).unwrap_err();
        assert!(err.contains("0-100"));
    }

    #[test]
    fn test_validate_input_valid_score() {
        let mut opt = make_option("a", "A");
        opt.score = Some(75.5);
        let input = make_input(vec![make_item(1, "选择", vec![opt, make_option("b", "B")])]);
        assert!(validate_input(&input).is_ok());
    }

    #[test]
    fn test_validate_input_valid_recommend() {
        let mut item = make_item(
            1,
            "选择",
            vec![make_option("a", "A"), make_option("b", "B")],
        );
        item.recommend = Some("a".into());
        let input = make_input(vec![item]);
        assert!(validate_input(&input).is_ok());
    }

    #[test]
    fn test_validate_input_invalid_recommend() {
        let mut item = make_item(
            1,
            "选择",
            vec![make_option("a", "A"), make_option("b", "B")],
        );
        item.recommend = Some("nonexistent".into());
        let input = make_input(vec![item]);
        let err = validate_input(&input).unwrap_err();
        assert!(err.contains("recommend"));
    }

    #[test]
    fn test_validate_input_location_empty_file() {
        let mut item = make_item(
            1,
            "选择",
            vec![make_option("a", "A"), make_option("b", "B")],
        );
        item.location = Some(Location {
            file: "  ".into(),
            start: 1,
            end: 10,
        });
        let input = make_input(vec![item]);
        let err = validate_input(&input).unwrap_err();
        assert!(err.contains("location.file"));
    }

    #[test]
    fn test_validate_input_with_all_optional_fields() {
        let mut opt = make_option("a", "选项A");
        opt.score = Some(80.0);
        opt.pros = Some(vec!["优点1".into()]);
        opt.cons = Some(vec!["缺点1".into()]);

        let mut item = make_item(1, "选择", vec![opt, make_option("b", "选项B")]);
        item.location = Some(Location {
            file: "test.rs".into(),
            start: 1,
            end: 10,
        });
        item.context = Some("上下文".into());
        item.recommend = Some("a".into());

        let input = make_input(vec![item]);
        assert!(validate_input(&input).is_ok());
    }

    // === validate_output 测试 ===

    #[test]
    fn test_validate_output_valid() {
        let input = make_input(vec![
            make_item(
                1,
                "选择1",
                vec![make_option("a", "A"), make_option("b", "B")],
            ),
            make_item(
                2,
                "选择2",
                vec![make_option("x", "X"), make_option("y", "Y")],
            ),
        ]);
        let output = DecideOutput {
            decisions: vec![
                Decision {
                    id: 1,
                    chosen: "a".into(),
                    note: None,
                },
                Decision {
                    id: 2,
                    chosen: "y".into(),
                    note: None,
                },
            ],
        };
        assert!(validate_output(&output, &input).is_ok());
    }

    #[test]
    fn test_validate_output_count_mismatch() {
        let input = make_input(vec![make_item(
            1,
            "选择",
            vec![make_option("a", "A"), make_option("b", "B")],
        )]);
        let output = DecideOutput { decisions: vec![] };
        let err = validate_output(&output, &input).unwrap_err();
        assert!(err.contains("不一致"));
    }

    #[test]
    fn test_validate_output_unknown_id() {
        let input = make_input(vec![make_item(
            1,
            "选择",
            vec![make_option("a", "A"), make_option("b", "B")],
        )]);
        let output = DecideOutput {
            decisions: vec![Decision {
                id: 99,
                chosen: "a".into(),
                note: None,
            }],
        };
        let err = validate_output(&output, &input).unwrap_err();
        assert!(err.contains("未知"));
    }

    #[test]
    fn test_validate_output_invalid_chosen() {
        let input = make_input(vec![make_item(
            1,
            "选择",
            vec![make_option("a", "A"), make_option("b", "B")],
        )]);
        let output = DecideOutput {
            decisions: vec![Decision {
                id: 1,
                chosen: "z".into(),
                note: None,
            }],
        };
        let err = validate_output(&output, &input).unwrap_err();
        assert!(err.contains("无效"));
    }

    #[test]
    fn test_validate_output_duplicate_decision() {
        let input = make_input(vec![
            make_item(
                1,
                "选择1",
                vec![make_option("a", "A"), make_option("b", "B")],
            ),
            make_item(
                2,
                "选择2",
                vec![make_option("x", "X"), make_option("y", "Y")],
            ),
        ]);
        let output = DecideOutput {
            decisions: vec![
                Decision {
                    id: 1,
                    chosen: "a".into(),
                    note: None,
                },
                Decision {
                    id: 1,
                    chosen: "b".into(),
                    note: None,
                },
            ],
        };
        let err = validate_output(&output, &input).unwrap_err();
        assert!(err.contains("重复"));
    }

    #[test]
    fn test_validate_output_with_note() {
        let input = make_input(vec![make_item(
            1,
            "选择",
            vec![make_option("a", "A"), make_option("b", "B")],
        )]);
        let output = DecideOutput {
            decisions: vec![Decision {
                id: 1,
                chosen: "a".into(),
                note: Some("选 A 因为更好".into()),
            }],
        };
        assert!(validate_output(&output, &input).is_ok());
    }

    // === 序列化/反序列化测试 ===

    #[test]
    fn test_decide_input_serialization() {
        let input = make_input(vec![make_item(
            1,
            "选择",
            vec![make_option("a", "A"), make_option("b", "B")],
        )]);
        let json = serde_json::to_string(&input).unwrap();
        let parsed: DecideInput = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.task, "测试任务");
        assert_eq!(parsed.items.len(), 1);
        assert_eq!(parsed.items[0].options.len(), 2);
    }

    #[test]
    fn test_meta_field_rename() {
        let input = DecideInput {
            task: "任务".into(),
            source: "src.md".into(),
            items: vec![make_item(
                1,
                "选择",
                vec![make_option("a", "A"), make_option("b", "B")],
            )],
            meta: Some(MetaInfo {
                created_at: "2024-01-01".into(),
                session_id: "test-session".into(),
            }),
        };
        let json = serde_json::to_string(&input).unwrap();
        assert!(json.contains("\"_meta\""));
        assert!(!json.contains("\"meta\""));
    }

    #[test]
    fn test_skip_serializing_none_fields() {
        let input = make_input(vec![make_item(
            1,
            "选择",
            vec![make_option("a", "A"), make_option("b", "B")],
        )]);
        let json = serde_json::to_string(&input).unwrap();
        assert!(!json.contains("_meta"));
        assert!(!json.contains("location"));
        assert!(!json.contains("context"));
        assert!(!json.contains("recommend"));
        assert!(!json.contains("score"));
        assert!(!json.contains("pros"));
        assert!(!json.contains("cons"));
    }
}
