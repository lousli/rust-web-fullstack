/// 算法模块
/// 
/// 实现医生投放评价的各种算法
/// 包括账号分类、性价比计算、数据趋势分析等

pub mod config;
pub mod scoring;
pub mod basic_scoring;
pub mod quality;

pub use scoring::ScoringAlgorithm;
