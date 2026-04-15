//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT Python Data Bridge - Data Processing & ML Integration
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Data structures and configurations for Python data processing.
//!  Actual Python calls are handled by sentient_python::PythonBridge.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
//  NUMPY TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// NumPy array wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumpyArray {
    pub data: Vec<f64>,
    pub shape: Vec<usize>,
    pub dtype: String,
}

impl NumpyArray {
    pub fn new_1d(data: Vec<f64>) -> Self {
        let shape = vec![data.len()];
        Self { data, shape, dtype: "float64".into() }
    }
    
    pub fn new_2d(data: Vec<Vec<f64>>) -> Self {
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };
        let flat: Vec<f64> = data.into_iter().flatten().collect();
        Self { data: flat, shape: vec![rows, cols], dtype: "float64".into() }
    }
    
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    pub fn reshape(&mut self, new_shape: Vec<usize>) {
        let new_size: usize = new_shape.iter().product();
        assert_eq!(new_size, self.data.len(), "Shape mismatch");
        self.shape = new_shape;
    }
}

/// Statistics result from NumPy computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsResult {
    pub mean: f64,
    pub std: f64,
    pub min: f64,
    pub max: f64,
    pub median: f64,
    pub variance: f64,
}

impl Default for StatsResult {
    fn default() -> Self {
        Self {
            mean: 0.0,
            std: 0.0,
            min: 0.0,
            max: 0.0,
            median: 0.0,
            variance: 0.0,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PANDAS TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Pandas DataFrame wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFrame {
    pub columns: Vec<String>,
    pub data: Vec<Vec<serde_json::Value>>,
    pub index: Vec<usize>,
    pub dtypes: HashMap<String, String>,
}

impl DataFrame {
    pub fn new(columns: Vec<String>) -> Self {
        let dtypes = columns.iter()
            .map(|c| (c.clone(), "object".into()))
            .collect();
        Self {
            columns,
            data: Vec::new(),
            index: Vec::new(),
            dtypes,
        }
    }
    
    pub fn add_row(&mut self, row: Vec<serde_json::Value>) {
        self.index.push(self.data.len());
        self.data.push(row);
    }
    
    pub fn shape(&self) -> (usize, usize) {
        (self.data.len(), self.columns.len())
    }
    
    pub fn get_column(&self, name: &str) -> Option<Vec<&serde_json::Value>> {
        let col_idx = self.columns.iter().position(|c| c == name)?;
        Some(self.data.iter().filter_map(|row| row.get(col_idx)).collect())
    }
    
    pub fn filter(&self, column: &str, predicate: impl Fn(&serde_json::Value) -> bool) -> DataFrame {
        let mut result = DataFrame::new(self.columns.clone());
        let col_idx = self.columns.iter().position(|c| c == column);
        
        if let Some(idx) = col_idx {
            for row in &self.data {
                if let Some(val) = row.get(idx) {
                    if predicate(val) {
                        result.add_row(row.clone());
                    }
                }
            }
        }
        
        result
    }
}

/// DataFrame metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFrameInfo {
    pub rows: usize,
    pub columns: usize,
    pub column_names: Vec<String>,
    pub dtypes: HashMap<String, String>,
    pub memory_usage_bytes: usize,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SCIKIT-LEARN TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Scikit-learn model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SklearnModelConfig {
    pub model_type: SklearnModelType,
    pub hyperparameters: HashMap<String, serde_json::Value>,
    pub random_state: Option<u64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SklearnModelType {
    RandomForestClassifier,
    RandomForestRegressor,
    SVC,
    SVR,
    LogisticRegression,
    LinearRegression,
    KNeighborsClassifier,
    KNeighborsRegressor,
    GradientBoostingClassifier,
    GradientBoostingRegressor,
    DecisionTreeClassifier,
    DecisionTreeRegressor,
    MLPClassifier,
    MLPRegressor,
}

impl SklearnModelType {
    pub fn module_path(&self) -> (&'static str, &'static str) {
        match self {
            Self::RandomForestClassifier => ("sklearn.ensemble", "RandomForestClassifier"),
            Self::RandomForestRegressor => ("sklearn.ensemble", "RandomForestRegressor"),
            Self::SVC => ("sklearn.svm", "SVC"),
            Self::SVR => ("sklearn.svm", "SVR"),
            Self::LogisticRegression => ("sklearn.linear_model", "LogisticRegression"),
            Self::LinearRegression => ("sklearn.linear_model", "LinearRegression"),
            Self::KNeighborsClassifier => ("sklearn.neighbors", "KNeighborsClassifier"),
            Self::KNeighborsRegressor => ("sklearn.neighbors", "KNeighborsRegressor"),
            Self::GradientBoostingClassifier => ("sklearn.ensemble", "GradientBoostingClassifier"),
            Self::GradientBoostingRegressor => ("sklearn.ensemble", "GradientBoostingRegressor"),
            Self::DecisionTreeClassifier => ("sklearn.tree", "DecisionTreeClassifier"),
            Self::DecisionTreeRegressor => ("sklearn.tree", "DecisionTreeRegressor"),
            Self::MLPClassifier => ("sklearn.neural_network", "MLPClassifier"),
            Self::MLPRegressor => ("sklearn.neural_network", "MLPRegressor"),
        }
    }
    
    pub fn is_classifier(&self) -> bool {
        matches!(self, 
            Self::RandomForestClassifier |
            Self::SVC |
            Self::LogisticRegression |
            Self::KNeighborsClassifier |
            Self::GradientBoostingClassifier |
            Self::DecisionTreeClassifier |
            Self::MLPClassifier
        )
    }
}

/// Model evaluation metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1: f64,
    pub auc_roc: Option<f64>,
    pub confusion_matrix: Option<Vec<Vec<u64>>>,
}

impl Default for ModelMetrics {
    fn default() -> Self {
        Self {
            accuracy: 0.0,
            precision: 0.0,
            recall: 0.0,
            f1: 0.0,
            auc_roc: None,
            confusion_matrix: None,
        }
    }
}

/// Training configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    pub test_size: f64,
    pub cross_validation_folds: usize,
    pub shuffle: bool,
    pub random_state: u64,
    pub stratify: bool,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            test_size: 0.2,
            cross_validation_folds: 5,
            shuffle: true,
            random_state: 42,
            stratify: true,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PYTORCH TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// PyTorch tensor wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorchTensor {
    pub data: Vec<f64>,
    pub shape: Vec<usize>,
    pub dtype: String,
    pub device: Device,
    pub requires_grad: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Device {
    Cpu,
    Cuda(usize),
    Mps,
}

impl Default for Device {
    fn default() -> Self {
        Self::Cpu
    }
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cpu => write!(f, "cpu"),
            Self::Cuda(idx) => write!(f, "cuda:{}", idx),
            Self::Mps => write!(f, "mps"),
        }
    }
}

impl TorchTensor {
    pub fn new(data: Vec<f64>, shape: Vec<usize>) -> Self {
        Self {
            data,
            shape,
            dtype: "float32".into(),
            device: Device::Cpu,
            requires_grad: false,
        }
    }
    
    pub fn zeros(shape: Vec<usize>) -> Self {
        let size: usize = shape.iter().product();
        Self::new(vec![0.0; size], shape)
    }
    
    pub fn ones(shape: Vec<usize>) -> Self {
        let size: usize = shape.iter().product();
        Self::new(vec![1.0; size], shape)
    }
    
    pub fn to_device(&mut self, device: Device) {
        self.device = device;
    }
}

/// Neural network model info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub architecture: String,
    pub parameters: usize,
    pub trainable_parameters: usize,
    pub layers: usize,
    pub input_shape: Vec<usize>,
    pub output_shape: Vec<usize>,
    pub device: Device,
    pub dtype: String,
}

/// Training history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingHistory {
    pub epochs: usize,
    pub train_loss: Vec<f64>,
    pub val_loss: Vec<f64>,
    pub train_accuracy: Vec<f64>,
    pub val_accuracy: Vec<f64>,
    pub learning_rates: Vec<f64>,
    pub best_epoch: usize,
    pub best_val_loss: f64,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  VISUALIZATION TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Plot configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotConfig {
    pub title: String,
    pub xlabel: String,
    pub ylabel: String,
    pub figsize: (f64, f64),
    pub dpi: u32,
    pub style: PlotStyle,
    pub legend: bool,
    pub grid: bool,
}

impl Default for PlotConfig {
    fn default() -> Self {
        Self {
            title: String::new(),
            xlabel: String::new(),
            ylabel: String::new(),
            figsize: (10.0, 6.0),
            dpi: 100,
            style: PlotStyle::Seaborn,
            legend: true,
            grid: true,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum PlotStyle {
    #[default]
    Seaborn,
    Bmh,
    DarkBackground,
    Ggplot,
    Fivethirtyeight,
}

/// Plot types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlotType {
    Line {
        x: Vec<f64>,
        y: Vec<f64>,
        label: Option<String>,
    },
    Scatter {
        x: Vec<f64>,
        y: Vec<f64>,
        labels: Option<Vec<i64>>,
    },
    Histogram {
        data: Vec<f64>,
        bins: usize,
    },
    Bar {
        categories: Vec<String>,
        values: Vec<f64>,
    },
    Heatmap {
        data: Vec<Vec<f64>>,
        xticklabels: Vec<String>,
        yticklabels: Vec<String>,
    },
    ConfusionMatrix {
        matrix: Vec<Vec<u64>>,
        labels: Vec<String>,
    },
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_numpy_array_1d() {
        let arr = NumpyArray::new_1d(vec![1.0, 2.0, 3.0]);
        assert_eq!(arr.shape, vec![3]);
        assert_eq!(arr.len(), 3);
    }
    
    #[test]
    fn test_numpy_array_2d() {
        let arr = NumpyArray::new_2d(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        assert_eq!(arr.shape, vec![2, 2]);
        assert_eq!(arr.len(), 4);
    }
    
    #[test]
    fn test_numpy_reshape() {
        let mut arr = NumpyArray::new_1d(vec![1.0, 2.0, 3.0, 4.0]);
        arr.reshape(vec![2, 2]);
        assert_eq!(arr.shape, vec![2, 2]);
    }
    
    #[test]
    fn test_dataframe() {
        let mut df = DataFrame::new(vec!["a".into(), "b".into()]);
        df.add_row(vec![serde_json::json!(1), serde_json::json!(2)]);
        df.add_row(vec![serde_json::json!(3), serde_json::json!(4)]);
        
        assert_eq!(df.shape(), (2, 2));
    }
    
    #[test]
    fn test_dataframe_filter() {
        let mut df = DataFrame::new(vec!["value".into()]);
        df.add_row(vec![serde_json::json!(1)]);
        df.add_row(vec![serde_json::json!(5)]);
        df.add_row(vec![serde_json::json!(3)]);
        
        let filtered = df.filter("value", |v| v.as_i64().unwrap_or(0) > 2);
        assert_eq!(filtered.shape(), (2, 1));
    }
    
    #[test]
    fn test_sklearn_model_type() {
        let model_type = SklearnModelType::RandomForestClassifier;
        assert!(model_type.is_classifier());
        assert_eq!(model_type.module_path(), ("sklearn.ensemble", "RandomForestClassifier"));
    }
    
    #[test]
    fn test_torch_tensor() {
        let tensor = TorchTensor::zeros(vec![3, 4]);
        assert_eq!(tensor.shape, vec![3, 4]);
        assert_eq!(tensor.data.len(), 12);
        assert!(tensor.data.iter().all(|&x| x == 0.0));
    }
    
    #[test]
    fn test_device_display() {
        assert_eq!(format!("{}", Device::Cpu), "cpu");
        assert_eq!(format!("{}", Device::Cuda(0)), "cuda:0");
    }
    
    #[test]
    fn test_model_metrics() {
        let metrics = ModelMetrics {
            accuracy: 0.95,
            precision: 0.94,
            recall: 0.96,
            f1: 0.95,
            auc_roc: Some(0.98),
            confusion_matrix: None,
        };
        
        assert!((metrics.accuracy - 0.95).abs() < 0.001);
        assert!((metrics.f1 - 0.95).abs() < 0.001);
    }
    
    #[test]
    fn test_training_config() {
        let config = TrainingConfig::default();
        assert_eq!(config.test_size, 0.2);
        assert_eq!(config.cross_validation_folds, 5);
    }
    
    #[test]
    fn test_plot_config() {
        let config = PlotConfig::default();
        assert_eq!(config.figsize, (10.0, 6.0));
        assert!(config.legend);
        assert!(config.grid);
    }
}
