//! Translation strings for all supported languages

use std::collections::HashMap;

/// English translations
pub fn english() -> HashMap<String, String> {
    let mut m = HashMap::new();
    
    // App
    m.insert("app.name".to_string(), "SENTIENT AI".to_string());
    m.insert("app.tagline".to_string(), "Your Intelligent Assistant".to_string());
    m.insert("app.version".to_string(), "Version {version}".to_string());
    
    // Greetings
    m.insert("greeting.hello".to_string(), "Hello, {name}!".to_string());
    m.insert("greeting.welcome".to_string(), "Welcome to SENTIENT!".to_string());
    m.insert("greeting.goodbye".to_string(), "Goodbye!".to_string());
    
    // Common
    m.insert("common.yes".to_string(), "Yes".to_string());
    m.insert("common.no".to_string(), "No".to_string());
    m.insert("common.ok".to_string(), "OK".to_string());
    m.insert("common.cancel".to_string(), "Cancel".to_string());
    m.insert("common.save".to_string(), "Save".to_string());
    m.insert("common.delete".to_string(), "Delete".to_string());
    m.insert("common.edit".to_string(), "Edit".to_string());
    m.insert("common.search".to_string(), "Search".to_string());
    m.insert("common.loading".to_string(), "Loading...".to_string());
    m.insert("common.error".to_string(), "Error".to_string());
    m.insert("common.success".to_string(), "Success".to_string());
    
    // Errors
    m.insert("error.general".to_string(), "An error occurred. Please try again.".to_string());
    m.insert("error.not_found".to_string(), "Not found".to_string());
    m.insert("error.unauthorized".to_string(), "Unauthorized access".to_string());
    m.insert("error.rate_limit".to_string(), "Rate limit exceeded. Please wait.".to_string());
    m.insert("error.timeout".to_string(), "Request timed out".to_string());
    
    // Agent
    m.insert("agent.status.active".to_string(), "Active".to_string());
    m.insert("agent.status.inactive".to_string(), "Inactive".to_string());
    m.insert("agent.status.error".to_string(), "Error".to_string());
    m.insert("agent.created".to_string(), "Agent created successfully".to_string());
    m.insert("agent.deleted".to_string(), "Agent deleted successfully".to_string());
    
    // Message
    m.insert("message.placeholder".to_string(), "Type your message...".to_string());
    m.insert("message.send".to_string(), "Send".to_string());
    m.insert("message.typing".to_string(), "Typing...".to_string());
    
    // Voice
    m.insert("voice.listening".to_string(), "Listening...".to_string());
    m.insert("voice.speaking".to_string(), "Speaking...".to_string());
    m.insert("voice.wake_word".to_string(), "Say 'Hey SENTIENT' to activate".to_string());
    
    // Channel
    m.insert("channel.connected".to_string(), "Connected to {channel}".to_string());
    m.insert("channel.disconnected".to_string(), "Disconnected from {channel}".to_string());
    
    // Memory
    m.insert("memory.saved".to_string(), "Memory saved".to_string());
    m.insert("memory.cleared".to_string(), "Memory cleared".to_string());
    
    // Settings
    m.insert("settings.title".to_string(), "Settings".to_string());
    m.insert("settings.language".to_string(), "Language".to_string());
    m.insert("settings.theme".to_string(), "Theme".to_string());
    m.insert("settings.notifications".to_string(), "Notifications".to_string());
    
    m
}

/// Turkish translations
pub fn turkish() -> HashMap<String, String> {
    let mut m = HashMap::new();
    
    // App
    m.insert("app.name".to_string(), "SENTIENT AI".to_string());
    m.insert("app.tagline".to_string(), "Akıllı Asistanınız".to_string());
    m.insert("app.version".to_string(), "Sürüm {version}".to_string());
    
    // Greetings
    m.insert("greeting.hello".to_string(), "Merhaba, {name}!".to_string());
    m.insert("greeting.welcome".to_string(), "SENTIENT'e hoş geldiniz!".to_string());
    m.insert("greeting.goodbye".to_string(), "Hoşça kalın!".to_string());
    
    // Common
    m.insert("common.yes".to_string(), "Evet".to_string());
    m.insert("common.no".to_string(), "Hayır".to_string());
    m.insert("common.ok".to_string(), "Tamam".to_string());
    m.insert("common.cancel".to_string(), "İptal".to_string());
    m.insert("common.save".to_string(), "Kaydet".to_string());
    m.insert("common.delete".to_string(), "Sil".to_string());
    m.insert("common.edit".to_string(), "Düzenle".to_string());
    m.insert("common.search".to_string(), "Ara".to_string());
    m.insert("common.loading".to_string(), "Yükleniyor...".to_string());
    m.insert("common.error".to_string(), "Hata".to_string());
    m.insert("common.success".to_string(), "Başarılı".to_string());
    
    // Errors
    m.insert("error.general".to_string(), "Bir hata oluştu. Lütfen tekrar deneyin.".to_string());
    m.insert("error.not_found".to_string(), "Bulunamadı".to_string());
    m.insert("error.unauthorized".to_string(), "Yetkisiz erişim".to_string());
    m.insert("error.rate_limit".to_string(), "Hız sınırı aşıldı. Lütfen bekleyin.".to_string());
    m.insert("error.timeout".to_string(), "İstek zaman aşımına uğradı".to_string());
    
    // Agent
    m.insert("agent.status.active".to_string(), "Aktif".to_string());
    m.insert("agent.status.inactive".to_string(), "Pasif".to_string());
    m.insert("agent.status.error".to_string(), "Hata".to_string());
    m.insert("agent.created".to_string(), "Ajan başarıyla oluşturuldu".to_string());
    m.insert("agent.deleted".to_string(), "Ajan başarıyla silindi".to_string());
    
    // Message
    m.insert("message.placeholder".to_string(), "Mesajınızı yazın...".to_string());
    m.insert("message.send".to_string(), "Gönder".to_string());
    m.insert("message.typing".to_string(), "Yazıyor...".to_string());
    
    // Voice
    m.insert("voice.listening".to_string(), "Dinliyor...".to_string());
    m.insert("voice.speaking".to_string(), "Konuşuyor...".to_string());
    m.insert("voice.wake_word".to_string(), "Etkinleştirmek için 'Hey SENTIENT' deyin".to_string());
    
    // Channel
    m.insert("channel.connected".to_string(), "{channel} kanalına bağlanıldı".to_string());
    m.insert("channel.disconnected".to_string(), "{channel} kanalından ayrıldı".to_string());
    
    // Memory
    m.insert("memory.saved".to_string(), "Bellek kaydedildi".to_string());
    m.insert("memory.cleared".to_string(), "Bellek temizlendi".to_string());
    
    // Settings
    m.insert("settings.title".to_string(), "Ayarlar".to_string());
    m.insert("settings.language".to_string(), "Dil".to_string());
    m.insert("settings.theme".to_string(), "Tema".to_string());
    m.insert("settings.notifications".to_string(), "Bildirimler".to_string());
    
    m
}

/// German translations
pub fn german() -> HashMap<String, String> {
    let mut m = HashMap::new();
    
    m.insert("app.name".to_string(), "SENTIENT AI".to_string());
    m.insert("app.tagline".to_string(), "Ihr intelligenter Assistent".to_string());
    m.insert("greeting.hello".to_string(), "Hallo, {name}!".to_string());
    m.insert("greeting.welcome".to_string(), "Willkommen bei SENTIENT!".to_string());
    m.insert("greeting.goodbye".to_string(), "Auf Wiedersehen!".to_string());
    m.insert("common.yes".to_string(), "Ja".to_string());
    m.insert("common.no".to_string(), "Nein".to_string());
    m.insert("common.ok".to_string(), "OK".to_string());
    m.insert("common.cancel".to_string(), "Abbrechen".to_string());
    m.insert("common.save".to_string(), "Speichern".to_string());
    m.insert("common.delete".to_string(), "Löschen".to_string());
    m.insert("common.loading".to_string(), "Laden...".to_string());
    m.insert("error.general".to_string(), "Ein Fehler ist aufgetreten. Bitte versuchen Sie es erneut.".to_string());
    m.insert("message.placeholder".to_string(), "Ihre Nachricht eingeben...".to_string());
    m.insert("message.send".to_string(), "Senden".to_string());
    m.insert("voice.listening".to_string(), "Hören...".to_string());
    m.insert("settings.title".to_string(), "Einstellungen".to_string());
    m.insert("settings.language".to_string(), "Sprache".to_string());
    
    m
}

/// French translations
pub fn french() -> HashMap<String, String> {
    let mut m = HashMap::new();
    
    m.insert("app.name".to_string(), "SENTIENT AI".to_string());
    m.insert("app.tagline".to_string(), "Votre assistant intelligent".to_string());
    m.insert("greeting.hello".to_string(), "Bonjour, {name}!".to_string());
    m.insert("greeting.welcome".to_string(), "Bienvenue sur SENTIENT!".to_string());
    m.insert("greeting.goodbye".to_string(), "Au revoir!".to_string());
    m.insert("common.yes".to_string(), "Oui".to_string());
    m.insert("common.no".to_string(), "Non".to_string());
    m.insert("common.ok".to_string(), "OK".to_string());
    m.insert("common.cancel".to_string(), "Annuler".to_string());
    m.insert("common.save".to_string(), "Enregistrer".to_string());
    m.insert("common.delete".to_string(), "Supprimer".to_string());
    m.insert("common.loading".to_string(), "Chargement...".to_string());
    m.insert("error.general".to_string(), "Une erreur s'est produite. Veuillez réessayer.".to_string());
    m.insert("message.placeholder".to_string(), "Tapez votre message...".to_string());
    m.insert("message.send".to_string(), "Envoyer".to_string());
    m.insert("voice.listening".to_string(), "Écoute...".to_string());
    m.insert("settings.title".to_string(), "Paramètres".to_string());
    m.insert("settings.language".to_string(), "Langue".to_string());
    
    m
}

/// Spanish translations
pub fn spanish() -> HashMap<String, String> {
    let mut m = HashMap::new();
    
    m.insert("app.name".to_string(), "SENTIENT AI".to_string());
    m.insert("app.tagline".to_string(), "Tu asistente inteligente".to_string());
    m.insert("greeting.hello".to_string(), "¡Hola, {name}!".to_string());
    m.insert("greeting.welcome".to_string(), "¡Bienvenido a SENTIENT!".to_string());
    m.insert("greeting.goodbye".to_string(), "¡Adiós!".to_string());
    m.insert("common.yes".to_string(), "Sí".to_string());
    m.insert("common.no".to_string(), "No".to_string());
    m.insert("common.ok".to_string(), "OK".to_string());
    m.insert("common.cancel".to_string(), "Cancelar".to_string());
    m.insert("common.save".to_string(), "Guardar".to_string());
    m.insert("common.delete".to_string(), "Eliminar".to_string());
    m.insert("common.loading".to_string(), "Cargando...".to_string());
    m.insert("error.general".to_string(), "Ocurrió un error. Por favor, inténtalo de nuevo.".to_string());
    m.insert("message.placeholder".to_string(), "Escribe tu mensaje...".to_string());
    m.insert("message.send".to_string(), "Enviar".to_string());
    m.insert("voice.listening".to_string(), "Escuchando...".to_string());
    m.insert("settings.title".to_string(), "Configuración".to_string());
    m.insert("settings.language".to_string(), "Idioma".to_string());
    
    m
}

/// Japanese translations
pub fn japanese() -> HashMap<String, String> {
    let mut m = HashMap::new();
    
    m.insert("app.name".to_string(), "SENTIENT AI".to_string());
    m.insert("app.tagline".to_string(), "あなたのインテリジェントアシスタント".to_string());
    m.insert("greeting.hello".to_string(), "こんにちは、{name}さん！".to_string());
    m.insert("greeting.welcome".to_string(), "SENTIENTへようこそ！".to_string());
    m.insert("greeting.goodbye".to_string(), "さようなら！".to_string());
    m.insert("common.yes".to_string(), "はい".to_string());
    m.insert("common.no".to_string(), "いいえ".to_string());
    m.insert("common.ok".to_string(), "OK".to_string());
    m.insert("common.cancel".to_string(), "キャンセル".to_string());
    m.insert("common.save".to_string(), "保存".to_string());
    m.insert("common.delete".to_string(), "削除".to_string());
    m.insert("common.loading".to_string(), "読み込み中...".to_string());
    m.insert("error.general".to_string(), "エラーが発生しました。もう一度お試しください。".to_string());
    m.insert("message.placeholder".to_string(), "メッセージを入力...".to_string());
    m.insert("message.send".to_string(), "送信".to_string());
    m.insert("voice.listening".to_string(), "聞いています...".to_string());
    m.insert("settings.title".to_string(), "設定".to_string());
    m.insert("settings.language".to_string(), "言語".to_string());
    
    m
}

/// Chinese translations
pub fn chinese() -> HashMap<String, String> {
    let mut m = HashMap::new();
    
    m.insert("app.name".to_string(), "SENTIENT AI".to_string());
    m.insert("app.tagline".to_string(), "您的智能助手".to_string());
    m.insert("greeting.hello".to_string(), "你好，{name}！".to_string());
    m.insert("greeting.welcome".to_string(), "欢迎使用SENTIENT！".to_string());
    m.insert("greeting.goodbye".to_string(), "再见！".to_string());
    m.insert("common.yes".to_string(), "是".to_string());
    m.insert("common.no".to_string(), "否".to_string());
    m.insert("common.ok".to_string(), "确定".to_string());
    m.insert("common.cancel".to_string(), "取消".to_string());
    m.insert("common.save".to_string(), "保存".to_string());
    m.insert("common.delete".to_string(), "删除".to_string());
    m.insert("common.loading".to_string(), "加载中...".to_string());
    m.insert("error.general".to_string(), "发生错误。请重试。".to_string());
    m.insert("message.placeholder".to_string(), "输入您的消息...".to_string());
    m.insert("message.send".to_string(), "发送".to_string());
    m.insert("voice.listening".to_string(), "正在聆听...".to_string());
    m.insert("settings.title".to_string(), "设置".to_string());
    m.insert("settings.language".to_string(), "语言".to_string());
    
    m
}

/// Russian translations
pub fn russian() -> HashMap<String, String> {
    let mut m = HashMap::new();
    
    m.insert("app.name".to_string(), "SENTIENT AI".to_string());
    m.insert("app.tagline".to_string(), "Ваш умный помощник".to_string());
    m.insert("greeting.hello".to_string(), "Привет, {name}!".to_string());
    m.insert("greeting.welcome".to_string(), "Добро пожаловать в SENTIENT!".to_string());
    m.insert("greeting.goodbye".to_string(), "До свидания!".to_string());
    m.insert("common.yes".to_string(), "Да".to_string());
    m.insert("common.no".to_string(), "Нет".to_string());
    m.insert("common.ok".to_string(), "ОК".to_string());
    m.insert("common.cancel".to_string(), "Отмена".to_string());
    m.insert("common.save".to_string(), "Сохранить".to_string());
    m.insert("common.delete".to_string(), "Удалить".to_string());
    m.insert("common.loading".to_string(), "Загрузка...".to_string());
    m.insert("error.general".to_string(), "Произошла ошибка. Пожалуйста, попробуйте снова.".to_string());
    m.insert("message.placeholder".to_string(), "Введите сообщение...".to_string());
    m.insert("message.send".to_string(), "Отправить".to_string());
    m.insert("voice.listening".to_string(), "Слушаю...".to_string());
    m.insert("settings.title".to_string(), "Настройки".to_string());
    m.insert("settings.language".to_string(), "Язык".to_string());
    
    m
}
