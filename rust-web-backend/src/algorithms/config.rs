/// 算法配置模块
/// 
/// 定义各种算法的配置参数和权重

/// 算法配置参数
/// 算法配置参数（保留用于高级算法功能扩展）
#[derive(Debug, Clone)]
#[allow(dead_code)]
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

/// 医疗健康领域专用权重配置
/// 医疗权重配置（保留用于医疗专用算法扩展）
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MedicalWeightConfig {
    // 一级权重分配
    pub account_influence_weight: f32,      // 账号影响力 22%
    pub cost_effectiveness_weight: f32,     // 性价比指数 35%
    pub content_quality_weight: f32,        // 内容质量 28%
    pub data_stability_weight: f32,         // 数据稳定性 15%
    
    // 内容质量细分权重
    pub performance_weight: f32,            // 医生表现力 10%
    pub editing_weight: f32,                // 剪辑水平 8%
    pub video_quality_weight: f32,          // 画面质量 7%
    pub professionalism_weight: f32,        // 专业性评分 3%
}

impl Default for MedicalWeightConfig {
    fn default() -> Self {
        Self {
            account_influence_weight: 22.0,
            cost_effectiveness_weight: 35.0,    // 提高，因为采购成本是关键
            content_quality_weight: 28.0,       // 提高，医疗内容质量很重要
            data_stability_weight: 15.0,        // 降低，因为医疗内容不追求爆款
            performance_weight: 10.0,
            editing_weight: 8.0,
            video_quality_weight: 7.0,
            professionalism_weight: 3.0,
        }
    }
}
