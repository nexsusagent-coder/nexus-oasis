package ai.sentient.plugin.intentions

import ai.sentient.plugin.SentientClient
import com.intellij.codeInsight.intention.IntentionAction
import com.intellij.codeInsight.intention.PsiElementBaseIntentionAction
import com.intellij.openapi.editor.Editor
import com.intellij.openapi.project.Project
import com.intellij.psi.PsiElement

/**
 * Code intentions for quick actions
 */
class ExplainIntention : PsiElementBaseIntentionAction(), IntentionAction {
    private val client = SentientClient.getInstance()
    
    override fun getFamilyName() = "SENTIENT OS"
    override fun getText() = "Explain with SENTIENT"
    
    override fun isAvailable(project: Project, editor: Editor?, element: PsiElement): Boolean {
        return editor?.selectionModel?.hasSelection() == true
    }
    
    override fun invoke(project: Project, editor: Editor?, element: PsiElement) {
        val selectedText = editor?.selectionModel?.selectedText ?: return
        val language = element.language.id
        
        com.intellij.openapi.progress.ProgressManager.getInstance().run(
            object : com.intellij.openapi.progress.Task.Backgroundable(project, "Explaining code...", true) {
                private var result: String? = null
                
                override fun run(indicator: com.intellij.openapi.progress.ProgressIndicator) {
                    result = client.explain(selectedText, language)
                }
                
                override fun onSuccess() {
                    result?.let {
                        com.intellij.notification.NotificationGroupManager.getInstance()
                            .getNotificationGroup("SENTIENT OS")
                            .createNotification(it.take(500), com.intellij.notification.NotificationType.INFORMATION)
                            .notify(project)
                    }
                }
            }
        )
    }
}

class RefactorIntention : PsiElementBaseIntentionAction(), IntentionAction {
    private val client = SentientClient.getInstance()
    
    override fun getFamilyName() = "SENTIENT OS"
    override fun getText() = "Refactor with SENTIENT"
    
    override fun isAvailable(project: Project, editor: Editor?, element: PsiElement): Boolean {
        return editor?.selectionModel?.hasSelection() == true
    }
    
    override fun invoke(project: Project, editor: Editor?, element: PsiElement) {
        val selectedText = editor?.selectionModel?.selectedText ?: return
        val language = element.language.id
        val document = editor.document
        
        com.intellij.openapi.progress.ProgressManager.getInstance().run(
            object : com.intellij.openapi.progress.Task.Backgroundable(project, "Refactoring code...", true) {
                private var result: String? = null
                
                override fun run(indicator: com.intellij.openapi.progress.ProgressIndicator) {
                    result = client.refactor(selectedText, language)
                }
                
                override fun onSuccess() {
                    result?.let { refactored ->
                        // Apply refactored code
                        com.intellij.openapi.command.WriteCommandAction.runWriteCommandAction(project) {
                            editor.selectionModel.let { selection ->
                                document.replaceString(
                                    selection.selectionStart,
                                    selection.selectionEnd,
                                    extractCode(refactored)
                                )
                            }
                        }
                    }
                }
            }
        )
    }
    
    private fun extractCode(text: String): String {
        val codeBlock = """```[\w]*\n([\s\S]*?)```""".toRegex().find(text)
        return codeBlock?.groupValues?.get(1)?.trim() ?: text
    }
}

class FixIntention : PsiElementBaseIntentionAction(), IntentionAction {
    private val client = SentientClient.getInstance()
    
    override fun getFamilyName() = "SENTIENT OS"
    override fun getText() = "Fix with SENTIENT"
    
    override fun isAvailable(project: Project, editor: Editor?, element: PsiElement): Boolean {
        return editor?.selectionModel?.hasSelection() == true
    }
    
    override fun invoke(project: Project, editor: Editor?, element: PsiElement) {
        val selectedText = editor?.selectionModel?.selectedText ?: return
        val language = element.language.id
        
        com.intellij.openapi.progress.ProgressManager.getInstance().run(
            object : com.intellij.openapi.progress.Task.Backgroundable(project, "Fixing code...", true) {
                private var result: String? = null
                
                override fun run(indicator: com.intellij.openapi.progress.ProgressIndicator) {
                    result = client.fix(selectedText, language)
                }
                
                override fun onSuccess() {
                    result?.let {
                        com.intellij.notification.NotificationGroupManager.getInstance()
                            .getNotificationGroup("SENTIENT OS")
                            .createNotification(it.take(500), com.intellij.notification.NotificationType.INFORMATION)
                            .notify(project)
                    }
                }
            }
        )
    }
}

class DocumentIntention : PsiElementBaseIntentionAction(), IntentionAction {
    private val client = SentientClient.getInstance()
    
    override fun getFamilyName() = "SENTIENT OS"
    override fun getText() = "Document with SENTIENT"
    
    override fun isAvailable(project: Project, editor: Editor?, element: PsiElement): Boolean {
        return editor?.selectionModel?.hasSelection() == true
    }
    
    override fun invoke(project: Project, editor: Editor?, element: PsiElement) {
        val selectedText = editor?.selectionModel?.selectedText ?: return
        val language = element.language.id
        
        com.intellij.openapi.progress.ProgressManager.getInstance().run(
            object : com.intellij.openapi.progress.Task.Backgroundable(project, "Generating documentation...", true) {
                private var result: String? = null
                
                override fun run(indicator: com.intellij.openapi.progress.ProgressIndicator) {
                    result = client.generateDocs(selectedText, language)
                }
                
                override fun onSuccess() {
                    result?.let { docs ->
                        // Insert documentation above selection
                        val document = editor!!.document
                        val startLine = document.getLineNumber(editor.selectionModel.selectionStart)
                        val insertOffset = document.getLineStartOffset(startLine)
                        
                        com.intellij.openapi.command.WriteCommandAction.runWriteCommandAction(project) {
                            document.insertString(insertOffset, docs + "\n")
                        }
                    }
                }
            }
        )
    }
}

class TestIntention : PsiElementBaseIntentionAction(), IntentionAction {
    private val client = SentientClient.getInstance()
    
    override fun getFamilyName() = "SENTIENT OS"
    override fun getText() = "Generate Tests with SENTIENT"
    
    override fun isAvailable(project: Project, editor: Editor?, element: PsiElement): Boolean {
        return editor?.selectionModel?.hasSelection() == true
    }
    
    override fun invoke(project: Project, editor: Editor?, element: PsiElement) {
        val selectedText = editor?.selectionModel?.selectedText ?: return
        val language = element.language.id
        
        com.intellij.openapi.progress.ProgressManager.getInstance().run(
            object : com.intellij.openapi.progress.Task.Backgroundable(project, "Generating tests...", true) {
                private var result: String? = null
                
                override fun run(indicator: com.intellij.openapi.progress.ProgressIndicator) {
                    result = client.generateTests(selectedText, language)
                }
                
                override fun onSuccess() {
                    result?.let {
                        // Create new file with tests - simplified notification
                        com.intellij.notification.NotificationGroupManager.getInstance()
                            .getNotificationGroup("SENTIENT OS")
                            .createNotification("Tests generated! Check the tool window.", com.intellij.notification.NotificationType.INFORMATION)
                            .notify(project)
                    }
                }
            }
        )
    }
}
