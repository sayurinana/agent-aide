const AppState = {
    task: "",
    source: "",
    items: [],
    decisions: {},
    notes: {},
    isSubmitting: false,
};

async function init() {
    try {
        const data = await loadItems();
        AppState.task = data.task || "";
        AppState.source = data.source || "";
        AppState.items = Array.isArray(data.items) ? data.items : [];

        // 如果有推荐项，默认选中推荐项
        AppState.items.forEach((item) => {
            if (item.recommend) {
                AppState.decisions[item.id] = item.recommend;
            }
        });

        renderItems(data);
        bindEvents();
    } catch (error) {
        showError("无法加载待定项数据，请刷新页面重试");
    }
}

async function loadItems() {
    const response = await fetch("/api/items");
    if (!response.ok) {
        throw new Error("加载失败");
    }
    return response.json();
}

function renderItems(data) {
    const container = document.getElementById("items-container");
    container.innerHTML = "";

    document.getElementById("task-name").textContent = data.task || "-";
    document.getElementById("task-source").textContent = data.source || "-";

    data.items.forEach((item) => {
        container.appendChild(renderItemCard(item));
    });

    updateProgress();
    updateSubmitButton();
}

function renderItemCard(item) {
    const card = document.createElement("article");
    card.className = "item-card";
    card.dataset.itemId = String(item.id);

    const header = document.createElement("header");
    header.className = "item-header";
    const title = document.createElement("h2");
    title.className = "item-title";
    const number = document.createElement("span");
    number.className = "item-number";
    number.textContent = `${item.id}.`;
    const titleText = document.createElement("span");
    titleText.textContent = item.title || "待定项";
    title.appendChild(number);
    title.appendChild(titleText);

    const recommend = document.createElement("span");
    recommend.className = "recommend-badge";
    if (item.recommend) {
        recommend.textContent = `推荐: ${item.recommend}`;
    } else {
        recommend.hidden = true;
    }

    header.appendChild(title);
    header.appendChild(recommend);
    card.appendChild(header);

    if (item.context) {
        const context = document.createElement("div");
        context.className = "item-context";
        context.textContent = item.context;
        card.appendChild(context);
    }

    if (item.location && item.location.file) {
        const location = document.createElement("div");
        location.className = "item-location";
        location.textContent = `位置: ${item.location.file}:${item.location.start}-${item.location.end}`;
        card.appendChild(location);
    }

    const options = renderOptions(item);
    card.appendChild(options);

    const noteWrap = document.createElement("div");
    noteWrap.className = "item-note";
    const noteLabel = document.createElement("label");
    noteLabel.setAttribute("for", `note-${item.id}`);
    noteLabel.textContent = "备注（可选）:";
    const textarea = document.createElement("textarea");
    textarea.id = `note-${item.id}`;
    textarea.placeholder = "添加补充说明...";
    noteWrap.appendChild(noteLabel);
    noteWrap.appendChild(textarea);
    card.appendChild(noteWrap);

    return card;
}

function renderOptions(item) {
    const optionsWrap = document.createElement("div");
    optionsWrap.className = "options-list";
    const current = AppState.decisions[item.id];

    item.options.forEach((option) => {
        const label = document.createElement("label");
        label.className = "option-item";
        label.dataset.value = option.value;
        label.dataset.recommended = option.value === item.recommend ? "true" : "false";

        const input = document.createElement("input");
        input.type = "radio";
        input.name = `item-${item.id}`;
        input.value = option.value;
        if (current === option.value) {
            input.checked = true;
            label.classList.add("selected");
        }

        const content = document.createElement("div");
        content.className = "option-content";

        const header = document.createElement("div");
        header.className = "option-header";
        const optLabel = document.createElement("span");
        optLabel.className = "option-label";
        optLabel.textContent = option.label || option.value;
        header.appendChild(optLabel);

        if (option.score !== undefined && option.score !== null) {
            const score = document.createElement("span");
            score.className = "option-score";
            score.textContent = `评分: ${option.score}`;
            header.appendChild(score);
        }

        content.appendChild(header);

        const hasPros = Array.isArray(option.pros) && option.pros.length > 0;
        const hasCons = Array.isArray(option.cons) && option.cons.length > 0;
        if (hasPros || hasCons) {
            const details = document.createElement("div");
            details.className = "option-details";
            if (hasPros) {
                const pros = document.createElement("div");
                pros.className = "option-pros";
                pros.innerHTML = `<strong>优点:</strong> ${option.pros.join("，")}`;
                details.appendChild(pros);
            }
            if (hasCons) {
                const cons = document.createElement("div");
                cons.className = "option-cons";
                cons.innerHTML = `<strong>缺点:</strong> ${option.cons.join("，")}`;
                details.appendChild(cons);
            }
            content.appendChild(details);
        }

        label.appendChild(input);
        label.appendChild(content);
        optionsWrap.appendChild(label);
    });

    return optionsWrap;
}

function bindEvents() {
    const container = document.getElementById("items-container");
    container.addEventListener("change", (event) => {
        const target = event.target;
        if (target && target.type === "radio") {
            const itemId = parseInt(target.name.replace("item-", ""), 10);
            handleOptionSelect(itemId, target.value);
        }
    });

    container.addEventListener("input", (event) => {
        const target = event.target;
        if (target && target.tagName === "TEXTAREA") {
            const itemId = parseInt(target.id.replace("note-", ""), 10);
            handleNoteInput(itemId, target.value);
        }
    });

    document.getElementById("submit-btn").addEventListener("click", submitDecisions);
}

function handleOptionSelect(itemId, value) {
    AppState.decisions[itemId] = value;
    const card = document.querySelector(`.item-card[data-item-id="${itemId}"]`);
    if (card) {
        const options = card.querySelectorAll(".option-item");
        options.forEach((opt) => {
            if (opt.dataset.value === value) {
                opt.classList.add("selected");
            } else {
                opt.classList.remove("selected");
            }
        });
        if (AppState.decisions[itemId]) {
            card.classList.add("completed");
        }
    }
    updateProgress();
    updateSubmitButton();
}

function handleNoteInput(itemId, note) {
    AppState.notes[itemId] = note;
}

function updateProgress() {
    const total = AppState.items.length;
    const completed = Object.keys(AppState.decisions).length;
    const text = document.getElementById("progress-text");
    text.textContent = `已完成 ${completed}/${total} 项`;
}

function canSubmit() {
    return (
        AppState.items.length > 0 &&
        Object.keys(AppState.decisions).length === AppState.items.length &&
        !AppState.isSubmitting
    );
}

function updateSubmitButton() {
    const button = document.getElementById("submit-btn");
    const allowed = canSubmit();
    button.disabled = !allowed;
    button.setAttribute("aria-disabled", allowed ? "false" : "true");
    button.textContent = AppState.isSubmitting ? "提交中..." : "提交决策";
}

function buildDecisionData() {
    const decisions = AppState.items.map((item) => {
        const note = AppState.notes[item.id];
        const trimmed = typeof note === "string" ? note.trim() : "";
        const payload = { id: item.id, chosen: AppState.decisions[item.id] };
        if (trimmed) {
            payload.note = trimmed;
        }
        return payload;
    });
    return { decisions };
}

async function submitDecisions() {
    if (!canSubmit()) {
        showError("请先完成所有待定项的选择");
        return;
    }
    AppState.isSubmitting = true;
    updateSubmitButton();

    try {
        const response = await fetch("/api/submit", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(buildDecisionData()),
        });

        if (!response.ok) {
            let detail = "提交失败";
            try {
                const error = await response.json();
                detail = error.detail || detail;
            } catch (e) {
                // ignore
            }
            throw new Error(detail);
        }

        showSuccess();
    } catch (error) {
        AppState.isSubmitting = false;
        updateSubmitButton();
        showError(`提交失败: ${error.message}`);
    }
}

function showSuccess() {
    const overlay = document.getElementById("success-overlay");
    overlay.hidden = false;
    AppState.isSubmitting = false;
    const button = document.getElementById("submit-btn");
    button.disabled = true;
    button.setAttribute("aria-disabled", "true");
}

let errorTimer = null;
function showError(message) {
    const toast = document.getElementById("error-toast");
    const text = document.getElementById("error-message");
    text.textContent = message;
    toast.hidden = false;
    if (errorTimer) {
        clearTimeout(errorTimer);
    }
    errorTimer = setTimeout(() => {
        toast.hidden = true;
    }, 4000);
}

document.addEventListener("DOMContentLoaded", init);
