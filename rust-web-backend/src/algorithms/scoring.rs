/// 综合评分算法实现
/// 
/// 根据文档规范实现医生投放评价的核心算法
/// 包括性价比指数、数据趋势指数、内容质量指数和综合评价指数

use crate::models::{Doctor, WeightConfig, CalculatedIndicators, AccountType};
use super::{AlgorithmConfig, AccountClassifier, DataTrendAnalyzer, ContentQualityAnalyzer};

/// 主要评分算法结构体
pub struct ScoringAlgorithm {
    config: AlgorithmConfig,
    classifier: AccountClassifier,
    trend_analyzer: DataTrendAnalyzer,
    quality_analyzer: ContentQualityAnalyzer,
}

impl ScoringAlgorithm {
    /// 创建新的评分算法实例
    pub fn new(config: AlgorithmConfig) -> Self {
        Self {
            config: config.clone(),
            classifier: AccountClassifier::new(config.clone()),
            trend_analyzer: DataTrendAnalyzer::new(config.clone()),
            quality_analyzer: ContentQualityAnalyzer::new(config.clone()),
        }
    }

    /// 计算完整的医生评价指标
    pub fn calculate_full_indicators(&self, doctor: &Doctor, weight_config: &WeightConfig) -> CalculatedIndicators {
        // 1. 账号分类
        let account_type = self.classifier.classify_account(doctor);
        
        // 2. 计算各项指数
        let cost_performance_index = self.calculate_cost_performance_index(doctor);
        let data_trend_index = self.trend_analyzer.calculate_trend_index(doctor);
        let content_quality_index = self.quality_analyzer.calculate_quality_index(doctor, weight_config);
        
        // 3. 计算综合评价指数
        let comprehensive_score = self.calculate_comprehensive_score(
            cost_performance_index,
            data_trend_index,
            content_quality_index,
            weight_config,
        );

        CalculatedIndicators {
            id: 0, // 数据库插入时会自动分配
            doctor_id: doctor.id,
            account_type,
            cost_performance_index,
            data_trend_index,
            content_quality_index,
            comprehensive_score,
            calculated_at: chrono::Utc::now().naive_utc(),
        }
    }

    /// 计算性价比指数
    /// 
    /// 根据文档公式：性价比指数 = (影响力权重 × 影响力分数 + 活跃度权重 × 活跃度分数) / 价格系数
    pub fn calculate_cost_performance_index(&self, doctor: &Doctor) -> f64 {
        // 影响力分数 = 粉丝数 / 最大粉丝数 * 100（归一化到0-100）
        let max_fans = 10_000_000.0; // 假设最大粉丝数为1000万
        let influence_score = (doctor.fans_count as f64 / max_fans * 100.0).min(100.0);
        
        // 活跃度分数 = 平均播放量 / 最大播放量 * 100
        let max_avg_play = 1_000_000.0; // 假设最大平均播放量为100万
        let activity_score = (doctor.avg_play_count as f64 / max_avg_play * 100.0).min(100.0);
        
        // 价格系数 = 机构报价 / 基准价格
        let price_coefficient = doctor.agency_price / self.config.base_price;
        
        // 性价比指数计算
        let weighted_score = self.config.influence_weight as f64 * influence_score + 
                           self.config.activity_weight as f64 * activity_score;
        
        // 避免除零错误
        if price_coefficient <= 0.0 {
            0.0
        } else {
            (weighted_score / price_coefficient).max(0.0).min(100.0)
        }
    }

    /// 计算综合评价指数
    /// 
    /// 根据权重配置合成最终分数
    fn calculate_comprehensive_score(
        &self,
        cost_performance: f64,
        data_trend: f64,
        content_quality: f64,
        weight_config: &WeightConfig,
    ) -> f64 {
        let total_weight = weight_config.cost_performance_weight + 
                          weight_config.data_trend_weight + 
                          weight_config.content_quality_weight;
        
        if total_weight <= 0.0 {
            return 0.0;
        }

        let weighted_sum = cost_performance * weight_config.cost_performance_weight as f64 +
                          data_trend * weight_config.data_trend_weight as f64 +
                          content_quality * weight_config.content_quality_weight as f64;
        
        (weighted_sum / total_weight as f64).max(0.0).min(100.0)
    }

    /// 批量计算多个医生的指标
    pub fn calculate_batch_indicators(
        &self, 
        doctors: &[Doctor], 
        weight_config: &WeightConfig
    ) -> Vec<CalculatedIndicators> {
        doctors.iter()
            .map(|doctor| self.calculate_full_indicators(doctor, weight_config))
            .collect()
    }

    /// 根据综合评价分数进行排名
    pub fn rank_doctors(&self, indicators: &mut [CalculatedIndicators]) {
        // 按综合评价分数降序排列
        indicators.sort_by(|a, b| b.comprehensive_score.partial_cmp(&a.comprehensive_score)
            .unwrap_or(std::cmp::Ordering::Equal));
    }

    /// 获取推荐的医生列表
    /// 
    /// 根据分数阈值和数量限制返回推荐医生
    pub fn get_recommended_doctors(
        &self,
        indicators: &[CalculatedIndicators],
        min_score: f64,
        limit: usize,
    ) -> Vec<&CalculatedIndicators> {
        indicators.iter()
            .filter(|indicator| indicator.comprehensive_score >= min_score)
            .take(limit)
            .collect()
    }

    /// 生成评价报告摘要
    pub fn generate_evaluation_summary(&self, indicators: &[CalculatedIndicators]) -> EvaluationSummary {
        if indicators.is_empty() {
            return EvaluationSummary::default();
        }

        let total_count = indicators.len();
        
        // 计算各指标的平均值
        let avg_cost_performance = indicators.iter()
            .map(|i| i.cost_performance_index)
            .sum::<f64>() / total_count as f64;
            
        let avg_data_trend = indicators.iter()
            .map(|i| i.data_trend_index)
            .sum::<f64>() / total_count as f64;
            
        let avg_content_quality = indicators.iter()
            .map(|i| i.content_quality_index)
            .sum::<f64>() / total_count as f64;
            
        let avg_comprehensive = indicators.iter()
            .map(|i| i.comprehensive_score)
            .sum::<f64>() / total_count as f64;

        // 统计账号类型分布
        let head_count = indicators.iter()
            .filter(|i| matches!(i.account_type, AccountType::Head))
            .count();
            
        let middle_count = indicators.iter()
            .filter(|i| matches!(i.account_type, AccountType::Middle))
            .count();
            
        let tail_count = indicators.iter()
            .filter(|i| matches!(i.account_type, AccountType::Tail))
            .count();

        // 找出最高分和最低分
        let max_score = indicators.iter()
            .map(|i| i.comprehensive_score)
            .fold(0.0, |a, b| a.max(b));
            
        let min_score = indicators.iter()
            .map(|i| i.comprehensive_score)
            .fold(100.0, |a, b| a.min(b));

        EvaluationSummary {
            total_doctors: total_count,
            avg_cost_performance_index: avg_cost_performance,
            avg_data_trend_index: avg_data_trend,
            avg_content_quality_index: avg_content_quality,
            avg_comprehensive_score: avg_comprehensive,
            head_account_count: head_count,
            middle_account_count: middle_count,
            tail_account_count: tail_count,
            max_comprehensive_score: max_score,
            min_comprehensive_score: min_score,
        }
    }
}

/// 评价摘要结构体
#[derive(Debug, Clone)]
pub struct EvaluationSummary {
    pub total_doctors: usize,
    pub avg_cost_performance_index: f64,
    pub avg_data_trend_index: f64,
    pub avg_content_quality_index: f64,
    pub avg_comprehensive_score: f64,
    pub head_account_count: usize,
    pub middle_account_count: usize,
    pub tail_account_count: usize,
    pub max_comprehensive_score: f64,
    pub min_comprehensive_score: f64,
}

impl Default for EvaluationSummary {
    fn default() -> Self {
        Self {
            total_doctors: 0,
            avg_cost_performance_index: 0.0,
            avg_data_trend_index: 0.0,
            avg_content_quality_index: 0.0,
            avg_comprehensive_score: 0.0,
            head_account_count: 0,
            middle_account_count: 0,
            tail_account_count: 0,
            max_comprehensive_score: 0.0,
            min_comprehensive_score: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::AccountType;

    #[test]
    fn test_cost_performance_calculation() {
        let config = AlgorithmConfig::default();
        let algorithm = ScoringAlgorithm::new(config);
        
        let doctor = Doctor {
            id: 1,
            name: "测试医生".to_string(),
            fans_count: 100000,
            avg_play_count: 50000,
            agency_price: 10000.0,
            ..Default::default()
        };
        
        let score = algorithm.calculate_cost_performance_index(&doctor);
        assert!(score > 0.0);
        assert!(score <= 100.0);
    }

    #[test]
    fn test_comprehensive_score_calculation() {
        let config = AlgorithmConfig::default();
        let algorithm = ScoringAlgorithm::new(config);
        
        let weight_config = WeightConfig {
            id: 1,
            config_name: "测试".to_string(),
            cost_performance_weight: 0.4,
            data_trend_weight: 0.3,
            content_quality_weight: 0.3,
            ..Default::default()
        };
        
        let score = algorithm.calculate_comprehensive_score(80.0, 70.0, 75.0, &weight_config);
        assert!(score > 0.0);
        assert!(score <= 100.0);
    }
}
