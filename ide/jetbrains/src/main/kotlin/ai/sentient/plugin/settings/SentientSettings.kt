package ai.sentient.plugin.settings

import com.intellij.openapi.application.ApplicationManager
import com.intellij.openapi.components.PersistentStateComponent
import com.intellij.openapi.components.Service
import com.intellij.openapi.components.State
import com.intellij.openapi.components.Storage

/**
 * Persistent settings for SENTIENT OS plugin
 */
@Service
@State(
    name = "SentientSettings",
    storage = [Storage("sentient_settings.xml")]
)
class SentientSettings : PersistentStateComponent<SentientSettings.State> {
    
    data class State(
        var apiUrl: String = "http://localhost:8080",
        var model: String = "gpt-4-turbo",
        var maxTokens: Int = 4096,
        var temperature: Double = 0.7,
        var streaming: Boolean = true,
        var language: String = "en"
    )
    
    private var state = State()
    
    var apiUrl: String
        get() = state.apiUrl
        set(value) { state.apiUrl = value }
    
    var model: String
        get() = state.model
        set(value) { state.model = value }
    
    var maxTokens: Int
        get() = state.maxTokens
        set(value) { state.maxTokens = value }
    
    var temperature: Double
        get() = state.temperature
        set(value) { state.temperature = value }
    
    var streaming: Boolean
        get() = state.streaming
        set(value) { state.streaming = value }
    
    var language: String
        get() = state.language
        set(value) { state.language = value }
    
    override fun getState(): State = state
    
    override fun loadState(state: State) {
        this.state = state
    }
    
    companion object {
        fun getInstance(): SentientSettings {
            return ApplicationManager.getApplication().getService(SentientSettings::class.java)
        }
    }
}
