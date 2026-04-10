package ai.sentient.plugin.ui

import ai.sentient.plugin.SentientClient
import ai.sentient.plugin.settings.SentientSettings
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.Messages
import com.intellij.openapi.wm.ToolWindow
import com.intellij.openapi.wm.ToolWindowFactory
import com.intellij.ui.components.JBScrollPane
import com.intellij.ui.components.JBTextArea
import java.awt.BorderLayout
import java.awt.Dimension
import java.awt.event.ActionEvent
import java.awt.event.KeyAdapter
import java.awt.event.KeyEvent
import javax.swing.*

/**
 * SENTIENT OS Tool Window Factory
 */
class SentientToolWindowFactory : ToolWindowFactory {
    
    override fun createToolWindowContent(project: Project, toolWindow: ToolWindow) {
        val panel = SentientChatPanel(project)
        val content = toolWindow.contentManager.factory.createContent(panel, "", false)
        toolWindow.contentManager.addContent(content)
    }
}

class SentientChatPanel(private val project: Project) : JPanel(BorderLayout()) {
    
    private val client: SentientClient = SentientClient.getInstance()
    private val chatArea = JBTextArea().apply {
        isEditable = false
        lineWrap = true
        wrapStyleWord = true
        font = font.deriveFont(14f)
    }
    private val inputField = JBTextArea().apply {
        lineWrap = true
        wrapStyleWord = true
        font = font.deriveFont(14f)
        rows = 3
    }
    private val sendButton = JButton("Send")
    private val modelLabel = JLabel()
    
    private val messages = mutableListOf<ai.sentient.plugin.Message>()
    
    init {
        setupUI()
        updateStatusLabel()
    }
    
    private fun setupUI() {
        // Chat display
        val scrollPane = JBScrollPane(chatArea).apply {
            preferredSize = Dimension(400, 300)
        }
        add(scrollPane, BorderLayout.CENTER)
        
        // Input panel
        val inputPanel = JPanel(BorderLayout()).apply {
            val inputScrollPane = JBScrollPane(inputField)
            add(inputScrollPane, BorderLayout.CENTER)
            add(sendButton, BorderLayout.EAST)
            
            // Model selector
            val topPanel = JPanel(BorderLayout())
            topPanel.add(modelLabel, BorderLayout.WEST)
            val modelButton = JButton("Change Model")
            modelButton.addActionListener { showModelSelector() }
            topPanel.add(modelButton, BorderLayout.EAST)
            add(topPanel, BorderLayout.NORTH)
        }
        add(inputPanel, BorderLayout.SOUTH)
        
        // Button action
        sendButton.addActionListener(::sendMessage)
        
        // Enter to send (Shift+Enter for new line)
        inputField.addKeyListener(object : KeyAdapter() {
            override fun keyPressed(e: KeyEvent) {
                if (e.keyCode == KeyEvent.VK_ENTER && !e.isShiftDown) {
                    e.consume()
                    sendMessage()
                }
            }
        })
        
        // Welcome message
        appendMessage("assistant", "🤖 Welcome to SENTIENT OS! How can I help you today?")
    }
    
    private fun sendMessage(e: ActionEvent? = null) {
        val text = inputField.text.trim()
        if (text.isEmpty()) return
        
        inputField.text = ""
        appendMessage("user", text)
        sendButton.isEnabled = false
        
        messages.add(ai.sentient.plugin.Message("user", text))
        
        // Run in background
        SwingUtilities.invokeLater {
            try {
                val response = client.chat(messages)
                appendMessage("assistant", response)
                messages.add(ai.sentient.plugin.Message("assistant", response))
            } catch (ex: Exception) {
                appendMessage("assistant", "❌ Error: ${ex.message}")
            } finally {
                sendButton.isEnabled = true
            }
        }
    }
    
    private fun appendMessage(role: String, content: String) {
        val prefix = if (role == "user") "👤 You: " else "🤖 SENTIENT: "
        chatArea.append("$prefix$content\n\n")
        chatArea.caretPosition = chatArea.document.length
    }
    
    private fun updateStatusLabel() {
        modelLabel.text = "Model: ${client.model}"
    }
    
    private fun showModelSelector() {
        val models = client.getAvailableModels().toTypedArray()
        val selected = Messages.showEditableChooseDialog(
            "Select LLM Model",
            "Model Selection",
            Messages.getQuestionIcon(),
            models,
            client.model,
            null
        )
        
        if (selected != null) {
            client.model = selected
            SentientSettings.getInstance().model = selected
            updateStatusLabel()
        }
    }
}
