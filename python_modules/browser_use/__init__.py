"""
SENTIENT Browser-Use Modülü
────────────────────────
Tarayıcı otomasyonu için browser-use kütüphanesini saran modül.
Otonom web gezintisi ve araştırma yapabilme yeteneği sağlar.

KURULUM:
    pip install browser-use playwright
    playwright install chromium
"""

from typing import Optional, Dict, Any, List, Callable
from dataclasses import dataclass, field
import json
import asyncio
import logging
from datetime import datetime
from pathlib import Path

# browser-use lazy import
try:
    from browser_use import Browser, BrowserUse, Agent, Controller
    from browser_use.browser.context import BrowserContext
    from langchain_core.language_models import BaseChatModel
    BROWSER_USE_AVAILABLE = True
except ImportError:
    BROWSER_USE_AVAILABLE = False
    Browser = None
    Agent = None
    Controller = None

logger = logging.getLogger("sentient.browser")


# ─── Veri Yapıları ───

@dataclass
class BrowserConfig:
    """Tarayıcı yapılandırması"""
    headless: bool = True
    disable_security: bool = False
    window_width: int = 1280
    window_height: int = 720
    user_agent: Optional[str] = None
    proxy: Optional[str] = None
    cookies_file: Optional[str] = None
    save_cookies: bool = True
    timeout_seconds: int = 120


@dataclass
class TaskResult:
    """Görev sonucu"""
    success: bool
    content: str
    url: Optional[str] = None
    screenshot: Optional[str] = None
    links: List[str] = field(default_factory=list)
    metadata: Dict[str, Any] = field(default_factory=dict)
    error: Optional[str] = None
    timestamp: str = field(default_factory=lambda: datetime.now().isoformat())


@dataclass
class ResearchResult:
    """Araştırma sonucu"""
    query: str
    summary: str
    sources: List[str]
    findings: List[str]
    confidence: float = 0.0
    duration_seconds: float = 0.0


class SENTIENTBrowserAgent:
    """
    SENTIENT Tarayıcı Ajanı
    
    Yetenekler:
    - Otonom web gezintisi
    - Akıllı arama ve araştırma
    - Sayfa analizi ve veri çıkarma
    - Ekran görüntüsü alma
    - Form doldurma ve buton tıklama
    """
    
    def __init__(self, config: Optional[BrowserConfig] = None):
        self.config = config or BrowserConfig()
        self.browser: Optional[Browser] = None
        self.context: Optional[BrowserContext] = None
        self.agent: Optional[Agent] = None
        self.controller: Optional[Controller] = None
        self.available = BROWSER_USE_AVAILABLE
        self._initialized = False
        self._history: List[Dict[str, Any]] = []
        
    def _get_browser_args(self) -> Dict[str, Any]:
        """Browser kullanımı için argümanları hazırla"""
        return {
            "headless": self.config.headless,
            "disable_security": self.config.disable_security,
        }
    
    async def initialize(self, llm_provider: Optional[Any] = None) -> Dict[str, Any]:
        """
        Tarayıcıyı ve ajanı başlat
        
        Args:
            llm_provider: LLM sağlayıcısı (LangChain formatında)
        """
        if not self.available:
            return {
                "success": False,
                "message": "browser-use kütüphanesi kurulu değil. pip install browser-use playwright",
                "error": "IMPORT_ERROR"
            }
        
        try:
            # Tarayıcıyı başlat
            self.browser = Browser(**self._get_browser_args())
            self.context = await self.browser.new_context(
                viewport={"width": self.config.window_width, "height": self.config.window_height}
            )
            
            # Controller oluştur
            self.controller = Controller()
            
            self._initialized = True
            logger.info("🌐 TARAYICI: Başlatıldı (headless=%s)", self.config.headless)
            
            return {
                "success": True,
                "message": "Tarayıcı başarıyla başlatıldı.",
                "config": {
                    "headless": self.config.headless,
                    "window_size": f"{self.config.window_width}x{self.config.window_height}"
                }
            }
        except Exception as e:
            logger.error("🌐 TARAYICI HATA → %s", str(e))
            return {
                "success": False,
                "message": f"Tarayıcı başlatma hatası: {str(e)}",
                "error": "INIT_ERROR"
            }
    
    async def execute_task(
        self,
        task: str,
        llm_provider: Optional[Any] = None,
        max_steps: int = 20
    ) -> TaskResult:
        """
        Otonom bir görevi çalıştır
        
        Args:
            task: Yapılacak görev (doğal dilde)
            llm_provider: LLM sağlayıcısı
            max_steps: Maksimum adım sayısı
        """
        if not self._initialized or self.context is None:
            return TaskResult(
                success=False,
                content="",
                error="Tarayıcı başlatılmadı. Önce initialize() çağırın."
            )
        
        start_time = datetime.now()
        
        try:
            logger.info("🌐 GÖREV BAŞLADI → %s", task[:100])
            
            if self.agent is None and llm_provider and Agent:
                self.agent = Agent(
                    task=task,
                    llm=llm_provider,
                    browser_context=self.context,
                    controller=self.controller,
                    max_actions=max_steps
                )
            elif self.agent:
                self.agent.task = task
            
            # Görevi çalıştır
            if self.agent:
                result = await self.agent.run()
                
                # Sonucu işle
                content = str(result) if result else "Görev tamamlandı, çıktı yok."
                
                # Geçmişe ekle
                self._history.append({
                    "task": task,
                    "result": content,
                    "timestamp": datetime.now().isoformat()
                })
                
                duration = (datetime.now() - start_time).total_seconds()
                logger.info("🌐 GÖREV TAMAMLANDI → %.2f saniye", duration)
                
                return TaskResult(
                    success=True,
                    content=content,
                    metadata={"duration_seconds": duration, "steps": max_steps}
                )
            else:
                return TaskResult(
                    success=False,
                    content="",
                    error="LLM sağlayıcısı gerekli. Agent oluşturulamadı."
                )
                
        except Exception as e:
            logger.error("🌐 GÖREV HATA → %s", str(e))
            return TaskResult(
                success=False,
                content="",
                error=f"Görev hatası: {str(e)}"
            )
    
    async def navigate(self, url: str) -> TaskResult:
        """URL'ye git"""
        if not self._initialized or self.context is None:
            return TaskResult(
                success=False,
                content="",
                error="Tarayıcı başlatılmadı."
            )
        
        try:
            page = await self.context.new_page()
            await page.goto(url, timeout=self.config.timeout_seconds * 1000)
            
            # Sayfa başlığını al
            title = await page.title()
            current_url = page.url
            
            logger.info("🌐 NAVIGATE → %s", url)
            
            return TaskResult(
                success=True,
                content=f"Sayfa yüklendi: {title}",
                url=current_url
            )
        except Exception as e:
            logger.error("🌐 NAVIGATE HATA → %s", str(e))
            return TaskResult(
                success=False,
                content="",
                url=url,
                error=f"Navigasyon hatası: {str(e)}"
            )
    
    async def search(self, query: str, engine: str = "google") -> TaskResult:
        """
        Web'de ara
        
        Args:
            query: Arama sorgusu
            engine: Arama motoru (google, duckduckgo, bing)
        """
        engine_urls = {
            "google": "https://www.google.com/search?q=",
            "duckduckgo": "https://duckduckgo.com/?q=",
            "bing": "https://www.bing.com/search?q="
        }
        
        if engine not in engine_urls:
            engine = "google"
        
        url = engine_urls[engine] + query.replace(" ", "+")
        result = await self.navigate(url)
        
        if result.success:
            result.metadata["search_query"] = query
            result.metadata["engine"] = engine
            logger.info("🌐 ARAMA → %s (%s)", query, engine)
        
        return result
    
    async def research(self, topic: str, depth: int = 3) -> ResearchResult:
        """
        Derinlemesine araştırma yap
        
        Args:
            topic: Araştırma konusu
            depth: Derinlik (kaç sayfa ziyaret edilecek)
        """
        if not self._initialized:
            return ResearchResult(
                query=topic,
                summary="",
                sources=[],
                findings=["Tarayıcı başlatılmadı."],
                confidence=0.0
            )
        
        logger.info("🔍 ARAŞTIRMA BAŞLADI → %s (derinlik: %d)", topic, depth)
        
        findings = []
        sources = []
        
        # Ana arama
        search_result = await self.search(topic)
        if search_result.success:
            sources.append(search_result.url or "Bilinmeyen URL")
        
        # TODO: Burada LLM ile derinlemesine araştırma yapılacak
        # Her sayfadan ana noktaları çıkar, linkleri takip et
        
        summary = f"'{topic}' hakkında araştırma tamamlandı. {len(sources)} kaynak bulundu."
        
        return ResearchResult(
            query=topic,
            summary=summary,
            sources=sources,
            findings=findings,
            confidence=0.7 if sources else 0.3,
            duration_seconds=0.0
        )
    
    async def screenshot(self, full_page: bool = False) -> TaskResult:
        """Ekran görüntüsü al"""
        if not self._initialized or self.context is None:
            return TaskResult(
                success=False,
                content="",
                error="Tarayıcı başlatılmadı."
            )
        
        try:
            # Aktif sayfayı al
            pages = self.context.pages()
            if not pages:
                return TaskResult(
                    success=False,
                    content="",
                    error="Açık sayfa yok."
                )
            
            page = pages[-1]
            
            # Screenshot al
            screenshot_bytes = await page.screenshot(full_page=full_page)
            
            # Base64'e çevir
            import base64
            screenshot_b64 = base64.b64encode(screenshot_bytes).decode('utf-8')
            
            logger.info("🌐 SCREENSHOT alındı")
            
            return TaskResult(
                success=True,
                content="Ekran görüntüsü alındı.",
                screenshot=screenshot_b64,
                url=page.url
            )
        except Exception as e:
            return TaskResult(
                success=False,
                content="",
                error=f"Screenshot hatası: {str(e)}"
            )
    
    async def extract_content(self, selector: Optional[str] = None) -> TaskResult:
        """
        Sayfa içeriğini çıkar
        
        Args:
            selector: CSS seçici (None = tüm sayfa)
        """
        if not self._initialized or self.context is None:
            return TaskResult(
                success=False,
                content="",
                error="Tarayıcı başlatılmadı."
            )
        
        try:
            pages = self.context.pages()
            if not pages:
                return TaskResult(
                    success=False,
                    content="",
                    error="Açık sayfa yok."
                )
            
            page = pages[-1]
            
            if selector:
                content = await page.inner_text(selector)
            else:
                # Ana içerik alanını bulmaya çalış
                content = await page.evaluate("""
                    () => {
                        // Ana içerik alanını bul
                        const main = document.querySelector('main, article, .content, #content');
                        if (main) return main.innerText;
                        
                        // Yoksa body'den al
                        return document.body.innerText;
                    }
                """)
            
            # Linkleri de topla
            links = await page.evaluate("""
                () => {
                    const links = Array.from(document.querySelectorAll('a[href]'));
                    return links.slice(0, 50).map(a => ({
                        url: a.href,
                        text: a.innerText.trim().slice(0, 100)
                    }));
                }
            """)
            
            logger.info("🌐 İÇERİK ÇIKARILDI → %d karakter", len(content))
            
            return TaskResult(
                success=True,
                content=content,
                links=[l["url"] for l in links if l.get("url")],
                url=page.url,
                metadata={"link_count": len(links)}
            )
        except Exception as e:
            return TaskResult(
                success=False,
                content="",
                error=f"İçerik çıkarma hatası: {str(e)}"
            )
    
    async def click(self, selector: str) -> TaskResult:
        """Bir elemente tıkla"""
        if not self._initialized or self.context is None:
            return TaskResult(
                success=False,
                content="",
                error="Tarayıcı başlatılmadı."
            )
        
        try:
            pages = self.context.pages()
            if not pages:
                return TaskResult(
                    success=False,
                    content="",
                    error="Açık sayfa yok."
                )
            
            page = pages[-1]
            await page.click(selector)
            
            logger.info("🌐 TIKLAMA → %s", selector)
            
            return TaskResult(
                success=True,
                content=f"Elemente tıklandı: {selector}",
                url=page.url
            )
        except Exception as e:
            return TaskResult(
                success=False,
                content="",
                error=f"Tıklama hatası: {str(e)}"
            )
    
    async def type_text(self, selector: str, text: str, press_enter: bool = False) -> TaskResult:
        """Bir input alanına yazı yaz"""
        if not self._initialized or self.context is None:
            return TaskResult(
                success=False,
                content="",
                error="Tarayıcı başlatılmadı."
            )
        
        try:
            pages = self.context.pages()
            if not pages:
                return TaskResult(
                    success=False,
                    content="",
                    error="Açık sayfa yok."
                )
            
            page = pages[-1]
            await page.fill(selector, text)
            
            if press_enter:
                await page.press(selector, "Enter")
            
            logger.info("🌐 YAZI YAZILDI → %s", selector)
            
            return TaskResult(
                success=True,
                content=f"Yazı yazıldı: {text[:50]}...",
                url=page.url
            )
        except Exception as e:
            return TaskResult(
                success=False,
                content="",
                error=f"Yazı yazma hatası: {str(e)}"
            )
    
    def get_history(self) -> List[Dict[str, Any]]:
        """Görev geçmişini döndür"""
        return self._history.copy()
    
    async def close(self) -> Dict[str, Any]:
        """Tarayıcıyı kapat"""
        if self.browser:
            try:
                await self.browser.close()
                self._initialized = False
                logger.info("🌐 TARAYICI: Kapatıldı")
                return {
                    "success": True,
                    "message": "Tarayıcı kapatıldı."
                }
            except Exception as e:
                return {
                    "success": False,
                    "message": f"Kapatma hatası: {str(e)}"
                }
        return {
            "success": True,
            "message": "Tarayıcı zaten kapalı."
        }


# ─── Senkron Wrapper (Rust uyumu için) ───

class SENTIENTBrowserSync:
    """
    Senkron API wrapper (Rust PyO3 köprüsü için)
    
    Tüm async metodları senkron wrapper ile sunar.
    """
    
    def __init__(self):
        self._agent = SENTIENTBrowserAgent()
        self._loop: Optional[asyncio.AbstractEventLoop] = None
    
    def _get_loop(self) -> asyncio.AbstractEventLoop:
        """Event loop al veya oluştur"""
        if self._loop is None or self._loop.is_closed():
            try:
                self._loop = asyncio.get_event_loop()
            except RuntimeError:
                self._loop = asyncio.new_event_loop()
                asyncio.set_event_loop(self._loop)
        return self._loop
    
    def initialize(self, headless: bool = True) -> Dict[str, Any]:
        """Tarayıcıyı başlat (senkron)"""
        loop = self._get_loop()
        self._agent.config.headless = headless
        return loop.run_until_complete(self._agent.initialize())
    
    def execute_task(self, task: str) -> Dict[str, Any]:
        """Görev çalıştır (senkron)"""
        loop = self._get_loop()
        result = loop.run_until_complete(self._agent.execute_task(task))
        return {
            "success": result.success,
            "content": result.content,
            "url": result.url,
            "error": result.error,
            "timestamp": result.timestamp
        }
    
    def navigate(self, url: str) -> Dict[str, Any]:
        """URL'ye git (senkron)"""
        loop = self._get_loop()
        result = loop.run_until_complete(self._agent.navigate(url))
        return {
            "success": result.success,
            "content": result.content,
            "url": result.url,
            "error": result.error
        }
    
    def search(self, query: str, engine: str = "google") -> Dict[str, Any]:
        """Web'de ara (senkron)"""
        loop = self._get_loop()
        result = loop.run_until_complete(self._agent.search(query, engine))
        return {
            "success": result.success,
            "content": result.content,
            "url": result.url,
            "error": result.error,
            "search_query": query,
            "engine": engine
        }
    
    def research(self, topic: str, depth: int = 3) -> Dict[str, Any]:
        """Araştırma yap (senkron)"""
        loop = self._get_loop()
        result = loop.run_until_complete(self._agent.research(topic, depth))
        return {
            "query": result.query,
            "summary": result.summary,
            "sources": result.sources,
            "findings": result.findings,
            "confidence": result.confidence,
            "duration_seconds": result.duration_seconds
        }
    
    def screenshot(self, full_page: bool = False) -> Dict[str, Any]:
        """Ekran görüntüsü al (senkron)"""
        loop = self._get_loop()
        result = loop.run_until_complete(self._agent.screenshot(full_page))
        return {
            "success": result.success,
            "screenshot": result.screenshot,
            "url": result.url,
            "error": result.error
        }
    
    def extract_content(self, selector: Optional[str] = None) -> Dict[str, Any]:
        """İçerik çıkar (senkron)"""
        loop = self._get_loop()
        result = loop.run_until_complete(self._agent.extract_content(selector))
        return {
            "success": result.success,
            "content": result.content,
            "links": result.links,
            "url": result.url,
            "error": result.error
        }
    
    def click(self, selector: str) -> Dict[str, Any]:
        """Elemente tıkla (senkron)"""
        loop = self._get_loop()
        result = loop.run_until_complete(self._agent.click(selector))
        return {
            "success": result.success,
            "content": result.content,
            "url": result.url,
            "error": result.error
        }
    
    def type_text(self, selector: str, text: str, press_enter: bool = False) -> Dict[str, Any]:
        """Yazı yaz (senkron)"""
        loop = self._get_loop()
        result = loop.run_until_complete(self._agent.type_text(selector, text, press_enter))
        return {
            "success": result.success,
            "content": result.content,
            "url": result.url,
            "error": result.error
        }
    
    def get_history(self) -> List[Dict[str, Any]]:
        """Geçmişi al"""
        return self._agent.get_history()
    
    def close(self) -> Dict[str, Any]:
        """Tarayıcıyı kapat (senkron)"""
        loop = self._get_loop()
        return loop.run_until_complete(self._agent.close())


# ─── SENTIENT Modül Metadata ───

SENTIENT_MODULE = {
    "name": "browser_use",
    "version": "0.2.0",
    "description": "SENTIENT Tarayıcı Otomasyon ve Araştırma Modülü",
    "capabilities": [
        "otonom_web_gezintisi",
        "akilli_arama",
        "derinlemesine_arastirma",
        "sayfa_analizi",
        "ekran_goruntusu",
        "form_doldurma",
        "veri_cikarma"
    ],
    "tools": [
        {
            "name": "browser_init",
            "function": "initialize",
            "description": "Tarayıcıyı başlatır",
            "args": {"headless": "bool, varsayılan True"}
        },
        {
            "name": "browser_task",
            "function": "execute_task",
            "description": "Doğal dille bir görevi çalıştırır",
            "args": {"task": "str - Yapılacak görev"}
        },
        {
            "name": "browser_navigate",
            "function": "navigate",
            "description": "Belirtilen URL'ye gider",
            "args": {"url": "str - Hedef URL"}
        },
        {
            "name": "browser_search",
            "function": "search",
            "description": "Web'de arama yapar",
            "args": {
                "query": "str - Arama sorgusu",
                "engine": "str - google/duckduckgo/bing"
            }
        },
        {
            "name": "browser_research",
            "function": "research",
            "description": "Derinlemesine araştırma yapar",
            "args": {
                "topic": "str - Araştırma konusu",
                "depth": "int - Derinlik (varsayılan 3)"
            }
        },
        {
            "name": "browser_screenshot",
            "function": "screenshot",
            "description": "Ekran görüntüsü alır",
            "args": {"full_page": "bool - Tüm sayfa (varsayılan False)"}
        },
        {
            "name": "browser_extract",
            "function": "extract_content",
            "description": "Sayfa içeriğini çıkarır",
            "args": {"selector": "str - CSS seçici (opsiyonel)"}
        },
        {
            "name": "browser_click",
            "function": "click",
            "description": "Bir elemente tıklar",
            "args": {"selector": "str - CSS seçici"}
        },
        {
            "name": "browser_type",
            "function": "type_text",
            "description": "Bir input alanına yazı yazar",
            "args": {
                "selector": "str - CSS seçici",
                "text": "str - Yazılacak metin",
                "press_enter": "bool - Enter'a bas (varsayılan False)"
            }
        },
        {
            "name": "browser_history",
            "function": "get_history",
            "description": "Görev geçmişini döndürür"
        },
        {
            "name": "browser_close",
            "function": "close",
            "description": "Tarayıcıyı kapatır"
        }
    ]
}


# ─── Kolay Erişim Fonksiyonları ───

def create_browser(headless: bool = True) -> SENTIENTBrowserSync:
    """Yeni tarayıcı örneği oluştur"""
    browser = SENTIENTBrowserSync()
    browser.initialize(headless=headless)
    return browser


def quick_search(query: str, engine: str = "google") -> Dict[str, Any]:
    """Hızlı arama yap"""
    browser = create_browser()
    result = browser.search(query, engine)
    browser.close()
    return result


def quick_research(topic: str, depth: int = 2) -> Dict[str, Any]:
    """Hızlı araştırma yap"""
    browser = create_browser()
    result = browser.research(topic, depth)
    browser.close()
    return result
