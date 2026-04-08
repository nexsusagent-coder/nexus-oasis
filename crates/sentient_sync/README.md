# SENTIENT SYNC - Silent Auto-Update Engine

> Kullanıcıya hissettirmeden entegre repoları güncelleyen arka plan motoru

## Özellikler

- 🔍 **Otomatik Keşif**: 71 entegre repoyu otomatik tespit eder
- 🔄 **Sessiz Güncelleme**: Kullanıcıya hissettirmeden günceller
- 🛡️ **Güvenlik Taraması**: Hassas dosyaları otomatik tespit eder
- 🔀 **Conflict Çözümü**: Akıllı conflict çözüm stratejileri
- 📊 **Detaylı Loglama**: Tüm işlemler loglanır
- ⏰ **Zamanlanmış Çalışma**: Belirlenen aralıklarla otomatik kontrol

## Mimari

```
┌─────────────────────────────────────────────────────────────────┐
│                    SENTIENT SYNC ENGINE                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   ┌─────────────┐    ┌─────────────┐    ┌─────────────┐       │
│   │   Config    │───▶│   Tracker   │───▶│   Updater   │       │
│   │  (Ayarlar)  │    │ (Repo Tarama)│   │ (Güncelleme)│       │
│   └─────────────┘    └─────────────┘    └─────────────┘       │
│          │                  │                  │                │
│          ▼                  ▼                  ▼                │
│   ┌─────────────────────────────────────────────────────┐     │
│   │                    Scheduler                         │     │
│   │           (Zamanlanmış Görev Yönetimi)              │     │
│   └─────────────────────────────────────────────────────┘     │
│          │                                                      │
│          ▼                                                      │
│   ┌─────────────────────────────────────────────────────┐     │
│   │                    Sync State                        │     │
│   │             (Durum Yönetimi - Memory)               │     │
│   └─────────────────────────────────────────────────────┘     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Kullanım

### Daemon Başlatma

```bash
# Daemon'ı başlat
cargo run --bin sentient-sync-daemon

# Veya doğrudan binary
./sentient-sync-daemon
```

### Programatik Kullanım

```rust
use sentient_sync::{SyncEngine, SyncConfig};

#[tokio::main]
async fn main() {
    // Config oluştur
    let config = SyncConfig::default();
    
    // Engine başlat
    let engine = SyncEngine::new(config).await.unwrap();
    
    // Manuel sync
    let report = engine.sync_all().await.unwrap();
    println!("Updated: {}, Failed: {}", report.updated, report.failed);
    
    // Veya arka planda sürekli çalıştır
    engine.start().await.unwrap();
}
```

## Konfigürasyon

```json
{
  "integrations_path": "./integrations",
  "sync_interval_minutes": 30,
  "max_concurrent_updates": 5,
  "network_timeout_secs": 60,
  "conflict_strategy": "PreferTheirs",
  "auto_merge": true,
  "security": {
    "verify_signatures": false,
    "allowed_branches": ["main", "master"],
    "scan_secrets": true
  }
}
```

### Conflict Stratejileri

| Strateji | Açıklama |
|----------|----------|
| `PreferTheirs` | Upstream'ı tercih et (önerilen) |
| `PreferOurs` | Local değişiklikleri koru |
| `Skip` | Conflict varsa atla |
| `Manual` | Manuel müdahale iste |

## Güvenlik

### Otomatik Tarama

- `.env` dosyaları
- `credentials` içeren dosyalar
- `api_key`, `password`, `token` içeren dosyalar
- `private_key` dosyaları

### Branch Kısıtlaması

Varsayılan olarak sadece `main` ve `master` branch'leri güncellenir.

## Loglama

```
INFO  sentient_sync::scheduler 🔄 Sync Scheduler started (interval: 30min)
DEBUG sentient_sync::tracker     Found 71 repositories
INFO  sentient_sync::scheduler   📦 Update available for: langchain
INFO  sentient_sync::updater     Updated langchain: abc123 -> def456 (15 files changed)
INFO  sentient_sync::scheduler   ✅ Sync cycle completed in 45s: 12 updated, 0 failed
```

## Modüller

| Modül | Açıklama |
|-------|----------|
| `config` | Konfigürasyon yönetimi |
| `tracker` | Repo keşif ve takip |
| `updater` | Sessiz güncelleme |
| `diff` | Değişiklik analizi |
| `sync_state` | Durum yönetimi |
| `scheduler` | Zamanlama |
| `webhook` | GitHub webhook desteği |

---

*SENTIENT OS - The Operating System That Thinks*
