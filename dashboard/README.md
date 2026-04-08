# 🧠 SENTIENT OS Dashboard

SENTIENT AI Operating System - Görsel Kontrol Merkezi

## Yapı

```
dashboard/
├── src/
│   ├── main.rs          # Dashboard entry point
│   ├── skills_hub.rs    # Yetenekler paneli
│   ├── tool_monitor.rs  # Araç durumu
│   └── vgate_panel.rs   # V-GATE bağlantı durumu
├── components/
│   ├── skills_grid.rs   # Skill'leri grid olarak göster
│   ├── tool_status.rs   # Tool durum kartları
│   └── memory_viz.rs    # Bellek görselleştirme
└── assets/
    ├── logo.svg         # SENTIENT logosu
    └── theme.css        # TUI teması
```

## Özellikler

1. **Skills Hub**: Yüklenmiş skill'leri görüntüle
2. **Tool Monitor**: Aktif araçları izle
3. **V-GATE Panel**: API bağlantı durumu
4. **Memory Viz**: Bellek kullanım grafiği

## Çalıştırma

```bash
cd /root/SENTIENT_CORE
cargo run --package sentient_dashboard
```

## Entegrasyon

Dashboard, `oasis_hands` crate'i ile entegre çalışır:
- `SkillLoader` ile skill listesi
- `SENTIENTToolExecutor` ile tool durumu
- `sentient_vgate` ile bağlantı durumu
