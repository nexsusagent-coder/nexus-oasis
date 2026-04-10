package ai.sentient.plugin.actions

import ai.sentient.plugin.SentientClient
import com.intellij.notification.NotificationGroupManager
import com.intellij.notification.NotificationType
import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.actionSystem.CommonDataKeys
import com.intellij.openapi.application.ApplicationManager
import com.intellij.openapi.progress.ProgressIndicator
import com.intellij.openapi.progress.ProgressManager
import com.intellij.openapi.progress.Task
import com.intellij.openapi.project.Project

/**
 * Base action for SENTIENT code operations
 */
abstract class SentientAction(
    private val title: String,
    private val promptType: String
) : AnAction() {
    
    protected val client: SentientClient = SentientClient.getInstance()
    
    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return
        val editor = e.getData(CommonDataKeys.EDITOR) ?: return
        val selectedText = editor.selectionModel.selectedText
        
        if (selectedText.isNullOrBlank()) {
            showNotification(project, "Please select some code first", NotificationType.WARNING)
            return
        }
        
        val language = e.getData(CommonDataKeys.PSI_FILE)?.language?.id ?: "text"
        val document = editor.document
        
        ProgressManager.getInstance().run(object : Task.Backgroundable(project, title, true) {
            private var result: String? = null
            
            override fun run(indicator: ProgressIndicator) {
                result = processCode(selectedText, language)
            }
            
            override fun onSuccess() {
                result?.let { showResult(project, it) }
            }
            
            override fun onThrowable(error: Throwable) {
                showNotification(project, "Error: ${error.message}", NotificationType.ERROR)
            }
        })
    }
    
    override fun update(e: AnActionEvent) {
        val editor = e.getData(CommonDataKeys.EDITOR)
        e.presentation.isEnabled = editor?.selectionModel?.hasSelection() == true
    }
    
    protected abstract fun processCode(code: String, language: String): String
    
    protected open fun showResult(project: Project, result: String) {
        showNotification(project, result.take(200) + if (result.length > 200) "..." else "", NotificationType.INFORMATION)
    }
    
    protected fun showNotification(project: Project, content: String, type: NotificationType) {
        NotificationGroupManager.getInstance()
            .getNotificationGroup("SENTIENT OS")
            .createNotification(content, type)
            .notify(project)
    }
}

// Concrete actions

class ExplainAction : SentientAction("Explaining code...", "explain") {
    override fun processCode(code: String, language: String) = client.explain(code, language)
}

class RefactorAction : SentientAction("Refactoring code...", "refactor") {
    override fun processCode(code: String, language: String) = client.refactor(code, language)
}

class FixAction : SentientAction("Fixing code...", "fix") {
    override fun processCode(code: String, language: String) = client.fix(code, language)
}

class TestAction : SentientAction("Generating tests...", "test") {
    override fun processCode(code: String, language: String) = client.generateTests(code, language)
}

class DocumentAction : SentientAction("Generating documentation...", "document") {
    override fun processCode(code: String, language: String) = client.generateDocs(code, language)
}

class OptimizeAction : SentientAction("Optimizing code...", "optimize") {
    override fun processCode(code: String, language: String) = client.optimize(code, language)
}

class ReviewAction : SentientAction("Reviewing code...", "review") {
    override fun processCode(code: String, language: String) = client.review(code, language)
}

class TranslateAction : SentientAction("Translating code...", "translate") {
    override fun processCode(code: String, language: String): String {
        // Show language selector dialog - simplified for now
        return client.translate(code, language, "Python") // Default target
    }
}

class ChatAction : AnAction() {
    override fun actionPerformed(e: AnActionEvent) {
        // Open the tool window
        e.project?.let { project ->
            com.intellij.openapi.wm.ToolWindowManager.getInstance(project)
                .getToolWindow("SENTIENT OS")
                ?.activate(null)
        }
    }
}

class CommitMessageAction : AnAction() {
    private val client = SentientClient.getInstance()
    
    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return
        
        ProgressManager.getInstance().run(object : Task.Backgroundable(project, "Generating commit message...", true) {
            private var message: String? = null
            
            override fun run(indicator: ProgressIndicator) {
                // Get staged changes - simplified
                message = client.generateCommitMessage("Staged changes")
            }
            
            override fun onSuccess() {
                message?.let { msg ->
                    // Copy to clipboard
                    com.intellij.openapi.ide.CopyPasteManager.getInstance().setContents(
                        java.awt.datatransfer.StringSelection(msg)
                    )
                    NotificationGroupManager.getInstance()
                        .getNotificationGroup("SENTIENT OS")
                        .createNotification("Commit message copied to clipboard!", NotificationType.INFORMATION)
                        .notify(project)
                }
            }
        })
    }
}
