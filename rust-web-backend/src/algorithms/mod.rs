/// 算法模块
/// 
/// 实现医生投放评价的各种算法
/// 包括账号分类、性价比计算、数据趋势分析等

pub mod scoring;
pub mod classification;
pub mod trends;
pub mod quality;

pub use scoring::ScoringAlgorithm;
pub use classification::{AccountClassifier, AccountType};
pub use trends::DataTrendAnalyzer;
pub use quality::ContentQualityAnalyzer;

use crate::models::{DoctorMetrics, DoctorScores, WeightConfig, CalculatedIndicators};

/// 算法配置参数
#[derive(Debug, Clone)]
pub struct AlgorithmConfig {
    // 账号分类阈值
    pub head_account_threshold: i32,
    pub middle_account_threshold: i32,
    
    // 性价比计算参数
    pub base_price: f64,
    pub influence_weight: f32,
    pub activity_weight: f32,
    
    // 趋势分析权重
    pub trend_7d_weight: f32,
    pub trend_15d_weight: f32,
    pub trend_30d_weight: f32,
}

impl Default for AlgorithmConfig {
    fn default() -> Self {
        Self {
            head_account_threshold: 500_000,
            middle_account_threshold: 100_000,
            base_price: 5000.0,
            influence_weight: 0.6,
            activity_weight: 0.4,
            trend_7d_weight: 0.5,
            trend_15d_weight: 0.3,
            trend_30d_weight: 0.2,
        }
    }
}

/// 综合评价计算器
pub struct ComprehensiveEvaluator {
    config: AlgorithmConfig,
}

impl ComprehensiveEvaluator {
    pub fn new(config: AlgorithmConfig) -> Self {
        Self { config }
    }

    /// 计算医生的综合指标
    pub fn calculate_indicators(
        &self,
        doctor_id: &str,
        followers: i32,
        total_likes: i32,
        total_works: i32,
        agency_price: Option<f64>,
        metrics: &DoctorMetrics,
        scores: &DoctorScores,
        weights: &WeightConfig,
    ) -> CalculatedIndicators {
        // 账号性质分类和评分
        let account_type = AccountClassifier::classify_account(followers, &self.config);
        let account_type_score = AccountClassifier::calculate_account_score(&account_type, followers, &self.config);
        
        // 性价比指数
        let cost_effectiveness_index = self.calculate_cost_effectiveness(
            agency_price.unwrap_or(0.0),
            followers,
            total_likes,
            &metrics,
        );
        
        // 数据趋势指数
        let data_trend_index = DataTrendAnalyzer::calculate_trend_index(metrics, &self.config);
        let growth_stability_index = DataTrendAnalyzer::calculate_stability_index(metrics, &self.config);
        
        // 内容质量指数
        let content_quality_index = ContentQualityAnalyzer::calculate_quality_index(scores);
        
        // 综合评价指数
        let comprehensive_index = self.calculate_comprehensive_index(
            account_type_score,
            cost_effectiveness_index,
            data_trend_index,
            content_quality_index,
            weights,
        );
        
        CalculatedIndicators {
            id: None,
            doctor_id: doctor_id.to_string(),
            account_type: account_type.as_str().to_string(),
            account_type_score,
            cost_effectiveness_index,
            data_trend_index,
            growth_stability_index,
            content_quality_index,
            comprehensive_index,
            calculated_at: Some(chrono::Utc::now()),
            weight_config_id: weights.id,
        }
    }

    /// 计算性价比指数
    fn calculate_cost_effectiveness(
        &self,
        price: f64,
        followers: i32,
        total_likes: i32,
        metrics: &DoctorMetrics,
    ) -> f32 {
        if price <= 0.0 {
            return 50.0; // 如果没有价格信息，返回中等分数
        }

        // 影响力分数 (基于粉丝量和总获赞量)
        let influence_score = Self::calculate_influence_score(followers, total_likes);
        
        // 活跃度分数 (基于近期数据)
        let activity_score = Self::calculate_activity_score(metrics);
        
        // 综合价值分数
        let value_score = influence_score * self.config.influence_weight 
                         + activity_score * self.config.activity_weight;
        
        // 性价比 = 价值分数 / 标准化价格
        let normalized_price = Self::normalize_price(price, self.config.base_price);
        
        ((value_score / normalized_price) * 100.0).min(100.0).max(0.0)
    }

    /// 计算影响力分数
    fn calculate_influence_score(followers: i32, total_likes: i32) -> f32 {
        let follower_score = (followers as f32).ln() * 10.0;
        let likes_score = (total_likes as f32).ln() * 8.0;
        
        ((follower_score * 0.6 + likes_score * 0.4).min(100.0)).max(0.0)
    }

    /// 计算活跃度分数
    fn calculate_activity_score(metrics: &DoctorMetrics) -> f32 {
        let total_engagement_7d = metrics.likes_7d + metrics.comments_7d + metrics.shares_7d;
        let works_frequency = if metrics.works_7d > 0 { 
            (total_engagement_7d as f32 / metrics.works_7d as f32) 
        } else { 0.0 };
        
        ((works_frequency.ln() * 15.0).min(100.0)).max(0.0)
    }

    /// 标准化价格
    fn normalize_price(price: f64, base_price: f64) -> f32 {
        ((price / base_price) as f32).max(0.1) // 最小值避免除零
    }

    /// 计算综合评价指数
    fn calculate_comprehensive_index(
        &self,
        account_type_score: f32,
        cost_effectiveness_index: f32,
        data_trend_index: f32,
        content_quality_index: f32,
        weights: &WeightConfig,
    ) -> f32 {
        let total_weight = weights.account_type_weight 
                         + weights.cost_effectiveness_weight 
                         + weights.data_trend_weight
                         + weights.performance_weight
                         + weights.affinity_weight
                         + weights.editing_weight
                         + weights.video_quality_weight;
        
        if total_weight == 0.0 {
            return 0.0;
        }
        
        let weighted_score = account_type_score * weights.account_type_weight
                           + cost_effectiveness_index * weights.cost_effectiveness_weight
                           + data_trend_index * weights.data_trend_weight
                           + content_quality_index * (weights.performance_weight 
                                                     + weights.affinity_weight 
                                                     + weights.editing_weight 
                                                     + weights.video_quality_weight);
        
        (weighted_score / total_weight).min(100.0).max(0.0)
    }
}
            affinity_weight: 0.25,
            editing_weight: 0.25,
            video_quality_weight: 0.2,
        }
    }
}
