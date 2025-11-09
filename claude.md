# aishell

**ID**: ai.syui.shell
**Name**: aishell
**SID**: ai.shell
**Version**: 0.1.0

## 概要

Claude Codeのような、AIがshellを操作するためのツール。
例えば、gpt-ossのようなllmを使用することを想定。場合によっては、MCPを駆使する。

## 主な機能

1. **マルチLLMプロバイダー対応**
   - OpenAI API互換（OpenAI, gpt-oss, etc.）
   - 将来的にClaude API、Ollamaなども対応予定

2. **Function Calling (Tool use)**
   - LLMが直接ツールを呼び出してシェルを操作
   - bash, read, write, list等のツールを提供

3. **MCPサーバーモード**
   - Claude Desktopとの連携が可能
   - aigptと同様のMCPプロトコル実装

## アーキテクチャ

```
User → CLI → LLM Provider → Function Calling → Shell Executor → Result
```

## AIOS統合

- **aigpt**: メモリー、パーソナリティ分析
- **aishell**: シェル操作、自動化
- **統合**: AIによるOS管理の実現

## 技術スタック

- Rust 2021
- tokio (async runtime)
- reqwest (HTTP client)
- duct (shell execution)
- clap (CLI framework)
