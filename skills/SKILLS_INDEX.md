# 📚 SENTIENT OS SKILL LIBRARY

> **5587 Native Skills** - The World's Largest AI Skill Collection

---

## 📊 Overview

| Category | Skills | Description |
|----------|--------|-------------|
| **Dev** | 2,965+ | Coding, Web, DevOps, CLI Tools |
| **OSINT** | 1,050+ | Search, Research, Browser Automation |
| **Social** | 238+ | Communication, Marketing |
| **Automation** | 306+ | Productivity, Calendar, Smart Home |
| **Media** | 246+ | Image/Video, Streaming, Speech |
| **Productivity** | 214+ | Notes, PDF, Apple Apps |
| **Security** | 52+ | Security, Passwords |
| **Mobile** | 233+ | Transportation, Health, Shopping |
| **Gaming** | 108+ | Gaming, Personal Development |

---

## 🗂️ Category Structure

```
skills/
├── Dev/
│   ├── Coding-Agents-IDEs/     # 1374 skills
│   ├── Web-Frontend/           # 901 skills
│   ├── DevOps-Cloud/           # 375 skills
│   ├── Git-GitHub/             # 155 skills
│   ├── CLI-Tools/              # 170 skills
│   └── iOS-macOS/              # 29 skills
├── OSINT/
│   ├── Search-Research/        # 339 skills
│   ├── Browser-Automation/     # 336 skills
│   └── Data-Analytics/         # 35 skills
├── Social/
│   ├── Communication/          # 141 skills
│   └── Marketing-Sales/        # 97 skills
├── Automation/
│   ├── Productivity/           # 202 skills
│   ├── Calendar/               # 64 skills
│   └── Smart-Home/             # 40 skills
├── Media/
│   ├── Image-Video-Gen/        # 164 skills
│   ├── Streaming/              # 84 skills
│   └── Speech/                 # 42 skills
├── Productivity/
│   ├── Notes-PKM/              # 69 skills
│   ├── PDF-Documents/          # 102 skills
│   └── Apple-Apps/             # 43 skills
├── Security/
│   └── Security-Passwords/     # 52 skills
├── Mobile/
│   ├── Transportation/         # 108 skills
│   ├── Health-Fitness/         # 81 skills
│   └── Shopping/               # 45 skills
└── Gaming/
    ├── Gaming/                 # 25 skills
    ├── Personal-Dev/           # 48 skills
    └── Moltbook/               # 35 skills
```

---

## 🔥 Top Categories

### Dev (2,965+ Skills)
Coding agents, IDEs, web development, frontend frameworks, DevOps, cloud platforms, Git workflows, CLI utilities.

**Sources:**
- OpenClaw Skills (5143 skills)
- Everything Claude Code (181 skills)
- Gstack Skills (37 skills)

### OSINT (1,050+ Skills)
Web research, browser automation, data analytics, intelligence gathering, automated browsing.

### Automation (306+ Skills)
Task automation, calendar management, smart home integration, productivity tools.

---

## 📖 Skill Format

Each skill is a YAML file with:

```yaml
name: skill-name
description: What this skill does
author: creator-name
tags: [tag1, tag2]
github_url: https://github.com/...
```

---

## 🔍 Usage

### CLI
```bash
# Search skills
./target/release/sentient-ingest search "browser automation"

# List all
./target/release/sentient-ingest stats

# Execute skill
./target/release/sentient-shell
> /skill browser-navigate
```

### Dashboard
```bash
./target/release/sentient-dashboard
# Navigate to Skills tab
```

---

## 📥 Ingest More Skills

```bash
# From OpenClaw repository
cargo run --bin sentient-ingest -- openclaw

# From Everything Claude Code
cargo run --bin sentient-ingest -- ecc

# From Gstack
cargo run --bin sentient-ingest -- gstack

# Full ingestion
cargo run --bin sentient-ingest -- full
```

---

## 📜 License

Skills are sourced from:
- [awesome-openclaw-skills](https://github.com/piersandpubs/awesome-openclaw-skills) (CC0)
- [everything-claude-code](https://github.com/psifertex/everything-claude-code) (MIT)
- OpenHarness Community Skills (MIT)

---

*Last Updated: 2026-04-06*
