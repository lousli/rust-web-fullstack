/// 基础评分算法
/// 
/// 实现账号类型评分、性价比评分等基础算法

use crate::models::{Doctor, WeightConfig, ScoreComponents};

/// 计算医生的综合评分
pub fn calculate_comprehensive_score(doctor: &Doctor, weight_config: &WeightConfig) -> ScoreComponents {
    // 账号性质评分
    let account_type_score = calculate_account_type_score(doctor.total_followers);
    
    // 性价比评分
    let cost_performance_score = calculate_cost_performance_score(
        doctor.agency_price.unwrap_or(0.0),
        doctor.total_followers,
        doctor.total_likes,
    );
    
    // 数据趋势评分
    let data_trend_score = calculate_data_trend_score(doctor);
      // 内容质量评分
    let content_quality_score = calculate_content_quality_score(doctor);
    
    // 综合评分
    let comprehensive_score = (account_type_score as f64) * weight_config.influence_weight +
        (cost_performance_score as f64) * weight_config.price_weight +
        (data_trend_score as f64) * weight_config.activity_weight +
        (content_quality_score as f64) * weight_config.quality_weight;
    
    ScoreComponents {
        account_type_score: account_type_score as f64,
        cost_performance_score: cost_performance_score as f64,
        data_trend_score: data_trend_score as f64,
        content_quality_score: content_quality_score as f64,
        comprehensive_score: comprehensive_score as f64,
    }
}

/// 计算账号性质评分
pub fn calculate_account_type_score(followers: i32) -> f32 {
    match followers {
        f if f >= 1_000_000 => 90.0,  // 头部账号
        f if f >= 500_000 => 80.0,
        f if f >= 100_000 => 70.0,    // 腰部账号
        f if f >= 50_000 => 60.0,
        f if f >= 10_000 => 50.0,     // 尾部账号
        _ => 30.0,
    }
}

/// 计算性价比评分
pub fn calculate_cost_performance_score(price: f64, followers: i32, likes: i32) -> f32 {
    if price <= 0.0 {
        return 50.0; // 默认中等分数
    }
    
    // 计算每1000粉丝的价格
    let price_per_1k_followers = (price / (followers as f64 / 1000.0)).max(1.0);
    
    // 根据价格区间评分 (价格越低，性价比越高)
    let price_score = match price_per_1k_followers {
        p if p <= 50.0 => 95.0,
        p if p <= 100.0 => 85.0,
        p if p <= 200.0 => 75.0,
        p if p <= 500.0 => 65.0,
        p if p <= 1000.0 => 55.0,
        _ => 35.0,
    };
    
    // 考虑互动率
    let engagement_rate = if followers > 0 {
        (likes as f64 / followers as f64) * 100.0
    } else {
        0.0
    };
      let engagement_bonus = match engagement_rate {
        r if r >= 10.0 => 10.0_f32,
        r if r >= 5.0 => 5.0_f32,
        r if r >= 2.0 => 2.0_f32,
        _ => 0.0_f32,
    };
    
    (price_score + engagement_bonus).min(100.0_f32)
}

/// 计算数据趋势评分
pub fn calculate_data_trend_score(doctor: &Doctor) -> f32 {
    let mut trend_score: f32 = 50.0; // 基础分数
    
    // 检查粉丝增长趋势
    if let (Some(followers_30d), Some(followers_15d), Some(followers_7d)) = 
        (doctor.followers_30d, doctor.followers_15d, doctor.followers_7d) {
        
        // 计算增长趋势
        let growth_30_15 = followers_15d - (followers_30d - followers_15d);
        let growth_15_7 = followers_7d - (followers_15d - followers_7d);
        
        if growth_15_7 > growth_30_15 {
            trend_score += 20.0; // 增长加速
        } else if growth_15_7 > 0 {
            trend_score += 10.0; // 持续增长
        }
    }
    
    // 检查作品发布频率
    if let Some(works_7d) = doctor.works_7d {
        match works_7d {
            w if w >= 3 => trend_score += 15.0,
            w if w >= 1 => trend_score += 10.0,
            _ => trend_score -= 5.0,
        }
    }
    
    trend_score.min(100.0_f32).max(0.0_f32)
}

/// 计算内容质量评分
pub fn calculate_content_quality_score(doctor: &Doctor) -> f32 {
    let mut quality_score = 50.0; // 基础分数
    
    // 基于人工评分
    if let Some(performance) = doctor.performance_score {
        quality_score += (performance as f32 - 5.0) * 5.0; // 5分为中位数
    }
    
    if let Some(affinity) = doctor.affinity_score {
        quality_score += (affinity as f32 - 5.0) * 3.0;
    }
    
    if let Some(editing) = doctor.editing_score {
        quality_score += (editing as f32 - 5.0) * 3.0;
    }
    
    if let Some(video_quality) = doctor.video_quality_score {
        quality_score += (video_quality as f32 - 5.0) * 4.0;
    }
    
    quality_score.min(100.0).max(0.0)
}
