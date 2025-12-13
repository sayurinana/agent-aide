#!/bin/bash
# 创建 aide-env-test 测试目录和项目

set -e

BASE_DIR="/home/user/temp/ccoptimize/test-cache/aide-env-test"

# 创建基础目录
mkdir -p "$BASE_DIR"

# 创建 rust-demo 项目
mkdir -p "$BASE_DIR/rust-demo/src"
cat > "$BASE_DIR/rust-demo/Cargo.toml" << 'EOF'
[package]
name = "rust-demo"
version = "0.1.0"
edition = "2021"

[dependencies]
EOF

cat > "$BASE_DIR/rust-demo/src/main.rs" << 'EOF'
fn main() {
    println!("Hello from Rust demo!");
}
EOF

# 创建 flutter-demo 项目（简化结构）
mkdir -p "$BASE_DIR/flutter-demo/lib"
cat > "$BASE_DIR/flutter-demo/pubspec.yaml" << 'EOF'
name: flutter_demo
description: A Flutter demo project for aide env testing.
version: 1.0.0

environment:
  sdk: '>=3.0.0 <4.0.0'

dependencies:
  flutter:
    sdk: flutter

dev_dependencies:
  flutter_test:
    sdk: flutter
EOF

cat > "$BASE_DIR/flutter-demo/lib/main.dart" << 'EOF'
import 'package:flutter/material.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      home: const Scaffold(
        body: Center(
          child: Text('Hello from Flutter demo!'),
        ),
      ),
    );
  }
}
EOF

# 创建 react-demo 项目（简化结构）
mkdir -p "$BASE_DIR/react-demo/src"
cat > "$BASE_DIR/react-demo/package.json" << 'EOF'
{
  "name": "react-demo",
  "version": "1.0.0",
  "description": "A React demo project for aide env testing",
  "main": "src/index.js",
  "scripts": {
    "start": "react-scripts start",
    "build": "react-scripts build",
    "test": "react-scripts test"
  },
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0"
  },
  "devDependencies": {
    "react-scripts": "5.0.1"
  }
}
EOF

cat > "$BASE_DIR/react-demo/src/index.js" << 'EOF'
import React from 'react';
import ReactDOM from 'react-dom/client';

function App() {
  return <h1>Hello from React demo!</h1>;
}

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(<App />);
EOF

echo "✓ 测试项目目录结构创建完成"
echo "  - $BASE_DIR/rust-demo"
echo "  - $BASE_DIR/flutter-demo"
echo "  - $BASE_DIR/react-demo"
