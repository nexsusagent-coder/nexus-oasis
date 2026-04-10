package ai.sentient.plugin.settings

import com.intellij.openapi.options.Configurable
import com.intellij.openapi.ui.ComboBox
import com.intellij.ui.components.JBTextField
import com.intellij.util.ui.FormBuilder
import java.awt.Dimension
import javax.swing.*

/**
 * SENTIENT OS Settings Configuration
 */
class SentientSettingsConfigurable : Configurable {
    private var apiUrlField: JBTextField? = null
    private var modelComboBox: ComboBox<String>? = null
    private var maxTokensField: JBTextField? = null
    private var temperatureField: JBTextField? = null
    private var streamingCheckBox: JCheckBox? = null
    
    private val settings: SentientSettings
        get() = SentientSettings.getInstance()
    
    override fun getDisplayName(): String = "SENTIENT OS"
    
    override fun createComponent(): JComponent {
        apiUrlField = JBTextField(settings.apiUrl, 40)
        modelComboBox = ComboBox(arrayOf(
            "gpt-4-turbo", "gpt-4o", "gpt-3.5-turbo",
            "claude-3-opus", "claude-3-sonnet", "claude-3-haiku",
            "gemini-1.5-pro", "gemini-1.5-flash",
            "llama-3.1-70b", "llama-3.1-405b",
            "mixtral-8x7b", "qwen-2.5-72b", "gemma-4-27b",
            "o1-preview", "o1-mini"
        )).apply {
            selectedItem = settings.model
        }
        maxTokensField = JBTextField(settings.maxTokens.toString(), 10)
        temperatureField = JBTextField(settings.temperature.toString(), 10)
        streamingCheckBox = JCheckBox("Enable streaming responses").apply {
            isSelected = settings.streaming
        }
        
        return FormBuilder.createFormBuilder()
            .addLabeledComponent("API URL:", apiUrlField!!)
            .addLabeledComponent("Model:", modelComboBox!!)
            .addLabeledComponent("Max Tokens:", maxTokensField!!)
            .addLabeledComponent("Temperature:", temperatureField!!)
            .addComponent(streamingCheckBox!!)
            .addComponentFillVertically(JPanel(), 0)
            .panel
    }
    
    override fun isModified(): Boolean {
        return apiUrlField?.text != settings.apiUrl ||
                modelComboBox?.selectedItem as? String != settings.model ||
                maxTokensField?.text?.toIntOrNull() != settings.maxTokens ||
                temperatureField?.text?.toDoubleOrNull() != settings.temperature ||
                streamingCheckBox?.isSelected != settings.streaming
    }
    
    override fun apply() {
        settings.apiUrl = apiUrlField?.text ?: "http://localhost:8080"
        settings.model = modelComboBox?.selectedItem as? String ?: "gpt-4-turbo"
        settings.maxTokens = maxTokensField?.text?.toIntOrNull() ?: 4096
        settings.temperature = temperatureField?.text?.toDoubleOrNull() ?: 0.7
        settings.streaming = streamingCheckBox?.isSelected ?: true
        
        // Update client
        val client = ai.sentient.plugin.SentientClient.getInstance()
        client.apiUrl = settings.apiUrl
        client.model = settings.model
    }
    
    override fun reset() {
        apiUrlField?.text = settings.apiUrl
        modelComboBox?.selectedItem = settings.model
        maxTokensField?.text = settings.maxTokens.toString()
        temperatureField?.text = settings.temperature.toString()
        streamingCheckBox?.isSelected = settings.streaming
    }
}
