//! ─── DASHBOARD ASSETS ───
//!
//! Enterprise-grade Command Center UI with TailwindCSS
//! Skills/Tool Hub for Asimilated Competitor Capabilities
//! Dark distopian theme, Real WebSocket state management

pub struct DashboardAssets;

impl DashboardAssets {
    /// Ana HTML sayfası - Enterprise Command Center
    pub fn index_html() -> String {
        r##"<!DOCTYPE html>
<html lang="tr" class="dark">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SENTIENT // Command Center</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <script>
        tailwind.config = {
            darkMode: 'class',
            theme: {
                extend: {
                    colors: {
                        'sentient': {
                            900: '#0a0a0f',
                            800: '#0f0f1a',
                            700: '#151525',
                            600: '#1a1a2e',
                            500: '#252540',
                            400: '#3a3a5a',
                            300: '#4a4a6a',
                            200: '#6a6a8a',
                            100: '#8a8aaa',
                        },
                        'cyber': {
                            primary: '#00fff2',
                            secondary: '#ff00ff',
                            warning: '#ffaa00',
                            danger: '#ff3366',
                            success: '#00ff88',
                        }
                    }
                }
            }
        }
    </script>
    <style>
        @import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600;700&family=Inter:wght@300;400;500;600;700&display=swap');
        
        body { font-family: 'Inter', sans-serif; background: linear-gradient(135deg, #0a0a0f 0%, #0f0f1a 50%, #0a0a0f 100%); }
        .font-mono { font-family: 'JetBrains Mono', monospace; }
        .glow-cyber { text-shadow: 0 0 10px #00fff2, 0 0 20px #00fff280; }
        .glow-success { text-shadow: 0 0 10px #00ff88, 0 0 20px #00ff8880; }
        .glow-danger { text-shadow: 0 0 10px #ff3366, 0 0 20px #ff336680; }
        .card-glow { box-shadow: 0 0 0 1px #00fff210, 0 4px 20px #00000050; }
        .sidebar-item:hover, .sidebar-item.active { background: linear-gradient(90deg, #00fff210 0%, transparent 100%); border-left: 2px solid #00fff2; }
        .pulse-dot { animation: pulse 2s infinite; }
        @keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.5; } }
        .grid-lines { background-image: linear-gradient(#00fff205 1px, transparent 1px), linear-gradient(90deg, #00fff205 1px, transparent 1px); background-size: 50px 50px; }
        .skill-card { transition: all 0.3s ease; }
        .skill-card:hover { transform: translateY(-2px); box-shadow: 0 0 30px #00fff220; }
        .skill-card.active { border-color: #00ff88; }
        .log-entry { animation: slideIn 0.3s ease; }
        @keyframes slideIn { from { opacity: 0; transform: translateX(-10px); } to { opacity: 1; transform: translateX(0); } }
        .scrollbar-thin::-webkit-scrollbar { width: 4px; }
        .scrollbar-thin::-webkit-scrollbar-track { background: #0f0f1a; }
        .scrollbar-thin::-webkit-scrollbar-thumb { background: #00fff240; border-radius: 2px; }
    </style>
</head>
<body class="min-h-screen text-gray-200 overflow-hidden">
    <div class="flex h-screen">
        <!-- ═══════════════════════════════════════════════════════════════════════ -->
        <!-- SIDEBAR -->
        <!-- ═══════════════════════════════════════════════════════════════════════ -->
        <aside class="w-64 bg-sentient-900 border-r border-sentient-600 flex flex-col">
            <div class="p-6 border-b border-sentient-600">
                <div class="flex items-center gap-3">
                    <div class="w-10 h-10 rounded-lg bg-gradient-to-br from-cyber-primary to-cyber-secondary flex items-center justify-center">
                        <span class="text-2xl">🐺</span>
                    </div>
                    <div>
                        <h1 class="text-lg font-bold text-cyber-primary glow-cyber">SENTIENT</h1>
                        <p class="text-xs text-gray-500 font-mono">NEXUS OASIS</p>
                    </div>
                </div>
            </div>
            
            <nav class="flex-1 py-4">
                <div class="px-4 mb-2"><span class="text-xs text-gray-600 uppercase tracking-wider">Ana Menü</span></div>
                
                <a href="#" class="sidebar-item active flex items-center gap-3 px-6 py-3 text-gray-300 hover:text-white transition-all" data-view="home" onclick="SENTIENTUI.setView('home')">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"/></svg>
                    <span>Command Center</span>
                </a>
                
                <a href="#" class="sidebar-item flex items-center gap-3 px-6 py-3 text-gray-400 hover:text-white transition-all" data-view="swarm" onclick="SENTIENTUI.setView('swarm')">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z"/></svg>
                    <span>Swarm Agents</span>
                    <span class="ml-auto bg-sentient-600 text-xs px-2 py-0.5 rounded" id="agent-count-sidebar">0</span>
                </a>
                
                <!-- SKILLS / TOOL HUB -->
                <a href="#" class="sidebar-item flex items-center gap-3 px-6 py-3 text-gray-400 hover:text-white transition-all" data-view="skills" onclick="SENTIENTUI.setView('skills')">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"/></svg>
                    <span>Skills / Tool Hub</span>
                    <span class="ml-auto bg-cyber-secondary text-xs px-2 py-0.5 rounded text-sentient-900 font-semibold" id="skills-active-count">0</span>
                </a>
                
                <a href="#" class="sidebar-item flex items-center gap-3 px-6 py-3 text-gray-400 hover:text-white transition-all" data-view="analytics" onclick="SENTIENTUI.setView('analytics')">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/></svg>
                    <span>Analytics</span>
                </a>
                
                <div class="px-4 mt-6 mb-2"><span class="text-xs text-gray-600 uppercase tracking-wider">Operasyonlar</span></div>
                
                <a href="#" class="sidebar-item flex items-center gap-3 px-6 py-3 text-gray-400 hover:text-white transition-all" data-view="tasks" onclick="SENTIENTUI.setView('tasks')">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"/></svg>
                    <span>Görevler</span>
                    <span class="ml-auto bg-cyber-warning text-sentient-900 text-xs px-2 py-0.5 rounded font-semibold" id="task-count-sidebar">0</span>
                </a>
                
                <a href="#" class="sidebar-item flex items-center gap-3 px-6 py-3 text-gray-400 hover:text-white transition-all" data-view="logs" onclick="SENTIENTUI.setView('logs')">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16"/></svg>
                    <span>Log Akışı</span>
                </a>
            </nav>
            
            <div class="p-4 border-t border-sentient-600">
                <div class="text-xs text-gray-600 mb-2">SİSTEM DURUMU</div>
                <div class="space-y-2">
                    <div class="flex justify-between text-sm">
                        <span class="text-gray-400">CPU</span>
                        <span class="font-mono text-cyber-primary" id="cpu-usage">0%</span>
                    </div>
                    <div class="w-full bg-sentient-700 rounded-full h-1">
                        <div class="bg-cyber-primary h-1 rounded-full transition-all" id="cpu-bar" style="width: 0%"></div>
                    </div>
                    <div class="flex justify-between text-sm">
                        <span class="text-gray-400">RAM</span>
                        <span class="font-mono text-cyber-secondary" id="ram-usage">0%</span>
                    </div>
                    <div class="w-full bg-sentient-700 rounded-full h-1">
                        <div class="bg-cyber-secondary h-1 rounded-full transition-all" id="ram-bar" style="width: 0%"></div>
                    </div>
                </div>
            </div>
        </aside>
        
        <!-- ═══════════════════════════════════════════════════════════════════════ -->
        <!-- MAIN CONTENT -->
        <!-- ═══════════════════════════════════════════════════════════════════════ -->
        <main class="flex-1 flex flex-col overflow-hidden">
            <!-- Top Bar -->
            <header class="h-16 bg-sentient-900 border-b border-sentient-600 flex items-center justify-between px-6">
                <div class="flex items-center gap-4">
                    <h2 class="text-lg font-semibold" id="view-title">Command Center</h2>
                    <span class="text-xs text-gray-600 font-mono">/</span>
                    <span class="text-sm text-gray-400" id="current-time">--:--:--</span>
                </div>
                
                <div class="flex items-center gap-6">
                    <div class="flex items-center gap-4 text-sm">
                        <div class="flex items-center gap-2">
                            <span class="text-gray-500">Agents:</span>
                            <span class="font-mono text-cyber-primary" id="agent-count-top">0</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <span class="text-gray-500">Tasks:</span>
                            <span class="font-mono text-cyber-warning" id="task-count-top">0</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <span class="text-gray-500">Uptime:</span>
                            <span class="font-mono text-gray-300" id="uptime">0s</span>
                        </div>
                    </div>
                    
                    <!-- Connection Status - CRITICAL FIX -->
                    <div class="flex items-center gap-2 px-3 py-1.5 rounded-full bg-sentient-800 border border-sentient-600" id="connection-status">
                        <div class="w-2 h-2 rounded-full bg-gray-500" id="connection-dot"></div>
                        <span class="text-sm font-medium" id="connection-text">Bağlanıyor...</span>
                    </div>
                </div>
            </header>
            
            <!-- System Health Metrics -->
            <div class="bg-sentient-800 border-b border-sentient-600 px-6 py-3">
                <div class="grid grid-cols-6 gap-4">
                    <div class="bg-gradient-to-br from-sentient-700 to-sentient-800 rounded-lg p-3 card-glow">
                        <div class="text-xs text-gray-500 mb-1">V-GATE</div>
                        <div class="flex items-center gap-2">
                            <div class="w-2 h-2 rounded-full bg-cyber-success pulse-dot"></div>
                            <span class="font-mono text-sm text-cyber-success">ONLINE</span>
                        </div>
                    </div>
                    <div class="bg-gradient-to-br from-sentient-700 to-sentient-800 rounded-lg p-3 card-glow">
                        <div class="text-xs text-gray-500 mb-1">MEMORY CUBE</div>
                        <div class="flex items-center gap-2">
                            <span class="font-mono text-sm text-gray-300" id="memory-count">0</span>
                            <span class="text-xs text-gray-500">entries</span>
                        </div>
                    </div>
                    <div class="bg-gradient-to-br from-sentient-700 to-sentient-800 rounded-lg p-3 card-glow">
                        <div class="text-xs text-gray-500 mb-1">SCOUT</div>
                        <div class="flex items-center gap-2">
                            <div class="w-2 h-2 rounded-full bg-cyber-primary pulse-dot"></div>
                            <span class="font-mono text-sm text-cyber-primary" id="scout-status">READY</span>
                        </div>
                    </div>
                    <div class="bg-gradient-to-br from-sentient-700 to-sentient-800 rounded-lg p-3 card-glow">
                        <div class="text-xs text-gray-500 mb-1">FORGE</div>
                        <div class="flex items-center gap-2">
                            <div class="w-2 h-2 rounded-full bg-cyber-secondary pulse-dot"></div>
                            <span class="font-mono text-sm text-cyber-secondary" id="forge-status">STANDBY</span>
                        </div>
                    </div>
                    <div class="bg-gradient-to-br from-sentient-700 to-sentient-800 rounded-lg p-3 card-glow">
                        <div class="text-xs text-gray-500 mb-1">THROUGHPUT</div>
                        <span class="font-mono text-sm text-gray-300" id="throughput">0</span>
                        <span class="text-xs text-gray-500">/sec</span>
                    </div>
                    <div class="bg-gradient-to-br from-sentient-700 to-sentient-800 rounded-lg p-3 card-glow">
                        <div class="text-xs text-gray-500 mb-1">QUEUE</div>
                        <span class="font-mono text-sm text-cyber-warning" id="queue-size">0</span>
                    </div>
                </div>
            </div>
            
            <!-- Dynamic Content Area -->
            <div class="flex-1 overflow-hidden flex" id="content-area">
                <!-- HOME VIEW -->
                <div class="flex-1 p-6 overflow-auto" id="view-home">
                    <div class="bg-sentient-800 rounded-xl border border-sentient-600 card-glow h-full">
                        <div class="p-4 border-b border-sentient-600 flex items-center justify-between">
                            <h3 class="font-semibold flex items-center gap-2">
                                <svg class="w-5 h-5 text-cyber-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"/></svg>
                                Agent Topology
                            </h3>
                            <button class="px-3 py-1 text-sm bg-sentient-600 hover:bg-sentient-500 rounded transition-colors" onclick="SENTIENTState.refreshAgents()">Yenile</button>
                        </div>
                        <div class="p-6 grid-lines min-h-96 relative" id="topology-grid">
                            <div class="flex items-center justify-center h-64 text-gray-600" id="topology-empty">
                                <div class="text-center">
                                    <svg class="w-12 h-12 mx-auto mb-3 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/></svg>
                                    <p>Agent bekleniyor...</p>
                                </div>
                            </div>
                            <div class="grid grid-cols-4 gap-4 hidden" id="agent-grid"></div>
                        </div>
                    </div>
                </div>
                
                <!-- SKILLS VIEW -->
                <div class="flex-1 p-6 overflow-auto hidden" id="view-skills">
                    <div class="space-y-6">
                        <div class="flex items-center justify-between">
                            <div>
                                <h3 class="text-xl font-semibold text-cyber-secondary">Skills / Tool Hub</h3>
                                <p class="text-sm text-gray-500">Asimile edilmiş rakip yetenekleri yönet</p>
                            </div>
                            <button onclick="SENTIENTSkills.refresh()" class="px-4 py-2 bg-sentient-600 hover:bg-sentient-500 rounded transition-colors flex items-center gap-2">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg>
                                Yenile
                            </button>
                        </div>
                        
                        <!-- Skills Grid -->
                        <div class="grid grid-cols-3 gap-4" id="skills-grid">
                            <!-- MindSearch -->
                            <div class="skill-card bg-sentient-800 border border-sentient-600 rounded-xl p-5 cursor-pointer" data-skill="mindsearch" onclick="SENTIENTSkills.toggle('mindsearch')">
                                <div class="flex items-center justify-between mb-4">
                                    <div class="flex items-center gap-3">
                                        <div class="w-12 h-12 rounded-lg bg-gradient-to-br from-purple-500 to-blue-600 flex items-center justify-center text-2xl">🧠</div>
                                        <div>
                                            <h4 class="font-semibold text-white">MindSearch</h4>
                                            <p class="text-xs text-gray-500">Deep Research</p>
                                        </div>
                                    </div>
                                    <div class="skill-status w-3 h-3 rounded-full bg-gray-600" id="skill-status-mindsearch"></div>
                                </div>
                                <p class="text-sm text-gray-400 mb-3">Akıllı araştırma ajanı, bilgi grafikleri oluşturur</p>
                                <div class="flex items-center justify-between text-xs">
                                    <span class="text-gray-500">Source: mindsearch/mindsearch</span>
                                    <span class="skill-badge px-2 py-1 rounded bg-sentient-700 text-gray-400" id="skill-badge-mindsearch">Pasif</span>
                                </div>
                            </div>
                            
                            <!-- Lightpanda Browser -->
                            <div class="skill-card bg-sentient-800 border border-sentient-600 rounded-xl p-5 cursor-pointer" data-skill="browser" onclick="SENTIENTSkills.toggle('browser')">
                                <div class="flex items-center justify-between mb-4">
                                    <div class="flex items-center gap-3">
                                        <div class="w-12 h-12 rounded-lg bg-gradient-to-br from-cyan-500 to-teal-600 flex items-center justify-center text-2xl">🌐</div>
                                        <div>
                                            <h4 class="font-semibold text-white">Lightpanda</h4>
                                            <p class="text-xs text-gray-500">Browser DOM</p>
                                        </div>
                                    </div>
                                    <div class="skill-status w-3 h-3 rounded-full bg-gray-600" id="skill-status-browser"></div>
                                </div>
                                <p class="text-sm text-gray-400 mb-3">Headless browser, DOM manipülasyonu ve web etkileşimi</p>
                                <div class="flex items-center justify-between text-xs">
                                    <span class="text-gray-500">Source: lightpanda-io/lightpanda</span>
                                    <span class="skill-badge px-2 py-1 rounded bg-sentient-700 text-gray-400" id="skill-badge-browser">Pasif</span>
                                </div>
                            </div>
                            
                            <!-- AutoResearch -->
                            <div class="skill-card bg-sentient-800 border border-sentient-600 rounded-xl p-5 cursor-pointer" data-skill="autoresearch" onclick="SENTIENTSkills.toggle('autoresearch')">
                                <div class="flex items-center justify-between mb-4">
                                    <div class="flex items-center gap-3">
                                        <div class="w-12 h-12 rounded-lg bg-gradient-to-br from-green-500 to-emerald-600 flex items-center justify-center text-2xl">📄</div>
                                        <div>
                                            <h4 class="font-semibold text-white">AutoResearch</h4>
                                            <p class="text-xs text-gray-500">PDF Parsing</p>
                                        </div>
                                    </div>
                                    <div class="skill-status w-3 h-3 rounded-full bg-gray-600" id="skill-status-autoresearch"></div>
                                </div>
                                <p class="text-sm text-gray-400 mb-3">PDF parsing, akademik makale analizi</p>
                                <div class="flex items-center justify-between text-xs">
                                    <span class="text-gray-500">Source: autoresearch/autoresearch</span>
                                    <span class="skill-badge px-2 py-1 rounded bg-sentient-700 text-gray-400" id="skill-badge-autoresearch">Pasif</span>
                                </div>
                            </div>
                            
                            <!-- n8n Automation -->
                            <div class="skill-card bg-sentient-800 border border-sentient-600 rounded-xl p-5 cursor-pointer" data-skill="n8n" onclick="SENTIENTSkills.toggle('n8n')">
                                <div class="flex items-center justify-between mb-4">
                                    <div class="flex items-center gap-3">
                                        <div class="w-12 h-12 rounded-lg bg-gradient-to-br from-orange-500 to-red-600 flex items-center justify-center text-2xl">⚡</div>
                                        <div>
                                            <h4 class="font-semibold text-white">n8n Automation</h4>
                                            <p class="text-xs text-gray-500">Workflows</p>
                                        </div>
                                    </div>
                                    <div class="skill-status w-3 h-3 rounded-full bg-gray-600" id="skill-status-n8n"></div>
                                </div>
                                <p class="text-sm text-gray-400 mb-3">Workflow otomasyonu ve entegrasyonlar</p>
                                <div class="flex items-center justify-between text-xs">
                                    <span class="text-gray-500">Source: n8n-io/n8n</span>
                                    <span class="skill-badge px-2 py-1 rounded bg-sentient-700 text-gray-400" id="skill-badge-n8n">Pasif</span>
                                </div>
                            </div>
                            
                            <!-- Web Search -->
                            <div class="skill-card bg-sentient-800 border border-sentient-600 rounded-xl p-5 cursor-pointer" data-skill="websearch" onclick="SENTIENTSkills.toggle('websearch')">
                                <div class="flex items-center justify-between mb-4">
                                    <div class="flex items-center gap-3">
                                        <div class="w-12 h-12 rounded-lg bg-gradient-to-br from-blue-500 to-indigo-600 flex items-center justify-center text-2xl">🔍</div>
                                        <div>
                                            <h4 class="font-semibold text-white">Web Search</h4>
                                            <p class="text-xs text-gray-500">DuckDuckGo</p>
                                        </div>
                                    </div>
                                    <div class="skill-status w-3 h-3 rounded-full bg-gray-600" id="skill-status-websearch"></div>
                                </div>
                                <p class="text-sm text-gray-400 mb-3">DuckDuckGo ve açık web araması</p>
                                <div class="flex items-center justify-between text-xs">
                                    <span class="text-gray-500">Source: internal</span>
                                    <span class="skill-badge px-2 py-1 rounded bg-sentient-700 text-gray-400" id="skill-badge-websearch">Pasif</span>
                                </div>
                            </div>
                            
                            <!-- Citation -->
                            <div class="skill-card bg-sentient-800 border border-sentient-600 rounded-xl p-5 cursor-pointer" data-skill="citation" onclick="SENTIENTSkills.toggle('citation')">
                                <div class="flex items-center justify-between mb-4">
                                    <div class="flex items-center gap-3">
                                        <div class="w-12 h-12 rounded-lg bg-gradient-to-br from-yellow-500 to-amber-600 flex items-center justify-center text-2xl">📚</div>
                                        <div>
                                            <h4 class="font-semibold text-white">Citation Manager</h4>
                                            <p class="text-xs text-gray-500">References</p>
                                        </div>
                                    </div>
                                    <div class="skill-status w-3 h-3 rounded-full bg-gray-600" id="skill-status-citation"></div>
                                </div>
                                <p class="text-sm text-gray-400 mb-3">Alıntı yönetimi ve kaynak doğrulama</p>
                                <div class="flex items-center justify-between text-xs">
                                    <span class="text-gray-500">Source: internal</span>
                                    <span class="skill-badge px-2 py-1 rounded bg-sentient-700 text-gray-400" id="skill-badge-citation">Pasif</span>
                                </div>
                            </div>
                        </div>
                        
                        <!-- Skills Stats -->
                        <div class="bg-sentient-800 rounded-xl border border-sentient-600 p-4">
                            <h4 class="font-semibold mb-3 flex items-center gap-2">
                                <svg class="w-5 h-5 text-cyber-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/></svg>
                                Skills İstatistikleri
                            </h4>
                            <div class="grid grid-cols-4 gap-4 text-center">
                                <div>
                                    <div class="text-2xl font-bold text-cyber-primary" id="stats-total">6</div>
                                    <div class="text-xs text-gray-500">Toplam</div>
                                </div>
                                <div>
                                    <div class="text-2xl font-bold text-cyber-success" id="stats-active">0</div>
                                    <div class="text-xs text-gray-500">Aktif</div>
                                </div>
                                <div>
                                    <div class="text-2xl font-bold text-cyber-warning" id="stats-executions">0</div>
                                    <div class="text-xs text-gray-500">Çalıştırma</div>
                                </div>
                                <div>
                                    <div class="text-2xl font-bold text-gray-400" id="stats-avg-duration">0ms</div>
                                    <div class="text-xs text-gray-500">Ort. Süre</div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                
                <!-- Right Panel - Logs -->
                <div class="w-80 bg-sentient-900 border-l border-sentient-600 flex flex-col">
                    <div class="p-4 border-b border-sentient-600">
                        <h3 class="font-semibold flex items-center gap-2">
                            <svg class="w-5 h-5 text-cyber-secondary" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
                            Canlı Log Akışı
                        </h3>
                    </div>
                    <div class="flex-1 overflow-y-auto p-4 space-y-2 scrollbar-thin" id="log-container">
                        <div class="text-center text-gray-600 text-sm py-8">Log bekleniyor...</div>
                    </div>
                    
                    <div class="p-4 border-t border-sentient-600">
                        <div class="text-xs text-gray-500 mb-2">HIZLI GÖREV</div>
                        <div class="flex gap-2">
                            <input type="text" id="quick-task-input" class="flex-1 bg-sentient-700 border border-sentient-600 rounded px-3 py-2 text-sm focus:outline-none focus:border-cyber-primary" placeholder="Görev girin..." onkeypress="if(event.key==='Enter')SENTIENTState.createTask()">
                            <button onclick="SENTIENTState.createTask()" class="px-4 py-2 bg-cyber-primary text-sentient-900 rounded font-semibold hover:bg-cyber-success transition-colors">Gönder</button>
                        </div>
                    </div>
                </div>
            </div>
        </main>
    </div>
    
    <script src="/app.js"></script>
</body>
</html>"##.to_string()
    }
    
    /// CSS stilleri
    pub fn style_css() -> String {
        r##"/* SENTIENT Enterprise Dashboard - Minimal Override */"##.to_string()
    }
    
    /// JavaScript - Enterprise State Management with Skills Hub
    pub fn app_js() -> String {
        r##"/**
 * SENTIENT Command Center - Enterprise State Management
 * Skills/Tool Hub Integration
 */

// ═══════════════════════════════════════════════════════════════════════════════
// SKILLS HUB MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

const SENTIENTSkills = {
    skills: {},
    
    async refresh() {
        try {
            const res = await fetch('/api/skills');
            const skills = await res.json();
            this.skills = {};
            skills.forEach(s => { this.skills[s.skill_type] = s; });
            this.updateUI();
            this.updateStats();
        } catch (e) {
            console.error('[Skills] Refresh error:', e);
        }
    },
    
    async toggle(skillType) {
        try {
            const res = await fetch(`/api/skills/${skillType}/toggle`, { method: 'POST' });
            const data = await res.json();
            if (data.success) {
                this.skills[skillType] = this.skills[skillType] || {};
                this.skills[skillType].enabled = data.enabled;
                this.updateUI();
                this.updateStats();
                SENTIENTState.addLog({
                    level: data.enabled ? 'success' : 'info',
                    source: 'SKILLS',
                    message: data.message
                });
            }
        } catch (e) {
            console.error('[Skills] Toggle error:', e);
            SENTIENTState.addLog({ level: 'error', source: 'SKILLS', message: 'Skill toggle hatası' });
        }
    },
    
    async execute(skillType, params = {}) {
        try {
            const res = await fetch(`/api/skills/${skillType}/execute`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(params)
            });
            return await res.json();
        } catch (e) {
            console.error('[Skills] Execute error:', e);
            return { success: false, error: e.message };
        }
    },
    
    updateUI() {
        Object.keys(this.skills).forEach(type => {
            const skill = this.skills[type];
            const statusEl = document.getElementById(`skill-status-${type}`);
            const badgeEl = document.getElementById(`skill-badge-${type}`);
            const cardEl = document.querySelector(`[data-skill="${type}"]`);
            
            if (statusEl) {
                statusEl.classList.remove('bg-gray-600', 'bg-cyber-success');
                statusEl.classList.add(skill.enabled ? 'bg-cyber-success' : 'bg-gray-600');
            }
            if (badgeEl) {
                badgeEl.classList.remove('bg-sentient-700', 'text-gray-400', 'bg-cyber-success', 'text-sentient-900');
                if (skill.enabled) {
                    badgeEl.classList.add('bg-cyber-success', 'text-sentient-900');
                    badgeEl.textContent = 'Aktif';
                } else {
                    badgeEl.classList.add('bg-sentient-700', 'text-gray-400');
                    badgeEl.textContent = 'Pasif';
                }
            }
            if (cardEl) {
                cardEl.classList.toggle('active', skill.enabled);
                cardEl.style.borderColor = skill.enabled ? '#00ff88' : '';
            }
        });
    },
    
    updateStats() {
        const active = Object.values(this.skills).filter(s => s.enabled).length;
        document.getElementById('skills-active-count').textContent = active;
        document.getElementById('stats-active').textContent = active;
    }
};

// ═══════════════════════════════════════════════════════════════════════════════
// MAIN STATE MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

const SENTIENTState = {
    ws: null,
    connected: false,
    reconnectAttempts: 0,
    maxReconnectAttempts: 10,
    reconnectDelay: 3000,
    
    state: { agents: [], tasks: [], logs: [], metrics: {} },
    
    init() {
        console.log('[SENTIENT] Initializing Command Center...');
        this.connect();
        this.startClock();
        SENTIENTSkills.refresh();
        setInterval(() => SENTIENTSkills.refresh(), 10000);
    },
    
    // ═══════════════════════════════════════════════════════════════════════════
    // WEBSOCKET CONNECTION - ROBUST IMPLEMENTATION
    // ═══════════════════════════════════════════════════════════════════════════
    
    connect() {
        if (this.ws && this.ws.readyState === WebSocket.OPEN) return;
        
        const protocol = location.protocol === 'https:' ? 'wss:' : 'ws:';
        const wsUrl = `${protocol}//${location.host}/ws`;
        
        console.log(`[SENTIENT] Connecting to ${wsUrl}...`);
        this.updateConnectionStatus('connecting');
        
        try {
            this.ws = new WebSocket(wsUrl);
            
            this.ws.onopen = () => {
                console.log('[SENTIENT] WebSocket CONNECTED');
                this.connected = true;
                this.reconnectAttempts = 0;
                this.updateConnectionStatus('connected');
                
                this.ws.send(JSON.stringify({
                    type: 'subscribe',
                    channels: ['scout', 'forge', 'swarm', 'thoughts', 'logs', 'metrics', 'skills']
                }));
            };
            
            this.ws.onmessage = (e) => {
                try {
                    const data = JSON.parse(e.data);
                    this.handleMessage(data);
                } catch (err) {
                    console.error('[SENTIENT] Parse error:', err);
                }
            };
            
            this.ws.onclose = (e) => {
                console.log('[SENTIENT] WebSocket CLOSED:', e.code);
                this.connected = false;
                this.updateConnectionStatus('disconnected');
                this.scheduleReconnect();
            };
            
            this.ws.onerror = (err) => {
                console.error('[SENTIENT] WebSocket ERROR:', err);
                this.connected = false;
                this.updateConnectionStatus('error');
            };
        } catch (e) {
            console.error('[SENTIENT] WebSocket init failed:', e);
            this.updateConnectionStatus('error');
            this.scheduleReconnect();
        }
    },
    
    scheduleReconnect() {
        if (this.reconnectAttempts < this.maxReconnectAttempts) {
            this.reconnectAttempts++;
            console.log(`[SENTIENT] Reconnecting in ${this.reconnectDelay/1000}s (${this.reconnectAttempts}/${this.maxReconnectAttempts})`);
            setTimeout(() => this.connect(), this.reconnectDelay);
        } else {
            this.updateConnectionStatus('failed');
        }
    },
    
    // ═══════════════════════════════════════════════════════════════════════════
    // CONNECTION STATUS UI - CRITICAL FIX
    // ═══════════════════════════════════════════════════════════════════════════
    
    updateConnectionStatus(status) {
        const dot = document.getElementById('connection-dot');
        const text = document.getElementById('connection-text');
        const container = document.getElementById('connection-status');
        
        if (!dot || !text) return;
        
        // Reset classes
        dot.className = 'w-2 h-2 rounded-full';
        container.className = 'flex items-center gap-2 px-3 py-1.5 rounded-full bg-sentient-800 border border-sentient-600';
        
        switch (status) {
            case 'connected':
                dot.classList.add('bg-cyber-success', 'pulse-dot');
                text.textContent = 'Connected';
                text.className = 'text-sm font-medium text-cyber-success';
                container.classList.add('border-cyber-success');
                break;
            case 'connecting':
                dot.classList.add('bg-cyber-warning', 'pulse-dot');
                text.textContent = 'Bağlanıyor...';
                text.className = 'text-sm font-medium text-cyber-warning';
                container.classList.add('border-cyber-warning');
                break;
            case 'disconnected':
            case 'error':
            case 'failed':
                dot.classList.add('bg-cyber-danger');
                text.textContent = status === 'failed' ? 'Connection Failed' : 'Disconnected';
                text.className = 'text-sm font-medium text-cyber-danger';
                container.classList.add('border-cyber-danger');
                break;
        }
        
        console.log(`[SENTIENT] Connection status: ${status}`);
    },
    
    handleMessage(data) {
        const type = data.type || data.event || 'unknown';
        
        switch (type) {
            case 'metrics': this.updateMetrics(data.payload || data); break;
            case 'agent_update': case 'agents': this.updateAgents(data.payload || data.agents || []); break;
            case 'task_created': this.addTask(data.payload || data); break;
            case 'log': this.addLog(data.payload || data); break;
            case 'skills_update': SENTIENTSkills.refresh(); break;
            case 'pong': break;
            default: if (data.success) this.addLog({ level: 'info', source: 'WS', message: data.message });
        }
    },
    
    updateMetrics(m) {
        if (m.cpu !== undefined) { document.getElementById('cpu-usage').textContent = m.cpu + '%'; document.getElementById('cpu-bar').style.width = m.cpu + '%'; }
        if (m.ram !== undefined) { document.getElementById('ram-usage').textContent = m.ram + '%'; document.getElementById('ram-bar').style.width = m.ram + '%'; }
        if (m.memory_count !== undefined) document.getElementById('memory-count').textContent = m.memory_count;
        if (m.throughput !== undefined) document.getElementById('throughput').textContent = m.throughput;
        if (m.queue_size !== undefined) document.getElementById('queue-size').textContent = m.queue_size;
        if (m.uptime_secs !== undefined) document.getElementById('uptime').textContent = this.formatUptime(m.uptime_secs);
        if (m.active_agents !== undefined) { document.getElementById('agent-count-top').textContent = m.active_agents; document.getElementById('agent-count-sidebar').textContent = m.active_agents; }
        if (m.active_tasks !== undefined) { document.getElementById('task-count-top').textContent = m.active_tasks; document.getElementById('task-count-sidebar').textContent = m.active_tasks; }
    },
    
    updateAgents(agents) {
        this.state.agents = agents;
        const grid = document.getElementById('agent-grid');
        const empty = document.getElementById('topology-empty');
        if (!grid) return;
        if (agents.length === 0) { grid.classList.add('hidden'); if (empty) empty.style.display = 'flex'; return; }
        if (empty) empty.style.display = 'none';
        grid.classList.remove('hidden');
        grid.innerHTML = agents.map(a => `<div class="bg-sentient-700 rounded-lg p-4 border border-sentient-500"><div class="flex items-center gap-3 mb-3"><div class="w-10 h-10 rounded-lg flex items-center justify-center" style="background:${a.color||'#00fff2'}20;border:1px solid ${a.color||'#00fff2'}"><span class="text-lg">${a.type==='scout'?'🔍':a.type==='forge'?'🔨':'🤖'}</span></div><div><div class="font-semibold text-sm">${a.name||a.id}</div><div class="text-xs text-gray-500">${a.type||'agent'}</div></div></div><div class="flex items-center justify-between text-xs"><span class="px-2 py-1 rounded ${a.status==='active'?'bg-cyber-success/20 text-cyber-success':'bg-gray-600/20 text-gray-400'}">${a.status||'active'}</span></div></div>`).join('');
    },
    
    addLog(log) {
        const container = document.getElementById('log-container');
        if (!container) return;
        const placeholder = container.querySelector('.text-center');
        if (placeholder) placeholder.remove();
        const entry = document.createElement('div');
        entry.className = 'log-entry text-xs font-mono p-2 rounded bg-sentient-700 border-l-2 ' + ({info:'border-cyber-primary',success:'border-cyber-success',warning:'border-cyber-warning',error:'border-cyber-danger'}[log.level]||'border-cyber-primary'));
        entry.innerHTML = `<div class="flex items-center justify-between mb-1"><span class="text-gray-500">${new Date().toLocaleTimeString('tr-TR')}</span><span class="text-cyber-primary">${log.source||'SYS'}</span></div><div class="text-gray-300">${log.message}</div>`;
        container.insertBefore(entry, container.firstChild);
        while (container.children.length > 50) container.removeChild(container.lastChild);
    },
    
    addTask(task) { this.state.tasks.unshift(task); this.updateTaskCounts(); this.addLog({ level: 'success', source: 'TASK', message: 'Yeni görev: ' + (task.goal || task.id) }); },
    updateTaskCounts() { const n = this.state.tasks.filter(t => t.status !== 'completed').length; document.getElementById('task-count-top').textContent = n; document.getElementById('task-count-sidebar').textContent = n; },
    
    createTask() {
        const input = document.getElementById('quick-task-input');
        const goal = input?.value?.trim();
        if (!goal) { this.addLog({ level: 'warning', source: 'UI', message: 'Lütfen görev girin' }); return; }
        if (!this.connected) { this.addLog({ level: 'error', source: 'WS', message: 'WebSocket bağlı değil' }); return; }
        this.ws.send(JSON.stringify({ type: 'create_task', goal }));
        input.value = '';
    },
    
    refreshAgents() { if (this.connected) this.ws.send(JSON.stringify({ type: 'get_agents' })); this.fetchMetrics(); },
    async fetchMetrics() { try { const r = await fetch('/api/stats'); if (r.ok) this.updateMetrics(await r.json()); } catch (e) {} },
    
    formatUptime(s) { if (s < 60) return s + 's'; if (s < 3600) return Math.floor(s/60) + 'm'; if (s < 86400) return Math.floor(s/3600) + 'h ' + Math.floor((s%3600)/60) + 'm'; return Math.floor(s/86400) + 'd ' + Math.floor((s%86400)/3600) + 'h'; },
    
    startClock() { const u = () => document.getElementById('current-time') && (document.getElementById('current-time').textContent = new Date().toLocaleTimeString('tr-TR')); u(); setInterval(u, 1000); }
};

// ═══════════════════════════════════════════════════════════════════════════════
// UI CONTROLLER
// ═══════════════════════════════════════════════════════════════════════════════

const SENTIENTUI = {
    setView(view) {
        document.querySelectorAll('.sidebar-item').forEach(i => i.classList.remove('active'));
        document.querySelector(`[data-view="${view}"]`)?.classList.add('active');
        ['home','swarm','skills','analytics','tasks','logs'].forEach(v => {
            const el = document.getElementById(`view-${v}`);
            if (el) el.classList.toggle('hidden', v !== view);
        });
        const titles = { home: 'Command Center', swarm: 'Swarm Agents', skills: 'Skills / Tool Hub', analytics: 'Analytics', tasks: 'Görevler', logs: 'Log Akışı' };
        document.getElementById('view-title').textContent = titles[view] || view;
    }
};

document.addEventListener('DOMContentLoaded', () => { SENTIENTState.init(); window.SENTIENTState = SENTIENTState; window.SENTIENTSkills = SENTIENTSkills; window.SENTIENTUI = SENTIENTUI; });"##.to_string()
    }
    
    /// PWA Manifest
    pub fn manifest_json() -> String {
        r##"{"name":"SENTIENT Command Center","short_name":"SENTIENT","description":"NEXUS OASIS AI Operating System","start_url":"/dashboard","display":"standalone","background_color":"#0a0a0f","theme_color":"#00fff2"}"##.to_string()
    }
    
    /// Service Worker
    pub fn service_worker_js() -> String {
        r##"// SENTIENT Service Worker
const CACHE='sentient-v1';
self.addEventListener('install',e=>self.skipWaiting());
self.addEventListener('fetch',e=>e.respondWith(fetch(e.request)));"##.to_string()
    }
    
    /// Claw3D sayfası
    pub fn claw3d_html() -> String {
        r##"<!DOCTYPE html><html lang="tr" class="dark"><head><meta charset="UTF-8"><title>SENTIENT // Claw3D</title><script src="https://cdn.tailwindcss.com"></script></head><body class="bg-sentient-900 min-h-screen flex items-center justify-center"><div class="text-center"><h1 class="text-3xl font-bold text-cyber-primary mb-4">🐺 Claw3D</h1><p class="text-gray-500">3D Swarm Visualization</p></div></body></html>"##.to_string()
    }
    
    /// Memory Bridge sayfası
    pub fn memory_bridge_html() -> String {
        r##"<!DOCTYPE html><html lang="tr" class="dark"><head><meta charset="UTF-8"><title>SENTIENT // Memory Bridge</title><script src="https://cdn.tailwindcss.com"></script></head><body class="bg-sentient-900 min-h-screen flex items-center justify-center"><div class="text-center"><h1 class="text-3xl font-bold text-cyber-secondary mb-4">🧠 Memory Bridge</h1><p class="text-gray-500">Memory Cube Visualization</p></div></body></html>"##.to_string()
    }
}
