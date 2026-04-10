package ai.sentient.plugin

import com.intellij.openapi.application.ApplicationManager
import com.intellij.openapi.components.Service
import com.intellij.openapi.diagnostic.Logger
import java.net.URI
import java.net.http.HttpClient
import java.net.http.HttpRequest
import java.net.http.HttpResponse

/**
 * SENTIENT OS API Client for JetBrains IDEs
 */
@Service
class SentientClient {
    private val LOG = Logger.getInstance(SentientClient::class.java)
    
    var apiUrl: String = "http://localhost:8080"
    var model: String = "gpt-4-turbo"
    var connected: Boolean = false
    
    private val httpClient: HttpClient = HttpClient.newBuilder()
        .version(HttpClient.Version.HTTP_1_1)
        .connectTimeout(java.time.Duration.ofSeconds(10))
        .build()
    
    companion object {
        fun getInstance(): SentientClient {
            return ApplicationManager.getApplication().getService(SentientClient::class.java)
        }
    }
    
    fun checkConnection(): Boolean {
        return try {
            val request = HttpRequest.newBuilder()
                .uri(URI.create("$apiUrl/health"))
                .timeout(java.time.Duration.ofSeconds(5))
                .GET()
                .build()
            
            val response = httpClient.send(request, HttpResponse.BodyHandlers.ofString())
            connected = response.statusCode() == 200
            connected
        } catch (e: Exception) {
            LOG.warn("Connection check failed: ${e.message}")
            connected = false
            false
        }
    }
    
    fun chat(messages: List<Message>): String {
        val requestBody = """
            {
                "messages": [
                    ${messages.joinToString(",") { 
                        """{"role": "${it.role}", "content": "${escapeJson(it.content)}"}""" 
                    }}
                ],
                "model": "$model",
                "stream": false,
                "max_tokens": 4096
            }
        """.trimIndent()
        
        val request = HttpRequest.newBuilder()
            .uri(URI.create("$apiUrl/api/chat"))
            .header("Content-Type", "application/json")
            .POST(HttpRequest.BodyPublishers.ofString(requestBody))
            .timeout(java.time.Duration.ofMinutes(2))
            .build()
        
        val response = httpClient.send(request, HttpResponse.BodyHandlers.ofString())
        
        if (response.statusCode() != 200) {
            throw Exception("Chat failed: ${response.statusCode()} - ${response.body()}")
        }
        
        return parseChatResponse(response.body())
    }
    
    fun explain(code: String, language: String): String {
        return chat(listOf(
            Message("system", "You are a code explanation expert. Explain the following code clearly and concisely."),
            Message("user", "Explain this $language code:\n\n```${language}\n$code\n```")
        ))
    }
    
    fun refactor(code: String, language: String): String {
        return chat(listOf(
            Message("system", "You are a code refactoring expert. Improve the code quality while maintaining functionality."),
            Message("user", "Refactor this $language code for better readability and performance:\n\n```${language}\n$code\n```")
        ))
    }
    
    fun fix(code: String, language: String): String {
        return chat(listOf(
            Message("system", "You are a code debugging expert. Find and fix issues in the code."),
            Message("user", "Find and fix bugs in this $language code:\n\n```${language}\n$code\n```")
        ))
    }
    
    fun generateTests(code: String, language: String): String {
        return chat(listOf(
            Message("system", "You are a test generation expert. Write comprehensive unit tests."),
            Message("user", "Generate unit tests for this $language code:\n\n```${language}\n$code\n```")
        ))
    }
    
    fun generateDocs(code: String, language: String): String {
        return chat(listOf(
            Message("system", "You are a documentation expert. Generate clear and comprehensive documentation."),
            Message("user", "Generate documentation for this $language code:\n\n```${language}\n$code\n```")
        ))
    }
    
    fun translate(code: String, sourceLang: String, targetLang: String): String {
        return chat(listOf(
            Message("system", "You are a code translation expert. Translate code from $sourceLang to $targetLang."),
            Message("user", "Translate this $sourceLang code to $targetLang:\n\n```${sourceLang}\n$code\n```")
        ))
    }
    
    fun optimize(code: String, language: String): String {
        return chat(listOf(
            Message("system", "You are a code optimization expert. Improve performance and efficiency."),
            Message("user", "Optimize this $language code for better performance:\n\n```${language}\n$code\n```")
        ))
    }
    
    fun review(code: String, language: String): String {
        return chat(listOf(
            Message("system", "You are a senior code reviewer. Provide detailed code review feedback."),
            Message("user", "Review this $language code and provide feedback:\n\n```${language}\n$code\n```")
        ))
    }
    
    fun generateCommitMessage(diff: String): String {
        return chat(listOf(
            Message("system", "You are a Git commit message expert. Generate concise, conventional commit messages."),
            Message("user", "Generate a commit message for these changes. Use conventional commits format:\n\n$diff")
        ))
    }
    
    fun getAvailableModels(): List<String> {
        return try {
            val request = HttpRequest.newBuilder()
                .uri(URI.create("$apiUrl/api/models"))
                .GET()
                .build()
            
            val response = httpClient.send(request, HttpResponse.BodyHandlers.ofString())
            parseModelsResponse(response.body())
        } catch (e: Exception) {
            getDefaultModels()
        }
    }
    
    private fun getDefaultModels(): List<String> {
        return listOf(
            "gpt-4-turbo", "gpt-4o", "gpt-3.5-turbo",
            "claude-3-opus", "claude-3-sonnet", "claude-3-haiku",
            "gemini-1.5-pro", "gemini-1.5-flash",
            "llama-3.1-70b", "llama-3.1-405b",
            "mixtral-8x7b", "qwen-2.5-72b", "gemma-4-27b"
        )
    }
    
    private fun parseChatResponse(json: String): String {
        // Simple JSON parsing - extract content from choices[0].message.content
        val contentMatch = """"content"\s*:\s*"((?:[^"\\]|\\.)*)"""".toRegex().find(json)
        return contentMatch?.groupValues?.get(1)?.unescapeJson() ?: json
    }
    
    private fun parseModelsResponse(json: String): List<String> {
        // Simple parsing - extract model IDs
        val modelMatches = """"id"\s*:\s*"([^"]+)"""".toRegex().findAll(json)
        return modelMatches.map { it.groupValues[1] }.toList()
    }
    
    private fun escapeJson(s: String): String {
        return s.replace("\\", "\\\\")
            .replace("\"", "\\\"")
            .replace("\n", "\\n")
            .replace("\r", "\\r")
            .replace("\t", "\\t")
    }
    
    private fun String.unescapeJson(): String {
        return this.replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t")
            .replace("\\\"", "\"")
            .replace("\\\\", "\\")
    }
}

data class Message(
    val role: String,
    val content: String
)
