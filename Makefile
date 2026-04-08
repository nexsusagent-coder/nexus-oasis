# ═══════════════════════════════════════════════════════════════════════════════
#  🧠 SENTIENT OS - MAKEFILE
#  The Operating System That Thinks
# ═══════════════════════════════════════════════════════════════════════════════

SHELL := /bin/bash
.PHONY: all build run test clean skills install help

# ═══════════════════════════════════════════════════════════════════════════════
# VARIABLES
# ═══════════════════════════════════════════════════════════════════════════════

CARGO := cargo
RELEASE := --release
SKILL_DB := data/sentient_skills.db
SKILL_DIR := data/skills
KNOWLEDGE_DIR := knowledge_base

# ═══════════════════════════════════════════════════════════════════════════════
# MAIN TARGETS
# ═══════════════════════════════════════════════════════════════════════════════

all: install build skills
	@echo "🧠 SENTIENT OS hazır!"

# ═══════════════════════════════════════════════════════════════════════════════
# INSTALLATION
# ═══════════════════════════════════════════════════════════════════════════════

install:
	@chmod +x setup.sh
	@./setup.sh all

install-rust:
	@./setup.sh rust

install-deps:
	@./setup.sh deps

install-docker:
	@./setup.sh docker

# ═══════════════════════════════════════════════════════════════════════════════
# BUILD
# ═══════════════════════════════════════════════════════════════════════════════

build:
	@echo "📦 SENTIENT OS derleniyor..."
	$(CARGO) build $(RELEASE)

build-dev:
	@echo "📦 SENTIENT OS derleniyor (debug)..."
	$(CARGO) build

build-fast:
	@echo "📦 Hızlı derleme..."
	$(CARGO) build $(RELEASE) --bin sentient-shell --bin sentient-dashboard

check:
	@echo "🔍 Kod kontrolü..."
	$(CARGO) check

clippy:
	@echo "🔍 Clippy analizi..."
	$(CARGO) clippy -- -W warnings

fmt:
	@echo "🎨 Formatlanıyor..."
	$(CARGO) fmt

# ═══════════════════════════════════════════════════════════════════════════════
# RUN
# ═══════════════════════════════════════════════════════════════════════════════

run: run-shell

run-shell:
	@echo "🧠 SENTIENT Shell başlatılıyor..."
	$(CARGO) run $(RELEASE) --bin sentient-shell

run-dashboard:
	@echo "📊 Dashboard başlatılıyor..."
	$(CARGO) run $(RELEASE) --bin sentient-dashboard

run-vgate:
	@echo "🔐 V-GATE Proxy başlatılıyor..."
	$(CARGO) run $(RELEASE) --bin sentient-vgate

run-orchestrator:
	@echo "🎯 Orchestrator başlatılıyor..."
	$(CARGO) run $(RELEASE) --bin sentient-orchestrator

# ═══════════════════════════════════════════════════════════════════════════════
# SKILLS
# ═══════════════════════════════════════════════════════════════════════════════

skills:
	@echo "📚 Skill Library güncelleniyor..."
	$(CARGO) run $(RELEASE) --bin sentient-ingest -- full

skills-stats:
	@echo "📊 Skill istatistikleri..."
	$(CARGO) run $(RELEASE) --bin sentient-ingest -- stats

skills-search:
	@read -p "Arama sorgusu: " query; \
	$(CARGO) run $(RELEASE) --bin sentient-ingest -- search "$$query"

skills-categories:
	@echo "📂 Kategoriler..."
	$(CARGO) run $(RELEASE) --bin sentient-ingest -- categories

# ═══════════════════════════════════════════════════════════════════════════════
# TEST
# ═══════════════════════════════════════════════════════════════════════════════

test:
	@echo "🧪 Testler çalıştırılıyor..."
	$(CARGO) test --workspace

test-verbose:
	@echo "🧪 Testler (detaylı)..."
	$(CARGO) test --workspace -- --nocapture

test-coverage:
	@echo "📊 Test coverage..."
	$(CARGO) tarpaulin --workspace --out Html

test-one:
	@read -p "Test adı: " name; \
	$(CARGO) test --workspace "$$name"

# ═══════════════════════════════════════════════════════════════════════════════
# DOCKER
# ═══════════════════════════════════════════════════════════════════════════════

docker-build:
	@echo "🐳 Docker image oluşturuluyor..."
	docker build -t sentient-os:latest .

docker-run:
	@echo "🐳 Docker container başlatılıyor..."
	docker run -it --rm \
		-v $(PWD)/data:/app/data \
		-v $(PWD)/.env:/app/.env \
		sentient-os:latest

docker-compose-up:
	@echo "🐳 Docker Compose başlatılıyor..."
	docker-compose up -d

docker-compose-down:
	@echo "🐳 Docker Compose durduruluyor..."
	docker-compose down

docker-logs:
	docker-compose logs -f

# ═══════════════════════════════════════════════════════════════════════════════
# CLEANUP
# ═══════════════════════════════════════════════════════════════════════════════

clean:
	@echo "🧹 Temizlik yapılıyor..."
	$(CARGO) clean
	rm -rf target/
	rm -rf venv/
	rm -rf node_modules/
	rm -f *.log

clean-skills:
	@echo "🧹 Skill cache temizleniyor..."
	rm -rf $(SKILL_DIR)/*.yaml
	rm -f $(SKILL_DB)

clean-all: clean clean-skills
	@echo "🧹 Tam temizlik yapıldı!"

# ═══════════════════════════════════════════════════════════════════════════════
# KNOWLEDGE BASE
# ═══════════════════════════════════════════════════════════════════════════════

knowledge:
	@echo "📚 Knowledge Base:"
	@ls -la $(KNOWLEDGE_DIR)/

knowledge-check:
	@echo "🔍 Knowledge Base kontrolü..."
	@test -f $(KNOWLEDGE_DIR)/nihai_entegrasyon.md && echo "✅ nihai_entegrasyon.md" || echo "❌ nihai_entegrasyon.md"
	@test -f $(KNOWLEDGE_DIR)/ek_direktifler.md && echo "✅ ek_direktifler.md" || echo "❌ ek_direktifler.md"
	@test -f $(KNOWLEDGE_DIR)/GAP_REPORT.md && echo "✅ GAP_REPORT.md" || echo "❌ GAP_REPORT.md"

# ═══════════════════════════════════════════════════════════════════════════════
# RELEASE
# ═══════════════════════════════════════════════════════════════════════════════

release: build
	@echo "🚀 Release build tamamlandı!"
	@ls -la target/release/sentient-*

install-bin:
	@echo "📦 Binary'ler kuruluyor..."
	@cp target/release/sentient-shell /usr/local/bin/sentient || sudo cp target/release/sentient-shell /usr/local/bin/sentient
	@chmod +x /usr/local/bin/sentient

# ═══════════════════════════════════════════════════════════════════════════════
# DEV TOOLS
# ═══════════════════════════════════════════════════════════════════════════════

dev:
	@echo "🔧 Development mode..."
	$(CARGO) watch -x "run --bin sentient-shell"

watch:
	@echo "👀 File watcher..."
	$(CARGO) watch -x check -x test

bench:
	@echo "⚡ Benchmarks..."
	$(CARGO) bench

doc:
	@echo "📖 Documentation..."
	$(CARGO) doc --no-deps --open

# ═══════════════════════════════════════════════════════════════════════════════
# SELF-CODING LOOP
# ═══════════════════════════════════════════════════════════════════════════════

self-improve:
	@echo "🔄 Self-Improvement Loop başlatılıyor..."
	$(CARGO) run $(RELEASE) --bin sentient-selfcoder

self-check:
	@echo "🔍 Codebase self-check..."
	$(CARGO) run $(RELEASE) --bin sentient-selfcoder -- check

self-fix:
	@echo "🔧 Auto-fix gaps..."
	$(CARGO) run $(RELEASE) --bin sentient-selfcoder -- fix

# ═══════════════════════════════════════════════════════════════════════════════
# STATUS
# ═══════════════════════════════════════════════════════════════════════════════

status:
	@echo ""
	@echo "╔═══════════════════════════════════════════════════════════════════╗"
	@echo "║   🧠 SENTIENT OS STATUS                                           ║"
	@echo "╠═══════════════════════════════════════════════════════════════════╣"
	@echo "║                                                                   ║"
	@echo "║   📦 Crates:          $(shell ls crates 2>/dev/null | wc -l | xargs printf '%-5s')                                    ║"
	@echo "║   📚 Skills:          $(shell find data/skills -name '*.yaml' 2>/dev/null | wc -l | xargs printf '%-5s')                                    ║"
	@echo "║   📖 Knowledge:       $(shell ls knowledge_base 2>/dev/null | wc -l | xargs printf '%-5s')                                    ║"
	@echo "║   🦀 Rust:            $(shell rustc --version 2>/dev/null | cut -d' ' -f2 | xargs printf '%-5s')                                    ║"
	@echo "║   🐳 Docker:          $(shell docker --version 2>/dev/null | cut -d' ' -f3 | tr -d ',' | xargs printf '%-5s')                                    ║"
	@echo "║                                                                   ║"
	@echo "╚═══════════════════════════════════════════════════════════════════╝"
	@echo ""

# ═══════════════════════════════════════════════════════════════════════════════
# HELP
# ═══════════════════════════════════════════════════════════════════════════════

help:
	@echo ""
	@echo "🧠 SENTIENT OS Makefile Komutları"
	@echo ""
	@echo "  KURULUM:"
	@echo "    make install         - Tam kurulum"
	@echo "    make build           - Release derleme"
	@echo "    make build-dev       - Debug derleme"
	@echo ""
	@echo "  ÇALIŞTIRMA:"
	@echo "    make run             - SENTIENT Shell"
	@echo "    make run-dashboard   - Dashboard"
	@echo "    make run-vgate       - V-GATE Proxy"
	@echo ""
	@echo "  SKILL:"
	@echo "    make skills          - Skill'leri güncelle"
	@echo "    make skills-stats    - İstatistikler"
	@echo "    make skills-search   - Skill ara"
	@echo ""
	@echo "  TEST:"
	@echo "    make test            - Testler"
	@echo "    make test-verbose    - Detaylı test"
	@echo ""
	@echo "  DOCKER:"
	@echo "    make docker-build    - Image oluştur"
	@echo "    make docker-run      - Container çalıştır"
	@echo ""
	@echo "  SELF-IMPROVEMENT:"
	@echo "    make self-improve    - Self-coding loop"
	@echo "    make self-check      - Gap kontrolü"
	@echo "    make self-fix        - Otomatik düzeltme"
	@echo ""
	@echo "  DİĞER:"
	@echo "    make clean           - Temizlik"
	@echo "    make status          - Durum raporu"
	@echo "    make help            - Bu yardım"
	@echo ""

# ═══════════════════════════════════════════════════════════════════════════════
# DEFAULT
# ═══════════════════════════════════════════════════════════════════════════════

.DEFAULT_GOAL := help
