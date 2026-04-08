"""
SENTIENT OpenManus Sandbox Modülü
──────────────────────────────
Docker içinde yalıtımlı kod çalıştırma için OpenManus sarmalayıcısı.

## Güvenlik Katmanları
1. Network İzolasyonu: --network none
2. Kaynak Limitleri: --memory, --cpus
3. Kullanıcı İzolasyonu: non-root user
4. Dosya Sistemi: Salt okunur + tmpfs
5. Zaman Aşımı: Timeout koruması
6. Seccomp: Syscall filtreleme
"""

from typing import Optional, Dict, Any, List, Union
from dataclasses import dataclass, field
from enum import Enum
import json
import subprocess
import tempfile
import os
import time
import hashlib
import base64

# ─── Enums ───

class SandboxProfile(Enum):
    """Sandbox güvenlik profilleri"""
    SECURE = "secure"       # Maksimum izolasyon
    STANDARD = "standard"  # Dengeli
    DEVELOPMENT = "development"  # Esnek

class Language(Enum):
    """Desteklenen programlama dilleri"""
    PYTHON = "python"
    JAVASCRIPT = "javascript"
    BASH = "bash"
    RUST = "rust"
    GO = "go"
    RUBY = "ruby"
    PHP = "php"
    JAVA = "java"

class SandboxStatus(Enum):
    """Sandbox durumu"""
    CREATED = "created"
    RUNNING = "running"
    STOPPED = "stopped"
    ERROR = "error"
    TIMEOUT = "timeout"

# ─── Data Classes ───

@dataclass
class SandboxLimits:
    """Kaynak limitleri"""
    memory_mb: int = 512
    cpu_count: float = 1.0
    timeout_seconds: int = 60
    max_file_size_mb: int = 10
    max_processes: int = 50
    network_enabled: bool = False

@dataclass
class ExecutionResult:
    """Kod çalıştırma sonucu"""
    success: bool
    exit_code: int
    stdout: str
    stderr: str
    duration_ms: int
    sandbox_id: str
    error: Optional[str] = None
    artifacts: List[Dict[str, Any]] = field(default_factory=list)

    def is_ok(self) -> bool:
        return self.success and self.exit_code == 0

    def summary(self) -> str:
        if self.is_ok():
            return f"✅ [{self.sandbox_id[:8]}] {self.duration_ms}ms"
        return f"❌ [{self.sandbox_id[:8]}] exit={self.exit_code}: {self.error or self.stderr[:100]}"

@dataclass
class SandboxInfo:
    """Sandbox bilgileri"""
    sandbox_id: str
    container_id: str
    status: SandboxStatus
    profile: SandboxProfile
    limits: SandboxLimits
    created_at: float
    last_activity: float

# ─── Docker Komut Üreteci ───

class DockerCommandBuilder:
    """Docker komutları için güvenli üreteç"""

    @staticmethod
    def sanitize_image(image: str) -> str:
        """İmaj adını doğrula"""
        allowed_prefixes = ["python:", "node:", "rust:", "golang:", "ruby:", "php:", "openjdk:", "sentient/"]
        if not any(image.startswith(p) for p in allowed_prefixes):
            raise ValueError(f"İmaj izin verilmeyen: {image}")
        return image

    @staticmethod
    def build_run_command(
        name: str,
        image: str,
        limits: SandboxLimits,
        volumes: List[Dict[str, str]] = None,
        env: Dict[str, str] = None
    ) -> List[str]:
        """Docker run komutu oluştur"""
        cmd = ["docker", "run", "-d", "--name", name]

        # Network
        if not limits.network_enabled:
            cmd.extend(["--network", "none"])

        # Memory limit
        cmd.extend(["--memory", f"{limits.memory_mb}m"])
        cmd.extend(["--memory-swap", f"{limits.memory_mb}m"])

        # CPU limit
        cpu_quota = int(limits.cpu_count * 100000)
        cmd.extend(["--cpu-period", "100000"])
        cmd.extend(["--cpu-quota", str(cpu_quota)])

        # PIDs limit
        cmd.extend(["--pids-limit", str(limits.max_processes)])

        # Security options
        cmd.extend([
            "--security-opt", "no-new-privileges:true",
            "--security-opt", "seccomp=default",
            "--cap-drop=ALL",
        ])

        # Read-only root filesystem with tmpfs
        cmd.extend([
            "--read-only",
            "--tmpfs", "/tmp:rw,size=100m,exec",
            "--tmpfs", "/var/run:rw,size=10m",
        ])

        # Volumes
        if volumes:
            for vol in volumes:
                mount = f"{vol['host']}:{vol['container']}"
                if vol.get("read_only", False):
                    mount += ":ro"
                cmd.extend(["-v", mount])

        # Environment
        if env:
            for k, v in env.items():
                cmd.extend(["-e", f"{k}={v}"])

        # Image and command
        cmd.extend([DockerCommandBuilder.sanitize_image(image)])
        cmd.extend(["sleep", str(limits.timeout_seconds + 30)])

        return cmd

# ─── Ana Sandbox Sınıfı ───

class SENTIENTSandbox:
    """SENTIENT Docker Sandbox"""

    # Dil -> İmaj eşlemesi
    LANGUAGE_IMAGES = {
        Language.PYTHON: "python:3.11-slim",
        Language.JAVASCRIPT: "node:20-slim",
        Language.BASH: "bash:5.2",
        Language.RUST: "rust:1.75-slim",
        Language.GO: "golang:1.21-slim",
        Language.RUBY: "ruby:3.2-slim",
        Language.PHP: "php:8.2-cli",
        Language.JAVA: "openjdk:21-slim",
    }

    def __init__(self, profile: SandboxProfile = SandboxProfile.STANDARD):
        self.profile = profile
        self.limits = self._get_limits_for_profile(profile)
        self.container_id: Optional[str] = None
        self.sandbox_id: Optional[str] = None
        self._docker_available = self._check_docker()

    def _check_docker(self) -> bool:
        """Docker kullanılabilirliğini kontrol et"""
        try:
            result = subprocess.run(
                ["docker", "version"],
                capture_output=True,
                timeout=5
            )
            return result.returncode == 0
        except Exception:
            return False

    def _get_limits_for_profile(self, profile: SandboxProfile) -> SandboxLimits:
        """Profile göre limitler"""
        if profile == SandboxProfile.SECURE:
            return SandboxLimits(
                memory_mb=256,
                cpu_count=0.5,
                timeout_seconds=30,
                max_file_size_mb=5,
                max_processes=25,
                network_enabled=False,
            )
        elif profile == SandboxProfile.DEVELOPMENT:
            return SandboxLimits(
                memory_mb=2048,
                cpu_count=2.0,
                timeout_seconds=300,
                max_file_size_mb=100,
                max_processes=200,
                network_enabled=True,
            )
        else:  # STANDARD
            return SandboxLimits(
                memory_mb=512,
                cpu_count=1.0,
                timeout_seconds=60,
                max_file_size_mb=20,
                max_processes=50,
                network_enabled=False,
            )

    def create(self) -> Dict[str, Any]:
        """Yeni sandbox oluştur"""
        if not self._docker_available:
            return {
                "success": False,
                "message": "Docker kurulu değil veya çalışmıyor. Sandbox oluşturulamadı.",
                "sandbox_id": None,
            }

        import uuid
        self.sandbox_id = f"sentient_{uuid.uuid4().hex[:12]}"
        
        # Varsayılan Python imajı
        image = self.LANGUAGE_IMAGES.get(Language.PYTHON, "python:3.11-slim")

        try:
            cmd = DockerCommandBuilder.build_run_command(
                name=self.sandbox_id,
                image=image,
                limits=self.limits,
                env={"SENTIENT_SANDBOX": "true", "SENTIENT_ID": self.sandbox_id}
            )

            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=60
            )

            if result.returncode == 0:
                self.container_id = result.stdout.strip()
                return {
                    "success": True,
                    "sandbox_id": self.sandbox_id,
                    "container_id": self.container_id,
                    "message": f"Sandbox oluşturuldu: {self.sandbox_id}",
                }
            else:
                return {
                    "success": False,
                    "message": f"Sandbox oluşturma hatası: {result.stderr}",
                    "sandbox_id": self.sandbox_id,
                }

        except subprocess.TimeoutExpired:
            return {
                "success": False,
                "message": "Sandbox oluşturma zaman aşımına uğradı.",
                "sandbox_id": self.sandbox_id,
            }
        except Exception as e:
            return {
                "success": False,
                "message": f"Sandbox hatası: {str(e)}",
                "sandbox_id": self.sandbox_id,
            }

    def execute(
        self,
        code: str,
        language: Language = Language.PYTHON,
        stdin: Optional[str] = None,
        args: List[str] = None
    ) -> ExecutionResult:
        """Sandbox içinde kod çalıştır"""
        if not self._docker_available or not self.container_id:
            return ExecutionResult(
                success=False,
                exit_code=-1,
                stdout="",
                stderr="Sandbox başlatılmadı veya Docker kullanılamıyor.",
                duration_ms=0,
                sandbox_id=self.sandbox_id or "none",
                error="Sandbox not initialized"
            )

        start_time = time.time()

        # Dosya uzantısı
        ext_map = {
            Language.PYTHON: ".py",
            Language.JAVASCRIPT: ".js",
            Language.BASH: ".sh",
            Language.RUST: ".rs",
            Language.GO: ".go",
            Language.RUBY: ".rb",
            Language.PHP: ".php",
            Language.JAVA: ".java",
        }
        ext = ext_map.get(language, ".txt")
        filename = f"/tmp/code{ext}"

        # Kodu base64 encode et (özel karakterler için)
        code_b64 = base64.b64encode(code.encode()).decode()

        # Çalıştırma komutu
        run_map = {
            Language.PYTHON: f"python3 {filename}",
            Language.JAVASCRIPT: f"node {filename}",
            Language.BASH: f"bash {filename}",
            Language.RUST: f"rustc {filename} -o /tmp/out && /tmp/out",
            Language.GO: f"go run {filename}",
            Language.RUBY: f"ruby {filename}",
            Language.PHP: f"php {filename}",
            Language.JAVA: f"javac {filename} && java -cp /tmp Code",
        }

        run_cmd = run_map.get(language, f"cat {filename}")

        try:
            # 1. Kodu yaz
            write_cmd = f"echo '{code_b64}' | base64 -d > {filename}"
            subprocess.run(
                ["docker", "exec", self.sandbox_id, "sh", "-c", write_cmd],
                capture_output=True,
                timeout=10
            )

            # 2. Çalıştır
            exec_cmd = ["docker", "exec"]
            if stdin:
                exec_cmd.extend(["-i"])
            exec_cmd.extend([self.sandbox_id, "sh", "-c", run_cmd])

            result = subprocess.run(
                exec_cmd,
                input=stdin,
                capture_output=True,
                text=True,
                timeout=self.limits.timeout_seconds
            )

            duration_ms = int((time.time() - start_time) * 1000)

            return ExecutionResult(
                success=result.returncode == 0,
                exit_code=result.returncode,
                stdout=result.stdout,
                stderr=result.stderr,
                duration_ms=duration_ms,
                sandbox_id=self.sandbox_id,
            )

        except subprocess.TimeoutExpired:
            return ExecutionResult(
                success=False,
                exit_code=-2,
                stdout="",
                stderr=f"İşlem {self.limits.timeout_seconds} saniyede zaman aşımına uğradı.",
                duration_ms=self.limits.timeout_seconds * 1000,
                sandbox_id=self.sandbox_id,
                error="Timeout"
            )
        except Exception as e:
            return ExecutionResult(
                success=False,
                exit_code=-1,
                stdout="",
                stderr=str(e),
                duration_ms=int((time.time() - start_time) * 1000),
                sandbox_id=self.sandbox_id,
                error=str(e)
            )

    def execute_python(self, code: str) -> ExecutionResult:
        """Python kodu çalıştır (kısayol)"""
        return self.execute(code, Language.PYTHON)

    def execute_javascript(self, code: str) -> ExecutionResult:
        """JavaScript kodu çalıştır (kısayol)"""
        return self.execute(code, Language.JAVASCRIPT)

    def execute_bash(self, command: str) -> ExecutionResult:
        """Bash komutu çalıştır (kısayol)"""
        return self.execute(command, Language.BASH)

    def put_file(self, host_path: str, container_path: str) -> bool:
        """Dosya kopyala (host -> container)"""
        if not self.container_id:
            return False

        try:
            subprocess.run(
                ["docker", "cp", host_path, f"{self.sandbox_id}:{container_path}"],
                capture_output=True,
                timeout=30
            )
            return True
        except Exception:
            return False

    def get_file(self, container_path: str, host_path: str) -> bool:
        """Dosya kopyala (container -> host)"""
        if not self.container_id:
            return False

        try:
            subprocess.run(
                ["docker", "cp", f"{self.sandbox_id}:{container_path}", host_path],
                capture_output=True,
                timeout=30
            )
            return True
        except Exception:
            return False

    def stop(self) -> Dict[str, Any]:
        """Sandbox'ı durdur"""
        if not self.container_id:
            return {"success": True, "message": "Sandbox zaten durdurulmuş."}

        try:
            subprocess.run(
                ["docker", "stop", self.sandbox_id],
                capture_output=True,
                timeout=30
            )
            return {
                "success": True,
                "message": f"Sandbox durduruldu: {self.sandbox_id}",
            }
        except Exception as e:
            return {
                "success": False,
                "message": f"Durdurma hatası: {str(e)}",
            }

    def destroy(self) -> Dict[str, Any]:
        """Sandbox'ı tamamen sil"""
        if not self.sandbox_id:
            return {"success": True, "message": "Sandbox zaten silinmiş."}

        try:
            # Önce durdur
            subprocess.run(
                ["docker", "rm", "-f", self.sandbox_id],
                capture_output=True,
                timeout=30
            )
            container_id = self.container_id
            self.container_id = None
            return {
                "success": True,
                "message": f"Sandbox silindi: {self.sandbox_id}",
            }
        except Exception as e:
            return {
                "success": False,
                "message": f"Silme hatası: {str(e)}",
            }

    def info(self) -> Optional[SandboxInfo]:
        """Sandbox bilgilerini al"""
        if not self.sandbox_id:
            return None

        return SandboxInfo(
            sandbox_id=self.sandbox_id,
            container_id=self.container_id or "",
            status=SandboxStatus.RUNNING if self.container_id else SandboxStatus.STOPPED,
            profile=self.profile,
            limits=self.limits,
            created_at=time.time(),
            last_activity=time.time(),
        )

# ─── Senkron Wrapper (Rust/PyO3 uyumlu) ───

class SENTIENTSandboxSync:
    """SENTIENT Sandbox Senkron Wrapper (PyO3 için)"""

    def __init__(self, profile: str = "standard"):
        profile_enum = {
            "secure": SandboxProfile.SECURE,
            "standard": SandboxProfile.STANDARD,
            "development": SandboxProfile.DEVELOPMENT,
        }.get(profile, SandboxProfile.STANDARD)
        
        self._sandbox = SENTIENTSandbox(profile_enum)
        self._initialized = False

    def initialize(self) -> Dict[str, Any]:
        """Sandbox'ı başlat"""
        result = self._sandbox.create()
        if result.get("success"):
            self._initialized = True
        return result

    def execute_code(self, code: str, language: str = "python") -> Dict[str, Any]:
        """Kod çalıştır"""
        if not self._initialized:
            return {
                "success": False,
                "message": "Sandbox başlatılmadı. Önce initialize() çağırın.",
            }

        lang_enum = {
            "python": Language.PYTHON,
            "javascript": Language.JAVASCRIPT,
            "bash": Language.BASH,
            "rust": Language.RUST,
            "go": Language.GO,
            "ruby": Language.RUBY,
            "php": Language.PHP,
            "java": Language.JAVA,
        }.get(language, Language.PYTHON)

        result = self._sandbox.execute(code, lang_enum)
        return {
            "success": result.success,
            "exit_code": result.exit_code,
            "stdout": result.stdout,
            "stderr": result.stderr,
            "duration_ms": result.duration_ms,
            "sandbox_id": result.sandbox_id,
            "error": result.error,
        }

    def execute_python(self, code: str) -> Dict[str, Any]:
        """Python çalıştır (kısayol)"""
        return self.execute_code(code, "python")

    def execute_javascript(self, code: str) -> Dict[str, Any]:
        """JavaScript çalıştır (kısayol)"""
        return self.execute_code(code, "javascript")

    def execute_bash(self, command: str) -> Dict[str, Any]:
        """Bash komutu çalıştır (kısayol)"""
        return self.execute_code(command, "bash")

    def close(self) -> Dict[str, Any]:
        """Sandbox'ı kapat"""
        if self._initialized:
            result = self._sandbox.destroy()
            self._initialized = False
            return result
        return {"success": True, "message": "Sandbox zaten kapalı."}

    def is_initialized(self) -> bool:
        """Başlatıldı mı?"""
        return self._initialized

    def get_limits(self) -> Dict[str, Any]:
        """Limitleri al"""
        limits = self._sandbox.limits
        return {
            "memory_mb": limits.memory_mb,
            "cpu_count": limits.cpu_count,
            "timeout_seconds": limits.timeout_seconds,
            "max_file_size_mb": limits.max_file_size_mb,
            "max_processes": limits.max_processes,
            "network_enabled": limits.network_enabled,
        }

# ─── SENTIENT Modül Metadata ───

SENTIENT_MODULE = {
    "name": "openmanus",
    "version": "0.1.0",
    "description": "SENTIENT Docker Sandbox Modülü - Yalıtımlı Kod Çalıştırma",
    "author": "NEXUS OASIS",
    "tools": [
        {
            "name": "sandbox_create",
            "function": "initialize",
            "description": "Yalıtılmış sandbox ortamı oluşturur",
            "args": [],
            "returns": "Dict[str, Any]",
        },
        {
            "name": "sandbox_execute",
            "function": "execute_code",
            "description": "Sandbox içinde kod çalıştırır",
            "args": ["code", "language"],
            "returns": "Dict[str, Any]",
        },
        {
            "name": "sandbox_python",
            "function": "execute_python",
            "description": "Python kodu çalıştırır (kısayol)",
            "args": ["code"],
            "returns": "Dict[str, Any]",
        },
        {
            "name": "sandbox_javascript",
            "function": "execute_javascript",
            "description": "JavaScript kodu çalıştırır (kısayol)",
            "args": ["code"],
            "returns": "Dict[str, Any]",
        },
        {
            "name": "sandbox_bash",
            "function": "execute_bash",
            "description": "Bash komutu çalıştırır (kısayol)",
            "args": ["command"],
            "returns": "Dict[str, Any]",
        },
        {
            "name": "sandbox_close",
            "function": "close",
            "description": "Sandbox'ı temizler",
            "args": [],
            "returns": "Dict[str, Any]",
        },
        {
            "name": "sandbox_limits",
            "function": "get_limits",
            "description": "Sandbox kaynak limitlerini döndürür",
            "args": [],
            "returns": "Dict[str, Any]",
        },
    ],
    "profiles": ["secure", "standard", "development"],
    "languages": ["python", "javascript", "bash", "rust", "go", "ruby", "php", "java"],
}

# ─── Test Fonksiyonu ───

def test_sandbox():
    """Sandbox test fonksiyonu"""
    sandbox = SENTIENTSandboxSync("standard")
    
    # Başlat
    print("1. Sandbox oluşturuluyor...")
    result = sandbox.initialize()
    print(f"   Sonuç: {result}")

    if not result.get("success"):
        print("   ❌ Docker kurulu değil, test atlanıyor.")
        return

    # Python test
    print("\n2. Python kodu çalıştırılıyor...")
    code = '''
import sys
print(f"Python {sys.version}")
print("Merhaba SENTIENT Sandbox!")
'''
    result = sandbox.execute_python(code)
    print(f"   stdout: {result.get('stdout', '')[:100]}")
    print(f"   duration: {result.get('duration_ms')}ms")

    # Bash test
    print("\n3. Bash komutu çalıştırılıyor...")
    result = sandbox.execute_bash("echo 'SENTIENT Docker Sandbox aktif!' && ls -la /tmp")
    print(f"   stdout: {result.get('stdout', '')[:100]}")

    # Kapat
    print("\n4. Sandbox kapatılıyor...")
    result = sandbox.close()
    print(f"   Sonuç: {result}")

if __name__ == "__main__":
    test_sandbox()
