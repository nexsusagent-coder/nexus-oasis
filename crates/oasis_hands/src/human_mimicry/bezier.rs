//! ═══════════════════════════════════════════════════════════════════════════════
//!  BEZIER CURVES - Matematiksel Yörünge Hesaplama
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Doğal fare hareketleri için Bezier eğrisi implementasyonu.
//! Cubic Bezier eğrileri kullanılarak insan benzeri yörüngeler oluşturulur.

use rand::Rng;
use serde::{Deserialize, Serialize};

/// Bezier noktası (x, y koordinatları)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BezierPoint {
    pub x: f64,
    pub y: f64,
}

impl BezierPoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Bezier eğrisi trait'i
pub trait BezierCurve {
    /// Eğri üzerindeki noktayı hesapla (t: 0.0 - 1.0)
    fn point_at(&self, t: f64) -> BezierPoint;
    
    /// Eğrinin tüm noktalarını oluştur
    fn generate_points(&self, segments: u32) -> Vec<BezierPoint>;
}

/// Cubic Bezier eğrisi (4 kontrol noktası)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CubicBezier {
    /// Başlangıç noktası
    pub p0: BezierPoint,
    /// İlk kontrol noktası
    pub p1: BezierPoint,
    /// İkinci kontrol noktası
    pub p2: BezierPoint,
    /// Bitiş noktası
    pub p3: BezierPoint,
}

impl CubicBezier {
    /// Yeni Cubic Bezier oluştur
    pub fn new(p0: BezierPoint, p1: BezierPoint, p2: BezierPoint, p3: BezierPoint) -> Self {
        Self { p0, p1, p2, p3 }
    }
    
    /// İki nokta arasında doğal Cubic Bezier oluştur
    pub fn between(from: (f64, f64), to: (f64, f64)) -> Self {
        let mut rng = rand::thread_rng();
        
        // Mesafeyi hesapla
        let dx = to.0 - from.0;
        let dy = to.1 - from.1;
        let distance = (dx * dx + dy * dy).sqrt();
        
        // Kontrol noktaları için rastgele offset
        // İnsan hareketlerinde kontrol noktaları genelde yolun ortasında olur
        let offset_factor = distance * 0.3;
        
        // Rastgele eğim açısı (doğal hareket için)
        let angle1 = rng.gen_range(0.0..std::f64::consts::PI);
        let angle2 = rng.gen_range(0.0..std::f64::consts::PI);
        
        // İlk kontrol noktası (başlangıca yakın)
        let cp1 = BezierPoint::new(
            from.0 + dx * 0.25 + angle1.cos() * offset_factor * rng.gen_range(0.3..1.0),
            from.1 + dy * 0.25 + angle1.sin() * offset_factor * rng.gen_range(0.3..1.0),
        );
        
        // İkinci kontrol noktası (bitişe yakın)
        let cp2 = BezierPoint::new(
            from.0 + dx * 0.75 + angle2.cos() * offset_factor * rng.gen_range(0.3..1.0),
            from.1 + dy * 0.75 + angle2.sin() * offset_factor * rng.gen_range(0.3..1.0),
        );
        
        Self {
            p0: BezierPoint::new(from.0, from.1),
            p1: cp1,
            p2: cp2,
            p3: BezierPoint::new(to.0, to.1),
        }
    }
    
    /// Eğrinin türevini hesapla (teğet vektör)
    pub fn tangent_at(&self, t: f64) -> (f64, f64) {
        let t2 = t * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        
        // Türev formülü
        let dx = 3.0 * mt2 * (self.p1.x - self.p0.x)
               + 6.0 * mt * t * (self.p2.x - self.p1.x)
               + 3.0 * t2 * (self.p3.x - self.p2.x);
        
        let dy = 3.0 * mt2 * (self.p1.y - self.p0.y)
               + 6.0 * mt * t * (self.p2.y - self.p1.y)
               + 3.0 * t2 * (self.p3.y - self.p2.y);
        
        (dx, dy)
    }
    
    /// Eğrinin uzunluğunu tahmin et
    pub fn estimate_length(&self, segments: u32) -> f64 {
        let points = self.generate_points(segments);
        let mut length = 0.0;
        
        for i in 1..points.len() {
            let dx = points[i].x - points[i-1].x;
            let dy = points[i].y - points[i-1].y;
            length += (dx * dx + dy * dy).sqrt();
        }
        
        length
    }
}

impl BezierCurve for CubicBezier {
    fn point_at(&self, t: f64) -> BezierPoint {
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;
        
        // Cubic Bezier formülü: B(t) = (1-t)³P₀ + 3(1-t)²tP₁ + 3(1-t)t²P₂ + t³P₃
        BezierPoint {
            x: mt3 * self.p0.x + 3.0 * mt2 * t * self.p1.x + 3.0 * mt * t2 * self.p2.x + t3 * self.p3.x,
            y: mt3 * self.p0.y + 3.0 * mt2 * t * self.p1.y + 3.0 * mt * t2 * self.p2.y + t3 * self.p3.y,
        }
    }
    
    fn generate_points(&self, segments: u32) -> Vec<BezierPoint> {
        (0..=segments)
            .map(|i| self.point_at(i as f64 / segments as f64))
            .collect()
    }
}

/// Quadratic Bezier eğrisi (3 kontrol noktası)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuadraticBezier {
    pub p0: BezierPoint,
    pub p1: BezierPoint,
    pub p2: BezierPoint,
}

impl BezierCurve for QuadraticBezier {
    fn point_at(&self, t: f64) -> BezierPoint {
        let mt = 1.0 - t;
        
        BezierPoint {
            x: mt * mt * self.p0.x + 2.0 * mt * t * self.p1.x + t * t * self.p2.x,
            y: mt * mt * self.p0.y + 2.0 * mt * t * self.p1.y + t * t * self.p2.y,
        }
    }
    
    fn generate_points(&self, segments: u32) -> Vec<BezierPoint> {
        (0..=segments)
            .map(|i| self.point_at(i as f64 / segments as f64))
            .collect()
    }
}

/// Bezier Motoru
pub struct BezierEngine {
    /// Segment sayısı (kalite)
    quality: u32,
}

impl BezierEngine {
    pub fn new(quality: u32) -> Self {
        Self { quality }
    }
    
    /// İki nokta arasında insan benzeri yol oluştur
    pub fn generate_path(&self, from: (f64, f64), to: (f64, f64)) -> Vec<(f64, f64)> {
        let bezier = CubicBezier::between(from, to);
        bezier.generate_points(self.quality)
            .into_iter()
            .map(|p| (p.x, p.y))
            .collect()
    }
    
    /// Çoklu noktalar için yol oluştur (yol boyunca hareket)
    pub fn generate_multi_point_path(&self, points: &[(f64, f64)]) -> Vec<(f64, f64)> {
        if points.len() < 2 {
            return points.to_vec();
        }
        
        let mut path = Vec::new();
        
        for i in 0..points.len() - 1 {
            let segment = self.generate_path(points[i], points[i + 1]);
            let seg_len = segment.len();
            
            // Son noktayı hariç tut (çakışmayı önle)
            if i < points.len() - 2 {
                path.extend(segment.into_iter().take(seg_len.saturating_sub(1)));
            } else {
                path.extend(segment);
            }
        }
        
        path
    }
    
    /// Eğriye varyasyon ekle (insan hatası simülasyonu)
    pub fn add_variation(&self, path: &[(f64, f64)], intensity: f64) -> Vec<(f64, f64)> {
        let mut rng = rand::thread_rng();
        
        path.iter()
            .map(|(x, y)| {
                let vx = (rng.gen::<f64>() - 0.5) * intensity * 2.0;
                let vy = (rng.gen::<f64>() - 0.5) * intensity * 2.0;
                (x + vx, y + vy)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bezier_point_creation() {
        let point = BezierPoint::new(10.0, 20.0);
        assert_eq!(point.x, 10.0);
        assert_eq!(point.y, 20.0);
    }
    
    #[test]
    fn test_cubic_bezier_point_at() {
        let bezier = CubicBezier::new(
            BezierPoint::new(0.0, 0.0),
            BezierPoint::new(0.0, 100.0),
            BezierPoint::new(100.0, 100.0),
            BezierPoint::new(100.0, 0.0),
        );
        
        // t=0 başlangıç noktası
        let p0 = bezier.point_at(0.0);
        assert_eq!(p0.x, 0.0);
        assert_eq!(p0.y, 0.0);
        
        // t=1 bitiş noktası
        let p1 = bezier.point_at(1.0);
        assert_eq!(p1.x, 100.0);
        assert_eq!(p1.y, 0.0);
    }
    
    #[test]
    fn test_cubic_bezier_between() {
        let bezier = CubicBezier::between((0.0, 0.0), (100.0, 100.0));
        
        let points = bezier.generate_points(10);
        assert_eq!(points.len(), 11);
        
        // İlk ve son noktalar doğru olmalı
        assert!((points[0].x - 0.0).abs() < 0.001);
        assert!((points[0].y - 0.0).abs() < 0.001);
        assert!((points[10].x - 100.0).abs() < 0.001);
        assert!((points[10].y - 100.0).abs() < 0.001);
    }
    
    #[test]
    fn test_bezier_engine() {
        let engine = BezierEngine::new(50);
        let path = engine.generate_path((0.0, 0.0), (100.0, 100.0));
        
        assert_eq!(path.len(), 51);
    }
    
    #[test]
    fn test_bezier_estimate_length() {
        let bezier = CubicBezier::between((0.0, 0.0), (100.0, 0.0));
        let length = bezier.estimate_length(100);
        
        // Düz çizgiye yakın olmalı
        assert!(length > 90.0 && length < 150.0);
    }
}
