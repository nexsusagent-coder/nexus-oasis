package ai.sentient.android

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import java.text.SimpleDateFormat
import java.util.*

/// ─── View Model ───
class SentientViewModel : ViewModel() {
    private val _messages = MutableStateFlow<List<ChatMessage>>(emptyList())
    val messages: StateFlow<List<ChatMessage>> = _messages
    
    private val _loading = MutableStateFlow(false)
    val loading: StateFlow<Boolean> = _loading
    
    private val _isListening = MutableStateFlow(false)
    val isListening: StateFlow<Boolean> = _isListening
    
    private val dateFormat = SimpleDateFormat("HH:mm", Locale.getDefault())
    
    fun sendMessage(text: String) {
        val userMessage = ChatMessage(
            id = UUID.randomUUID().toString(),
            role = "user",
            content = text,
            timestamp = dateFormat.format(Date())
        )
        
        _messages.value = _messages.value + userMessage
        _loading.value = true
        
        // Simulate response
        viewModelScope.launch {
            kotlinx.coroutines.delay(1500)
            
            val response = ChatMessage(
                id = UUID.randomUUID().toString(),
                role = "assistant",
                content = "Merhaba! Ben SENTIENT. Size nasıl yardımcı olabilirim?",
                timestamp = dateFormat.format(Date())
            )
            
            _messages.value = _messages.value + response
            _loading.value = false
        }
    }
    
    fun toggleVoice() {
        _isListening.value = !_isListening.value
        
        if (_isListening.value) {
            // Auto-stop after 5 seconds
            viewModelScope.launch {
                kotlinx.coroutines.delay(5000)
                _isListening.value = false
            }
        }
    }
}
