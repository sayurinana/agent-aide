# aide decide Web 前端设计

## 一、概述

aide decide 的 Web 前端提供待定项确认界面，供用户选择选项并提交决策。

### 1.1 技术选型

| 技术 | 选择 | 理由 |
|------|------|------|
| 框架 | 纯 HTML/CSS/JS | 无需构建工具，无 Node.js 依赖 |
| 样式 | 原生 CSS | 简单场景无需预处理器 |
| 交互 | 原生 JavaScript | 避免引入框架依赖 |

### 1.2 设计原则

- **简洁**：聚焦核心功能，无多余装饰
- **清晰**：信息层次分明，操作明确
- **可用**：支持主流浏览器，无障碍友好

## 二、页面结构

### 2.1 整体布局

```
┌─────────────────────────────────────────────────────────────┐
│  ┌─────────────────────────────────────────────────────┐    │
│  │                     页面头部                         │    │
│  │  标题: Aide 待定项确认                               │    │
│  │  任务: <task>                                        │    │
│  │  来源: <source>                                      │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                   待定项列表                         │    │
│  │                                                      │    │
│  │  ┌─────────────────────────────────────────────┐    │    │
│  │  │ 待定项卡片 1                                 │    │    │
│  │  │ ...                                          │    │    │
│  │  └─────────────────────────────────────────────┘    │    │
│  │                                                      │    │
│  │  ┌─────────────────────────────────────────────┐    │    │
│  │  │ 待定项卡片 2                                 │    │    │
│  │  │ ...                                          │    │    │
│  │  └─────────────────────────────────────────────┘    │    │
│  │                                                      │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                     页面底部                         │    │
│  │                              [提交决策]              │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

### 2.2 待定项卡片结构

```
┌─────────────────────────────────────────────────────────────┐
│  1. <title>                                    [推荐: xxx]  │
├─────────────────────────────────────────────────────────────┤
│  <context>                                                   │
│                                                              │
│  位置: <file>:<start>-<end>                                 │
├─────────────────────────────────────────────────────────────┤
│  选项:                                                       │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ ○ <option_a.label>                        评分: 85  │    │
│  │   优点: xxx, xxx                                     │    │
│  │   缺点: xxx                                          │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ ● <option_b.label>  ← 已选中                评分: 70│    │
│  │   优点: xxx                                          │    │
│  │   缺点: xxx, xxx                                     │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
├─────────────────────────────────────────────────────────────┤
│  备注（可选）:                                               │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                                                      │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

## 三、HTML 结构原型

```html
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Aide 待定项确认</title>
    <link rel="stylesheet" href="/style.css">
</head>
<body>
    <div class="container">
        <!-- 页面头部 -->
        <header class="header">
            <h1>Aide 待定项确认</h1>
            <div class="task-info">
                <p><strong>任务:</strong> <span id="task-name"></span></p>
                <p><strong>来源:</strong> <span id="task-source"></span></p>
            </div>
        </header>

        <!-- 待定项列表 -->
        <main class="items-container" id="items-container">
            <!-- 动态生成待定项卡片 -->
        </main>

        <!-- 页面底部 -->
        <footer class="footer">
            <div class="progress">
                <span id="progress-text">已完成 0/0 项</span>
            </div>
            <button class="submit-btn" id="submit-btn" disabled>
                提交决策
            </button>
        </footer>
    </div>

    <!-- 提交成功提示 -->
    <div class="success-overlay" id="success-overlay" hidden>
        <div class="success-message">
            <h2>决策已提交</h2>
            <p>您可以关闭此页面</p>
        </div>
    </div>

    <script src="/app.js"></script>
</body>
</html>
```

### 3.1 待定项卡片模板

```html
<article class="item-card" data-item-id="{id}">
    <header class="item-header">
        <h2 class="item-title">
            <span class="item-number">{id}.</span>
            {title}
        </h2>
        <span class="recommend-badge" data-show="{hasRecommend}">
            推荐: {recommend}
        </span>
    </header>

    <div class="item-context" data-show="{hasContext}">
        {context}
    </div>

    <div class="item-location" data-show="{hasLocation}">
        位置: {file}:{start}-{end}
    </div>

    <div class="options-list">
        <!-- 选项列表 -->
    </div>

    <div class="item-note">
        <label for="note-{id}">备注（可选）:</label>
        <textarea id="note-{id}" placeholder="添加补充说明..."></textarea>
    </div>
</article>
```

### 3.2 选项模板

```html
<label class="option-item" data-value="{value}" data-recommended="{isRecommended}">
    <input type="radio" name="item-{itemId}" value="{value}">
    <div class="option-content">
        <div class="option-header">
            <span class="option-label">{label}</span>
            <span class="option-score" data-show="{hasScore}">
                评分: {score}
            </span>
        </div>
        <div class="option-details" data-show="{hasDetails}">
            <div class="option-pros" data-show="{hasPros}">
                <strong>优点:</strong> {pros}
            </div>
            <div class="option-cons" data-show="{hasCons}">
                <strong>缺点:</strong> {cons}
            </div>
        </div>
    </div>
</label>
```

## 四、CSS 样式规范

### 4.1 设计变量

```css
:root {
    /* 颜色 */
    --color-primary: #2563eb;
    --color-primary-hover: #1d4ed8;
    --color-success: #16a34a;
    --color-warning: #ca8a04;
    --color-error: #dc2626;
    --color-text: #1f2937;
    --color-text-secondary: #6b7280;
    --color-border: #e5e7eb;
    --color-background: #f9fafb;
    --color-card: #ffffff;

    /* 间距 */
    --spacing-xs: 4px;
    --spacing-sm: 8px;
    --spacing-md: 16px;
    --spacing-lg: 24px;
    --spacing-xl: 32px;

    /* 圆角 */
    --radius-sm: 4px;
    --radius-md: 8px;
    --radius-lg: 12px;

    /* 阴影 */
    --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.05);
    --shadow-md: 0 4px 6px rgba(0, 0, 0, 0.1);

    /* 字体 */
    --font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    --font-size-sm: 14px;
    --font-size-md: 16px;
    --font-size-lg: 18px;
    --font-size-xl: 24px;
}
```

### 4.2 关键样式类

| 类名 | 用途 |
|------|------|
| `.container` | 页面容器，最大宽度 800px，居中 |
| `.header` | 页面头部 |
| `.item-card` | 待定项卡片 |
| `.item-card.completed` | 已选择的卡片（边框高亮） |
| `.option-item` | 选项容器 |
| `.option-item.selected` | 已选中的选项（背景高亮） |
| `.option-item[data-recommended="true"]` | 推荐选项（特殊标记） |
| `.submit-btn` | 提交按钮 |
| `.submit-btn:disabled` | 禁用状态（未完成所有选择） |
| `.success-overlay` | 提交成功遮罩 |

### 4.3 响应式设计

虽然主要面向桌面浏览器，但应保证基本的响应式支持：

```css
/* 小屏幕适配 */
@media (max-width: 640px) {
    .container {
        padding: var(--spacing-sm);
    }

    .item-card {
        padding: var(--spacing-md);
    }

    .option-details {
        flex-direction: column;
    }
}
```

## 五、JavaScript 交互逻辑

### 5.1 应用状态

```javascript
const AppState = {
    task: "",           // 任务名称
    source: "",         // 来源文档
    items: [],          // 待定项列表
    decisions: {},      // 当前决策 { itemId: chosenValue }
    notes: {},          // 用户备注 { itemId: noteText }
    isSubmitting: false // 提交中标志
};
```

### 5.2 核心函数

```javascript
/**
 * 初始化应用
 * 1. 从 API 加载数据
 * 2. 渲染界面
 * 3. 绑定事件
 */
async function init(): void

/**
 * 加载待定项数据
 * GET /api/items
 */
async function loadItems(): DecideInput

/**
 * 渲染待定项列表
 */
function renderItems(data: DecideInput): void

/**
 * 渲染单个待定项卡片
 */
function renderItemCard(item: DecideItem): HTMLElement

/**
 * 渲染选项列表
 */
function renderOptions(item: DecideItem): HTMLElement

/**
 * 处理选项选择
 */
function handleOptionSelect(itemId: number, value: string): void

/**
 * 处理备注输入
 */
function handleNoteInput(itemId: number, note: string): void

/**
 * 更新进度显示
 */
function updateProgress(): void

/**
 * 检查是否可以提交
 */
function canSubmit(): boolean

/**
 * 提交决策
 * POST /api/submit
 */
async function submitDecisions(): void

/**
 * 显示成功提示
 */
function showSuccess(): void

/**
 * 显示错误提示
 */
function showError(message: string): void
```

### 5.3 事件绑定

```javascript
// 页面加载完成后初始化
document.addEventListener('DOMContentLoaded', init);

// 选项选择事件（事件委托）
document.getElementById('items-container').addEventListener('change', (e) => {
    if (e.target.type === 'radio') {
        const itemId = parseInt(e.target.name.replace('item-', ''));
        const value = e.target.value;
        handleOptionSelect(itemId, value);
    }
});

// 备注输入事件（事件委托）
document.getElementById('items-container').addEventListener('input', (e) => {
    if (e.target.tagName === 'TEXTAREA') {
        const itemId = parseInt(e.target.id.replace('note-', ''));
        handleNoteInput(itemId, e.target.value);
    }
});

// 提交按钮点击
document.getElementById('submit-btn').addEventListener('click', submitDecisions);
```

### 5.4 交互流程

```
@startuml
skinparam defaultFontName "PingFang SC"

start

:页面加载;

:调用 init();

:GET /api/items;

if (加载成功?) then (是)
  :渲染待定项列表;
  :绑定事件监听;
else (否)
  :显示错误信息;
  stop
endif

repeat
  :用户操作;
  switch (操作类型)
  case (选择选项)
    :更新 decisions;
    :高亮选中选项;
    :更新进度;
  case (输入备注)
    :更新 notes;
  case (点击提交)
    if (所有项已选择?) then (是)
      :禁用提交按钮;
      :POST /api/submit;
      if (提交成功?) then (是)
        :显示成功提示;
        stop
      else (否)
        :显示错误信息;
        :恢复提交按钮;
      endif
    else (否)
      :提示完成所有选择;
    endif
  endswitch
repeat while (继续操作?)

stop
@enduml
```

## 六、错误处理

### 6.1 加载错误

```javascript
async function loadItems() {
    try {
        const response = await fetch('/api/items');
        if (!response.ok) {
            throw new Error(`HTTP ${response.status}`);
        }
        return await response.json();
    } catch (error) {
        showError('无法加载待定项数据，请刷新页面重试');
        throw error;
    }
}
```

### 6.2 提交错误

```javascript
async function submitDecisions() {
    try {
        const response = await fetch('/api/submit', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(buildDecisionData())
        });

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.detail || '提交失败');
        }

        showSuccess();
    } catch (error) {
        showError(`提交失败: ${error.message}`);
        AppState.isSubmitting = false;
        updateSubmitButton();
    }
}
```

### 6.3 错误提示样式

```html
<div class="error-toast" id="error-toast" hidden>
    <span class="error-icon">✗</span>
    <span class="error-message" id="error-message"></span>
</div>
```

## 七、无障碍支持

### 7.1 语义化 HTML

- 使用 `<header>`, `<main>`, `<footer>`, `<article>` 等语义标签
- 使用 `<label>` 关联表单控件
- 使用 `<fieldset>` 和 `<legend>` 组织选项组

### 7.2 键盘导航

- 所有交互元素可通过 Tab 键访问
- 选项可通过方向键切换
- Enter 键可触发提交

### 7.3 ARIA 属性

```html
<main role="main" aria-label="待定项列表">
    <article role="group" aria-labelledby="item-title-1">
        <h2 id="item-title-1">...</h2>
        <div role="radiogroup" aria-label="选项">
            ...
        </div>
    </article>
</main>

<button aria-disabled="true" aria-describedby="submit-hint">
    提交决策
</button>
<span id="submit-hint" class="sr-only">
    请先完成所有待定项的选择
</span>
```

## 八、浏览器兼容性

### 8.1 目标浏览器

| 浏览器 | 最低版本 |
|--------|----------|
| Chrome | 80+ |
| Firefox | 75+ |
| Safari | 13+ |
| Edge | 80+ |

### 8.2 避免使用的特性

- CSS Grid 子网格（subgrid）
- CSS 容器查询（container queries）
- JavaScript 可选链操作符（?.）在旧版本中不支持
- Top-level await

### 8.3 Polyfill 策略

不引入 polyfill，通过避免新特性保证兼容性。

## 九、文件清单

```
aide/decide/web/
├── index.html      # 主页面
├── style.css       # 样式文件
└── app.js          # 交互逻辑
```

## 十、相关文档

- [decide 详细设计入口](README.md)
- [HTTP 服务设计](server.md)
- [数据格式文档](../../formats/data.md)
