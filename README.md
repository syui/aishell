# aishell

**AI-powered shell automation tool** - A generic alternative to Claude Code

## Overview

aishellは、AIがシェルを操作するための汎用的なツールです。Claude Codeのような機能を、より柔軟で拡張可能な形で提供します。

**主な特徴:**
- **マルチLLMプロバイダー対応**: OpenAI、Claude、ローカルLLM（gpt-oss等）
- **Function Calling**: LLMがツールを直接呼び出してシェルを操作
- **MCPサーバー**: Claude Desktopとの連携も可能
- **AIOS統合**: aigptと組み合わせてAIによるOS管理を実現

## Installation

```bash
# Rust環境が必要
cargo build --release

# バイナリをインストール
cargo install --path .
```

## Usage

### 1. 対話型シェル (Interactive Shell)

```bash
# OpenAI互換APIを使用
export OPENAI_API_KEY="your-api-key"
aishell shell

# 別のモデルを指定
aishell shell -m gpt-4o

# gpt-ossなどのOpenAI互換サーバーを使用
export OPENAI_BASE_URL="http://localhost:8080/v1"
aishell shell
```

**使用例:**
```
aishell> List all Rust files in src/
[Executing tool: list]
src/main.rs
src/lib.rs
...

aishell> Create a new file hello.txt with "Hello, World!"
[Executing tool: write]
Successfully wrote to file: hello.txt

aishell> Show me the git status
[Executing tool: bash]
On branch main
...
```

### 2. ワンショット実行 (Single Command)

```bash
aishell exec "Show me the current directory structure"
```

### 3. MCPサーバーモード (Claude Desktop Integration)

```bash
aishell server
```

**Claude Desktop設定** (`~/Library/Application Support/Claude/claude_desktop_config.json`):
```json
{
  "mcpServers": {
    "aishell": {
      "command": "/path/to/aishell",
      "args": ["server"]
    }
  }
}
```

## Architecture

```
aishell/
├── src/
│   ├── cli/         # 対話型インターフェイス (REPL)
│   ├── llm/         # LLMプロバイダー (OpenAI互換)
│   ├── shell/       # シェル実行エンジン
│   ├── mcp/         # MCPサーバー実装
│   └── config/      # 設定管理
```

**実行フロー:**
```
User Input → LLM (Function Calling) → Tool Execution → Shell → Result → LLM → User
```

## Available Tools

aishellは以下のツールをLLMに提供します:

- **bash**: シェルコマンドを実行
- **read**: ファイルを読み込み
- **write**: ファイルに書き込み
- **list**: ファイル一覧を取得

## Environment Variables

| 変数 | 説明 | デフォルト |
|------|------|----------|
| `OPENAI_API_KEY` | OpenAI APIキー | (必須) |
| `OPENAI_BASE_URL` | APIベースURL | `https://api.openai.com/v1` |
| `OPENAI_MODEL` | 使用するモデル | `gpt-4` |

## Integration with AIOS

aishellは[aigpt](https://github.com/syui/aigpt)と組み合わせることで、AIOS（AI Operating System）の一部として機能します:

- **aigpt**: AIメモリー、パーソナリティ分析
- **aishell**: シェル操作、自動化
- **AIOS**: これらを統合したAIによるOS管理システム

## Comparison with Claude Code

| 機能 | Claude Code | aishell |
|------|------------|---------|
| LLM | Claude専用 | **マルチプロバイダー** |
| 実行環境 | Electron Desktop | **CLI/MCP** |
| カスタマイズ | 限定的 | **完全制御** |
| ローカルLLM | 非対応 | **対応可能** |
| AIOS統合 | 不可 | **ネイティブ対応** |

## Development

```bash
# 開発ビルド
cargo build

# テスト実行
cargo test

# ログ有効化
RUST_LOG=debug aishell shell
```

## Technical Stack

- **Language**: Rust 2021
- **CLI**: clap 4.5
- **Async Runtime**: tokio 1.40
- **HTTP Client**: reqwest 0.12
- **Shell Execution**: duct 0.13
- **REPL**: rustyline 14.0

## Roadmap

- [ ] Anthropic Claude API対応
- [ ] Ollama対応（ローカルLLM）
- [ ] より高度なツールセット（git統合、ファイル検索等）
- [ ] 設定ファイルサポート
- [ ] セッション履歴の永続化
- [ ] プラグインシステム

## License

MIT License

## Author

syui

## Related Projects

- [aigpt](https://github.com/syui/aigpt) - AI Memory System
