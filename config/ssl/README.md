# ═══════════════════════════════════════════════════════════════════════════════
# SENTIENT CORE - SSL Certificates Directory
# ═══════════════════════════════════════════════════════════════════════════════
#
# Bu dizin SSL sertifikaları için kullanılır.
# Production ortamında gerçek sertifikalarınızı buraya yerleştirin.
#
# Gerekli dosyalar:
#   - cert.pem  : SSL sertifikası
#   - key.pem   : Private key
#   - chain.pem : CA chain (opsiyonel)
#
# Development için self-signed sertifika oluşturma:
#
#   openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
#     -days 365 -nodes -subj "/CN=localhost"
#
# Let's Encrypt kullanımı:
#
#   certbot certonly --standalone -d your-domain.com
#   cp /etc/letsencrypt/live/your-domain.com/fullchain.pem cert.pem
#   cp /etc/letsencrypt/live/your-domain.com/privkey.pem key.pem
#
# ⚠️  UYARI: Private key dosyalarını asla git'e commit etmeyin!
# ═══════════════════════════════════════════════════════════════════════════════
