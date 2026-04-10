package ai.sentient.plugin.statusbar

import ai.sentient.plugin.SentientClient
import com.intellij.openapi.project.Project
import com.intellij.openapi.wm.StatusBar
import com.intellij.openapi.wm.StatusBarWidget
import com.intellij.openapi.wm.impl.status.TextWidget
import com.intellij.util.Consumer
import java.awt.event.MouseEvent
import javax.swing.Icon

/**
 * Status bar widget for SENTIENT OS
 */
class SentientStatusBarWidget(private val project: Project) : TextWidget("SENTIENT_STATUS") {
    
    private val client = SentientClient.getInstance()
    
    override fun ID(): String = "SENTIENT_STATUS"
    
    override fun getPresentation(): StatusBarWidget.WidgetPresentation {
        return object : StatusBarWidget.TextPresentation {
            override fun getText(): String {
                return if (client.connected) {
                    "🤖 SENTIENT: ${client.model}"
                } else {
                    "🤖 SENTIENT: Disconnected"
                }
            }
            
            override fun getTooltipText(): String {
                return if (client.connected) {
                    "SENTIENT OS Connected - Click to change model"
                } else {
                    "SENTIENT OS Disconnected - Click to configure"
                }
            }
            
            override fun getClickConsumer(): Consumer<MouseEvent>? {
                return Consumer { _ ->
                    // Open settings or model selector
                    if (client.connected) {
                        showModelSelector()
                    } else {
                        showSettings()
                    }
                }
            }
        }
    }
    
    private fun showModelSelector() {
        val models = client.getAvailableModels()
        val selected = com.intellij.openapi.ui.Messages.showEditableChooseDialog(
            "Select Model",
            "SENTIENT OS Model",
            com.intellij.openapi.ui.Messages.getQuestionIcon(),
            models.toTypedArray(),
            client.model,
            null
        )
        
        if (selected != null) {
            client.model = selected
            update()
        }
    }
    
    private fun showSettings() {
        com.intellij.openapi.options.ShowSettingsUtil.getInstance()
            .showSettingsDialog(project, "SENTIENT OS")
    }
    
    fun update() {
        // Trigger repaint
        myStatusBar?.updateWidget(ID())
    }
}

class SentientStatusBarFactory : StatusBarWidgetFactory {
    override fun getId(): String = "SENTIENT_STATUS"
    
    override fun getDisplayName(): String = "SENTIENT OS Status"
    
    override fun isAvailable(project: Project): Boolean = true
    
    override fun createWidget(project: Project): StatusBarWidget {
        return SentientStatusBarWidget(project)
    }
    
    override fun canBeEnabledOn(statusBar: StatusBar): Boolean = true
}
