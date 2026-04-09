package ai.sentient.android

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import kotlinx.coroutines.launch

/// ─── Main Activity ───
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            SentientTheme {
                SentientApp()
            }
        }
    }
}

/// ─── Theme ───
@Composable
fun SentientTheme(content: @Composable () -> Unit) {
    val darkColors = darkColorScheme(
        primary = Color(0xFF6366F1),
        primaryContainer = Color(0xFF4F46E5),
        secondary = Color(0xFFA855F7),
        background = Color(0xFF0F0F1A),
        surface = Color(0xFF1A1A2E),
        onBackground = Color.White,
        onSurface = Color.White,
    )
    
    MaterialTheme(
        colorScheme = darkColors,
        typography = Typography(),
        content = content
    )
}

/// ─── Main App ───
@Composable
fun SentientApp() {
    val viewModel: SentientViewModel = viewModel()
    var selectedTab by remember { mutableStateOf(0) }
    
    Scaffold(
        bottomBar = {
            NavigationBar {
                NavigationBarItem(
                    selected = selectedTab == 0,
                    onClick = { selectedTab = 0 },
                    icon = { Icon(Icons.Default.Chat, "Sohbet") },
                    label = { Text("Sohbet") }
                )
                NavigationBarItem(
                    selected = selectedTab == 1,
                    onClick = { selectedTab = 1 },
                    icon = { Icon(Icons.Default.Devices, "Kanallar") },
                    label = { Text("Kanallar") }
                )
                NavigationBarItem(
                    selected = selectedTab == 2,
                    onClick = { selectedTab = 2 },
                    icon = { Icon(Icons.Default.Mic, "Ses") },
                    label = { Text("Ses") }
                )
                NavigationBarItem(
                    selected = selectedTab == 3,
                    onClick = { selectedTab = 3 },
                    icon = { Icon(Icons.Default.Settings, "Ayarlar") },
                    label = { Text("Ayarlar") }
                )
            }
        }
    ) { padding ->
        Box(modifier = Modifier.padding(padding)) {
            when (selectedTab) {
                0 -> ChatScreen(viewModel)
                1 -> ChannelsScreen()
                2 -> VoiceScreen(viewModel)
                3 -> SettingsScreen()
            }
        }
    }
}

/// ─── Chat Screen ───
@Composable
fun ChatScreen(viewModel: SentientViewModel) {
    var inputText by remember { mutableStateOf(TextFieldValue("")) }
    val messages by viewModel.messages.collectAsState()
    val loading by viewModel.loading.collectAsState()
    val listState = rememberLazyListState()
    val coroutineScope = rememberCoroutineScope()
    
    Column(modifier = Modifier.fillMaxSize()) {
        // Messages
        LazyColumn(
            state = listState,
            modifier = Modifier
                .weight(1f)
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp)
        ) {
            items(messages) { message ->
                MessageBubble(message = message)
            }
            
            if (loading) {
                item {
                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        horizontalArrangement = Arrangement.Center
                    ) {
                        CircularProgressIndicator(
                            modifier = Modifier.size(24.dp),
                            color = MaterialTheme.colorScheme.primary
                        )
                    }
                }
            }
        }
        
        // Input
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp)
                .imePadding(),
            horizontalArrangement = Arrangement.spacedBy(12.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            OutlinedTextField(
                value = inputText,
                onValueChange = { inputText = it },
                modifier = Modifier.weight(1f),
                placeholder = { Text("Mesajınızı yazın...") },
                shape = RoundedCornerShape(24.dp)
            )
            
            FilledIconButton(
                onClick = {
                    if (inputText.text.isNotBlank()) {
                        viewModel.sendMessage(inputText.text)
                        inputText = TextFieldValue("")
                        coroutineScope.launch {
                            listState.animateScrollToItem(messages.size)
                        }
                    }
                },
                enabled = inputText.text.isNotBlank() && !loading
            ) {
                Icon(Icons.Default.Send, "Gönder")
            }
        }
    }
}

/// ─── Message Bubble ───
@Composable
fun MessageBubble(message: ChatMessage) {
    val isUser = message.role == "user"
    
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = if (isUser) Arrangement.End else Arrangement.Start
    ) {
        Surface(
            modifier = Modifier.widthIn(max = 280.dp),
            shape = RoundedCornerShape(
                topStart = 16.dp,
                topEnd = 16.dp,
                bottomStart = if (isUser) 16.dp else 4.dp,
                bottomEnd = if (isUser) 4.dp else 16.dp
            ),
            color = if (isUser) 
                MaterialTheme.colorScheme.primary.copy(alpha = 0.2f)
            else 
                MaterialTheme.colorScheme.surfaceVariant
        ) {
            Column(modifier = Modifier.padding(12.dp)) {
                Text(
                    text = message.content,
                    style = MaterialTheme.typography.bodyLarge
                )
                Text(
                    text = message.timestamp,
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.6f),
                    modifier = Modifier.padding(top = 4.dp)
                )
            }
        }
    }
}

/// ─── Channels Screen ───
@Composable
fun ChannelsScreen() {
    val channels = listOf(
        Channel("Telegram", true, 5),
        Channel("Discord", true, 3),
        Channel("WhatsApp", false, 0),
        Channel("Signal", false, 0),
    )
    
    LazyColumn(
        modifier = Modifier.fillMaxSize(),
        contentPadding = PaddingValues(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp)
    ) {
        items(channels) { channel ->
            ChannelCard(channel)
        }
    }
}

@Composable
fun ChannelCard(channel: Channel) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        shape = RoundedCornerShape(16.dp)
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Icon(
                imageVector = when (channel.name) {
                    "Telegram" -> Icons.Default.Send
                    "Discord" -> Icons.Default.Gamepad
                    else -> Icons.Default.Message
                },
                contentDescription = channel.name,
                modifier = Modifier.size(32.dp),
                tint = if (channel.connected) Color(0xFF22C55E) else Color.Gray
            )
            
            Spacer(modifier = Modifier.width(16.dp))
            
            Text(
                text = channel.name,
                style = MaterialTheme.typography.titleMedium,
                modifier = Modifier.weight(1f)
            )
            
            Text(
                text = if (channel.connected) "Bağlı" else "Bağlı değil",
                style = MaterialTheme.typography.labelMedium,
                color = if (channel.connected) Color(0xFF22C55E) else Color.Gray
            )
            
            if (channel.unread > 0) {
                Spacer(modifier = Modifier.width(8.dp))
                Badge { Text(channel.unread.toString()) }
            }
        }
    }
}

/// ─── Voice Screen ───
@Composable
fun VoiceScreen(viewModel: SentientViewModel) {
    val isListening by viewModel.isListening.collectAsState()
    
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(32.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        // Mic Button
        Box(
            modifier = Modifier
                .size(200.dp)
                .background(
                    brush = Brush.radialGradient(
                        colors = if (isListening)
                            listOf(Color(0xFFEF4444), Color(0xFFF97316))
                        else
                            listOf(Color(0xFF6366F1), Color(0xFFA855F7))
                    ),
                    shape = RoundedCornerShape(100.dp)
                ),
            contentAlignment = Alignment.Center
        ) {
            IconButton(
                onClick = { viewModel.toggleVoice() },
                modifier = Modifier.size(200.dp)
            ) {
                Icon(
                    imageVector = if (isListening) Icons.Default.GraphicEq else Icons.Default.Mic,
                    contentDescription = "Ses",
                    modifier = Modifier.size(80.dp),
                    tint = Color.White
                )
            }
        }
        
        Spacer(modifier = Modifier.height(32.dp))
        
        Text(
            text = if (isListening) "Dinliyorum..." else "Sesli komut için dokunun",
            style = MaterialTheme.typography.titleMedium,
            color = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.7f)
        )
        
        Spacer(modifier = Modifier.height(48.dp))
        
        Column(horizontalAlignment = Alignment.CenterHorizontally) {
            Text(
                text = "Uyandırma kelimesi:",
                style = MaterialTheme.typography.labelLarge
            )
            Text(
                text = "\"Hey SENTIENT\"",
                style = MaterialTheme.typography.headlineMedium,
                color = MaterialTheme.colorScheme.primary
            )
        }
    }
}

/// ─── Settings Screen ───
@Composable
fun SettingsScreen() {
    var apiKey by remember { mutableStateOf("") }
    var serverUrl by remember { mutableStateOf("https://api.sentient.ai") }
    var voiceEnabled by remember { mutableStateOf(true) }
    
    LazyColumn(
        modifier = Modifier.fillMaxSize(),
        contentPadding = PaddingValues(16.dp)
    ) {
        item {
            Text("API", style = MaterialTheme.typography.labelLarge)
            Spacer(modifier = Modifier.height(8.dp))
            OutlinedTextField(
                value = apiKey,
                onValueChange = { apiKey = it },
                label = { Text("API Key") },
                modifier = Modifier.fillMaxWidth()
            )
            Spacer(modifier = Modifier.height(8.dp))
            OutlinedTextField(
                value = serverUrl,
                onValueChange = { serverUrl = it },
                label = { Text("Server URL") },
                modifier = Modifier.fillMaxWidth()
            )
        }
        
        item {
            Spacer(modifier = Modifier.height(24.dp))
            Text("Ses", style = MaterialTheme.typography.labelLarge)
            Spacer(modifier = Modifier.height(8.dp))
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Text("Ses Etkin")
                Switch(checked = voiceEnabled, onCheckedChange = { voiceEnabled = it })
            }
        }
        
        item {
            Spacer(modifier = Modifier.height(24.dp))
            Text("Hakkında", style = MaterialTheme.typography.labelLarge)
            Spacer(modifier = Modifier.height(8.dp))
            ListItem(
                headlineContent = { Text("Versiyon") },
                trailingContent = { Text("0.1.0", color = Color.Gray) }
            )
        }
    }
}

/// ─── Data Models ───
data class ChatMessage(
    val id: String,
    val role: String,
    val content: String,
    val timestamp: String
)

data class Channel(
    val name: String,
    val connected: Boolean,
    val unread: Int
)
