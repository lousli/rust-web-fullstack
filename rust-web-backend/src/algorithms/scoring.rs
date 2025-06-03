/// 综合评分算法实现
/// 
/// 根据文档规范实现医生投放评价的核心算法
/// 包括账号影响力指数、性价比指数、内容质量指数、专业可信度指数和ROI预测指数

use crate::models::{Doctor, WeightConfig, CalculatedIndicators, AccountType, DoctorMetrics, DoctorScores, DoctorScore};
use super::config::{AlgorithmConfig, MedicalWeightConfig};
use chrono::Utc;

/// 主要评分算法结构体
/// 评分算法（包含各种评分计算方法，部分方法预留用于功能扩展）
#[allow(dead_code)]
pub struct ScoringAlgorithm {
    config: AlgorithmConfig,
}

impl ScoringAlgorithm {
    /// 创建新的评分算法实例
    pub fn new(config: AlgorithmConfig) -> Self {
        Self {
            config,
        }
    }    /// 账号分类（预留用于账号分类功能扩展）
    #[allow(dead_code)]
    fn classify_account(&self, followers: i32) -> AccountType {
        if followers >= self.config.head_account_threshold {
            AccountType::Head
        } else if followers >= self.config.middle_account_threshold {
            AccountType::Middle
        } else {
            AccountType::Tail
        }
    }    /// 计算账号影响力分数（预留用于影响力计算功能扩展）
    #[allow(dead_code)]
    fn calculate_account_score(&self, account_type: &AccountType, followers: i32, total_likes: i32, total_works: i32) -> f64 {
        // 基础分数
        let base_score = match account_type {
            AccountType::Head => 85.0,
            AccountType::Middle => 60.0,
            AccountType::Tail => 30.0,
        };
        
        // 粉丝量调整
        let follower_bonus = match account_type {
            AccountType::Head => (followers as f64 / 1_000_000.0 * 15.0).min(15.0),
            AccountType::Middle => (followers as f64 / 500_000.0 * 25.0).min(25.0),
            AccountType::Tail => (followers as f64 / 100_000.0 * 30.0).min(30.0),
        };
        
        // 历史互动率调整
        let engagement_rate = if followers > 0 {
            total_likes as f64 / followers as f64
        } else {
            0.0
        };
        let engagement_bonus = engagement_rate * 20.0; // 最高20分奖励
        
        // 内容产出稳定性调整
        let content_stability = if total_works > 100 {
            10.0
        } else if total_works > 50 {
            5.0
        } else {
            0.0
        };
          (base_score + follower_bonus + engagement_bonus.min(20.0) + content_stability).min(100.0)
    }

    /// 计算性价比指数（预留用于性价比分析功能扩展）
    #[allow(dead_code)]
    pub fn calculate_cost_effectiveness(&self,
        price: f64,
        _followers: i32,
        _total_likes: i32,
        likes_7d: i32,
        comments_7d: i32,
        shares_7d: i32,
        works_7d: i32,
    ) -> f64 {
        // 行业均价对比 (假设行业均价6000元)
        let industry_avg_price = 6000.0;
        let price_competitiveness = if price <= industry_avg_price {
            (industry_avg_price - price) / industry_avg_price * 30.0 + 70.0
        } else {
            70.0 - ((price - industry_avg_price) / industry_avg_price * 30.0)
        }.max(0.0).min(100.0);
        
        // 7天单作品效果
        let avg_interaction_per_work = if works_7d > 0 {
            (likes_7d + comments_7d * 3 + shares_7d * 5) as f64 / works_7d as f64
        } else {
            0.0
        };
        
        // 基于单作品效果评分
        let effectiveness_score = if avg_interaction_per_work >= 1000.0 {
            90.0
        } else if avg_interaction_per_work >= 500.0 {
            70.0 + (avg_interaction_per_work - 500.0) / 500.0 * 20.0
        } else {
            (avg_interaction_per_work / 500.0 * 70.0).max(10.0)
        };
        
        // 综合性价比指数
        (price_competitiveness * 0.4 + effectiveness_score * 0.6).min(100.0)    }

    /// 计算专业可信度指数（预留用于专业可信度分析功能扩展）
    #[allow(dead_code)]
    pub fn calculate_medical_credibility(&self,
        title: &str,
        department: &str,
        content_score: f32,
    ) -> f32 {
        // 职称权重
        let title_score = match title {
            "主任医师" | "教授" => 90.0,
            "副主任医师" | "副教授" => 80.0,
            "主治医师" => 70.0,
            "住院医师" => 60.0,
            _ => 50.0,
        };
        
        // 科室热度调整（基于医疗投放经验）
        let department_multiplier = match department {
            "内分泌科" | "心内科" | "消化科" => 1.1,  // 慢病科室，适合长期投放
            "呼吸科" | "神经内科" => 1.05,
            "皮肤科" | "妇科" => 0.95,               // 竞争激烈
            _ => 1.0,
        };
          (title_score * 0.6 + content_score * 10.0 * 0.4) * department_multiplier
    }

    /// 计算ROI预测指数（预留用于ROI预测功能扩展）
    #[allow(dead_code)]
    pub fn calculate_roi_prediction(&self,
        price: f64,
        followers: i32,
        engagement_rate: f32,
        content_quality: f32,
    ) -> f32 {
        // 预期曝光量（基于粉丝量和互动率）
        let expected_exposure = followers as f32 * engagement_rate * 0.3; // 30%的粉丝会看到
        
        // 预期转化（基于内容质量，医疗内容转化率较低）
        let conversion_rate = content_quality / 100.0 * 0.02; // 2%基础转化率
        let expected_conversion = expected_exposure * conversion_rate;
        
        // ROI = 预期转化价值 / 投放成本
        let conversion_value = expected_conversion * 50.0; // 假设每转化50元价值
        let roi = conversion_value / price as f32;
        
        (roi * 100.0).min(100.0_f32) // 标准化到0-100    }

    /// 计算数据趋势指数（预留用于数据趋势分析功能扩展）
    #[allow(dead_code)]
    fn calculate_data_trend_index(&self, metrics: &DoctorMetrics) -> f64 {
        // 计算各周期的增长率
        let likes_growth = self.calculate_growth_trend(metrics.likes_7d, metrics.likes_15d, metrics.likes_30d);
        let followers_growth = self.calculate_growth_trend(metrics.followers_7d, metrics.followers_15d, metrics.followers_30d);
        let works_efficiency = self.calculate_content_efficiency(
            metrics.works_7d, metrics.works_15d, metrics.works_30d,
            metrics.likes_7d, metrics.likes_15d, metrics.likes_30d
        );
        
        // 加权综合
        let trend_score = likes_growth * 0.4 + followers_growth * 0.4 + works_efficiency * 0.2;
        
        trend_score.min(100.0).max(0.0)    }

    /// 计算增长稳定性指数（预留用于增长稳定性分析功能扩展）
    #[allow(dead_code)]
    fn calculate_growth_stability_index(&self, metrics: &DoctorMetrics) -> f64 {
        let periods = vec![
            (metrics.likes_7d as f64, 7.0),
            (metrics.likes_15d as f64, 15.0),
            (metrics.likes_30d as f64, 30.0),
        ];

        // 计算各周期的日均增长率
        let daily_rates: Vec<f64> = periods.iter()
            .map(|(value, days)| value / days)
            .collect();

        if daily_rates.is_empty() {
            return 0.0;
        }

        // 计算均值和方差
        let mean = daily_rates.iter().sum::<f64>() / daily_rates.len() as f64;
        let variance = daily_rates.iter()
            .map(|rate| (rate - mean).powi(2))
            .sum::<f64>() / daily_rates.len() as f64;

        // 稳定性评分：方差越小，稳定性越高
        if variance == 0.0 {
            100.0
        } else {
            (100.0 / (1.0 + variance.sqrt())).min(100.0)        }
    }

    /// 计算医疗健康领域综合评分（预留用于医疗专业评分功能扩展）
    /// 基于医疗健康领域专用权重配置
    #[allow(dead_code)]
    pub fn calculate_medical_comprehensive_score(&self,
        doctor: &Doctor,
        metrics: &DoctorMetrics,
        scores: &DoctorScores,
        medical_config: &MedicalWeightConfig,
    ) -> CalculatedIndicators {
        // 1. 账号分类和影响力评分
        let account_type = self.classify_account(doctor.total_followers);
        let account_influence_score = self.calculate_account_score(
            &account_type,
            doctor.total_followers,
            doctor.total_likes,
            doctor.total_works
        );

        // 2. 性价比指数
        let cost_effectiveness_score = self.calculate_cost_effectiveness(
            doctor.agency_price.unwrap_or(6000.0),
            doctor.total_followers,
            doctor.total_likes,
            metrics.likes_7d,
            metrics.comments_7d,
            metrics.shares_7d,
            metrics.works_7d,
        );

        // 3. 内容质量指数（细分权重）
        let content_quality_score = self.calculate_medical_content_quality(scores, medical_config);        // 4. 专业可信度指数
        let _credibility_score = self.calculate_medical_credibility(
            doctor.title.as_deref().unwrap_or(""),
            doctor.department.as_deref().unwrap_or(""),
            scores.performance_score.unwrap_or(7.0),
        );

        // 5. ROI预测指数
        let engagement_rate = if doctor.total_followers > 0 {
            doctor.total_likes as f32 / doctor.total_followers as f32
        } else {
            0.0
        };
        let roi_prediction_score = self.calculate_roi_prediction(
            doctor.agency_price.unwrap_or(6000.0),
            doctor.total_followers,
            engagement_rate,
            content_quality_score as f32,
        );

        // 6. 数据稳定性指数
        let data_stability_score = self.calculate_growth_stability_index(metrics);

        // 7. 综合评分计算
        let comprehensive_score = (
            account_influence_score * medical_config.account_influence_weight as f64 +
            cost_effectiveness_score * medical_config.cost_effectiveness_weight as f64 +
            content_quality_score * medical_config.content_quality_weight as f64 +
            data_stability_score * medical_config.data_stability_weight as f64
        ) / 100.0; // 权重和为100%

        CalculatedIndicators {
            id: None,
            doctor_id: doctor.id.clone(),
            account_type: match account_type {
                AccountType::Head => "head".to_string(),
                AccountType::Middle => "middle".to_string(),
                AccountType::Tail => "tail".to_string(),
            },
            account_type_score: account_influence_score,
            cost_performance_index: cost_effectiveness_score,
            data_trend_index: roi_prediction_score as f64,
            growth_stability_index: data_stability_score,
            content_quality_index: content_quality_score,
            comprehensive_score,
            calculated_at: Some(Utc::now()),
            weight_config_id: None,
        }    }

    /// 计算医疗内容质量（细分权重）（预留用于医疗内容评分功能扩展）
    #[allow(dead_code)]
    fn calculate_medical_content_quality(&self, scores: &DoctorScores, config: &MedicalWeightConfig) -> f64 {
        let performance = scores.performance_score.unwrap_or(0.0) as f64 * 10.0;
        let editing = scores.editing_score.unwrap_or(0.0) as f64 * 10.0;
        let video_quality = scores.video_quality_score.unwrap_or(0.0) as f64 * 10.0;
        let professionalism = 75.0; // 基于表现力等推算的专业性评分

        // 按照医疗健康领域权重计算
        let total_weight = config.performance_weight + config.editing_weight + 
                          config.video_quality_weight + config.professionalism_weight;
        
        if total_weight <= 0.0 {
            return 0.0;
        }

        (performance * config.performance_weight as f64 +
         editing * config.editing_weight as f64 +
         video_quality * config.video_quality_weight as f64 +
         professionalism * config.professionalism_weight as f64) / total_weight as f64    }

    /// 计算增长趋势（预留用于增长趋势分析功能扩展）
    #[allow(dead_code)]
    fn calculate_growth_trend(&self, val_7d: i32, val_15d: i32, val_30d: i32) -> f64 {
        // 计算日均增长
        let daily_7d = val_7d as f64 / 7.0;
        let daily_15d = val_15d as f64 / 15.0;
        let daily_30d = val_30d as f64 / 30.0;
        
        // 趋势评分：近期表现越好分数越高
        let short_term_weight = 0.5;
        let medium_term_weight = 0.3;
        let long_term_weight = 0.2;
        
        let weighted_daily = daily_7d * short_term_weight + 
                            daily_15d * medium_term_weight + 
                            daily_30d * long_term_weight;
        
        // 标准化到0-100分
        if weighted_daily <= 0.0 {
            0.0
        } else {
            (weighted_daily.ln() * 15.0 + 50.0).min(100.0).max(0.0)
        }    }

    /// 计算内容效率（预留用于内容效率分析功能扩展）
    #[allow(dead_code)]
    fn calculate_content_efficiency(&self, works_7d: i32, works_15d: i32, works_30d: i32, likes_7d: i32, likes_15d: i32, likes_30d: i32) -> f64 {
        if works_7d == 0 || works_15d == 0 || works_30d == 0 {
            return 0.0;
        }
        
        // 计算单条内容的平均获赞数
        let efficiency_7d = likes_7d as f64 / works_7d as f64;
        let efficiency_15d = likes_15d as f64 / works_15d as f64;
        let efficiency_30d = likes_30d as f64 / works_30d as f64;
        
        // 内容效率趋势
        let avg_efficiency = (efficiency_7d + efficiency_15d + efficiency_30d) / 3.0;
        
        if avg_efficiency <= 0.0 {
            0.0
        } else {
            (avg_efficiency.ln() * 20.0 + 50.0).min(100.0).max(0.0)        }
    }

    /// 计算完整的医生评价指标（预留用于完整评价功能扩展）
    #[allow(dead_code)]
    pub fn calculate_full_indicators(&self, doctor: &Doctor, metrics: &DoctorMetrics, scores: &DoctorScores, weight_config: &WeightConfig) -> CalculatedIndicators {
        // 1. 账号分类
        let account_type = self.classify_account(doctor.total_followers);
        let account_type_score = self.calculate_account_score(&account_type, doctor.total_followers, doctor.total_likes, doctor.total_works);
        
        // 2. 计算各项指数
        let cost_performance_index = self.calculate_cost_performance_index(doctor, metrics);
        let data_trend_index = self.calculate_data_trend_index(metrics);
        let growth_stability_index = self.calculate_growth_stability_index(metrics);        let content_quality_index = self.calculate_content_quality_index(scores);
          // 3. 计算综合评价指数
        let comprehensive_score = self.calculate_comprehensive_score(
            account_type_score,
            cost_performance_index,
            data_trend_index,
            content_quality_index,
            weight_config,
        );CalculatedIndicators {
            id: None, // 数据库插入时会自动分配
            doctor_id: doctor.id.clone(),
            account_type: match account_type {
                AccountType::Head => "head".to_string(),
                AccountType::Middle => "middle".to_string(),
                AccountType::Tail => "tail".to_string(),
            },
            account_type_score,
            cost_performance_index,
            data_trend_index,
            growth_stability_index,
            content_quality_index,
            comprehensive_score,
            calculated_at: Some(Utc::now()),
            weight_config_id: weight_config.id.map(|id| id as i32),
        }    }

    /// 计算性价比指数（预留用于性价比计算功能扩展）
    /// 根据文档公式：性价比指数 = (影响力权重 × 影响力分数 + 活跃度权重 × 活跃度分数) / 价格系数
    #[allow(dead_code)]
    pub fn calculate_cost_performance_index(&self, doctor: &Doctor, metrics: &DoctorMetrics) -> f64 {
        // 影响力分数 = 粉丝数 / 最大粉丝数 * 100（归一化到0-100）
        let max_fans = 10_000_000.0; // 假设最大粉丝数为1000万
        let influence_score = ((doctor.total_followers as f64) / max_fans * 100.0).min(100.0);
        
        // 活跃度分数 = 近期互动量 / 最大互动量 * 100
        let max_interaction = 1_000_000.0; // 假设最大互动量为100万
        let recent_interaction = (metrics.likes_7d + metrics.comments_7d + metrics.shares_7d) as f64;
        let activity_score = (recent_interaction / max_interaction * 100.0).min(100.0);
        
        // 价格系数 = 机构报价 / 基准价格
        let base_price = self.config.base_price;
        let price = doctor.agency_price.unwrap_or(base_price);
        let price_coefficient = price / base_price;
        
        // 性价比指数计算
        let weighted_score = self.config.influence_weight as f64 * influence_score + 
                           self.config.activity_weight as f64 * activity_score;
        
        // 避免除零错误
        if price_coefficient <= 0.0 {
            0.0
        } else {
            (weighted_score / price_coefficient).max(0.0).min(100.0)        }
    }

    /// 计算综合评价指数（预留用于综合评价功能扩展）
    /// 根据权重配置合成最终分数
    #[allow(dead_code)]
    fn calculate_comprehensive_score(
        &self,
        account_type_score: f64,
        cost_effectiveness: f64,
        data_trend: f64,
        content_quality: f64,
        weight_config: &WeightConfig,
    ) -> f64 {        // 简化权重映射：将7个旧权重字段映射到4个新权重字段
        // influence_weight = account_type_weight 
        // activity_weight = data_trend_weight
        // quality_weight = performance + affinity + editing + video_quality
        // price_weight = cost_effectiveness_weight
        
        let total_weight = 1.0; // 新的权重总和为1.0
        
        let weighted_sum = account_type_score * weight_config.influence_weight +
                          cost_effectiveness * weight_config.price_weight +
                          data_trend * weight_config.activity_weight +
                          content_quality * weight_config.quality_weight;
        
        (weighted_sum / total_weight).max(0.0).min(100.0)    }

    /// 批量计算多个医生的指标（预留用于批量计算功能扩展）
    #[allow(dead_code)]
    pub fn calculate_batch_indicators(
        &self, 
        doctors: &[Doctor], 
        metrics: &[DoctorMetrics],
        scores: &[DoctorScores],
        weight_config: &WeightConfig
    ) -> Vec<CalculatedIndicators> {
        doctors.iter()
            .zip(metrics.iter())
            .zip(scores.iter())
            .map(|((doctor, metric), score)| self.calculate_full_indicators(doctor, metric, score, weight_config))
            .collect()    }

    /// 根据综合评价分数进行排名（预留用于排名功能扩展）
    #[allow(dead_code)]
    pub fn rank_doctors(&self, indicators: &mut [CalculatedIndicators]) {
        // 按综合评价分数降序排列
        indicators.sort_by(|a, b| b.comprehensive_score.partial_cmp(&a.comprehensive_score)            .unwrap_or(std::cmp::Ordering::Equal));
    }

    /// 获取推荐的医生列表（预留用于推荐功能扩展）
    /// 根据分数阈值和数量限制返回推荐医生
    #[allow(dead_code)]
    pub fn get_recommended_doctors<'a>(
        &self,
        indicators: &'a [CalculatedIndicators],
        min_score: f64,
        limit: usize,
    ) -> Vec<&'a CalculatedIndicators> {
        indicators.iter()
            .filter(|indicator| indicator.comprehensive_score >= min_score)
            .take(limit)
            .collect()    }

    /// 生成评价报告摘要（预留用于报告生成功能扩展）
    #[allow(dead_code)]
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
            .sum::<f64>() / total_count as f64;        // 统计账号类型分布
        let head_count = indicators.iter()
            .filter(|i| i.account_type == "head")
            .count();
            
        let middle_count = indicators.iter()
            .filter(|i| i.account_type == "middle")
            .count();
            
        let tail_count = indicators.iter()
            .filter(|i| i.account_type == "tail")
            .count();

        // 找出最高分和最低分
        let max_score = indicators.iter()
            .map(|i| i.comprehensive_score)
            .fold(0.0f64, |a, b| a.max(b));
            
        let min_score = indicators.iter()
            .map(|i| i.comprehensive_score)
            .fold(100.0f64, |a, b| a.min(b));

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

    /// 计算所有医生的评分
    pub fn calculate_scores(doctors: &[Doctor], weight_config: &WeightConfig) -> Vec<DoctorScore> {
        let mut scores = Vec::new();
        
        for (index, doctor) in doctors.iter().enumerate() {
            let account_type = if doctor.total_followers >= 500_000 {
                "head".to_string()
            } else if doctor.total_followers >= 100_000 {
                "middle".to_string()
            } else {
                "tail".to_string()
            };
            
            // 计算各项评分
            let influence_score = Self::calculate_influence_score_static(doctor);
            let quality_score = Self::calculate_quality_score_static(doctor);
            let activity_score = Self::calculate_activity_score_static(doctor);
              // 计算综合评分
            let comprehensive_score = influence_score * 0.4 +
                quality_score * 0.3 +
                activity_score * 0.3;
            
            // 计算性价比指数
            let cost_performance_index = if let Some(price) = doctor.agency_price {
                if price > 0.0 {
                    (comprehensive_score / (price / 10000.0)).min(100.0)
                } else {
                    50.0
                }
            } else {
                50.0
            };
            
            let doctor_score = DoctorScore {
                id: None,
                doctor_id: doctor.id.clone(),
                doctor_name: Some(doctor.name.clone()),
                department: doctor.department.clone(),
                region: doctor.region.clone(),
                institution: doctor.agency_name.clone(),
                account_type: Some(account_type),
                influence_score,
                quality_score,
                activity_score,
                comprehensive_score,
                cost_performance_index,
                cost_performance_score: Some(cost_performance_index),
                data_index_score: Some(activity_score),
                performance_score: doctor.performance_score.map(|s| s as f64),
                affinity_score: doctor.affinity_score.map(|s| s as f64),
                editing_score: doctor.editing_score.map(|s| s as f64),
                video_quality_score: doctor.video_quality_score.map(|s| s as f64),
                weighted_total_score: comprehensive_score,
                ranking: Some((index + 1) as i64),
                weight_config_id: weight_config.id.map(|id| id as i32).unwrap_or(1),
                calculated_at: Some(Utc::now()),
            };
            
            scores.push(doctor_score);
        }
        
        // 按综合评分排序
        scores.sort_by(|a, b| b.comprehensive_score.partial_cmp(&a.comprehensive_score).unwrap());
        
        // 更新排名
        for (index, score) in scores.iter_mut().enumerate() {
            score.ranking = Some((index + 1) as i64);
        }
        
        scores    }

    /// 获取投资建议（预留用于投资建议功能扩展）
    #[allow(dead_code)]
    pub fn get_investment_recommendations(scores: &[DoctorScore], doctors: &[Doctor], limit: usize) -> Vec<serde_json::Value> {
        let mut recommendations = Vec::new();
        
        // 取前N个高性价比医生
        let top_scores: Vec<_> = scores.iter()
            .filter(|s| s.cost_performance_index >= 70.0)
            .take(limit)
            .collect();
            
        for score in top_scores {
            if let Some(doctor) = doctors.iter().find(|d| d.id == score.doctor_id) {
                let mut recommendation = serde_json::Map::new();
                
                recommendation.insert("doctor_id".to_string(), serde_json::Value::String(doctor.id.clone()));
                recommendation.insert("name".to_string(), serde_json::Value::String(doctor.name.clone()));
                recommendation.insert("comprehensive_score".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(score.comprehensive_score).unwrap()));
                recommendation.insert("cost_performance_index".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(score.cost_performance_index).unwrap()));
                recommendation.insert("reason".to_string(), serde_json::Value::String(
                    format!("综合评分{:.1}分，性价比指数{:.1}分，建议优先投放", score.comprehensive_score, score.cost_performance_index)
                ));
                
                recommendations.push(serde_json::Value::Object(recommendation));
            }
        }
        
        recommendations    }
    
    /// 静态方法：计算影响力评分（预留用于影响力计算功能扩展）
    #[allow(dead_code)]
    fn calculate_influence_score_static(doctor: &Doctor) -> f64 {
        let follower_score = (doctor.total_followers as f64).ln() * 8.0;
        let likes_score = (doctor.total_likes as f64).ln() * 6.0;
        
        ((follower_score * 0.6 + likes_score * 0.4) / 2.0).min(100.0).max(0.0)    }
    
    /// 静态方法：计算质量评分（预留用于质量评分功能扩展）
    #[allow(dead_code)]
    fn calculate_quality_score_static(doctor: &Doctor) -> f64 {
        let mut quality_score = 50.0;
        
        if let Some(performance) = doctor.performance_score {
            quality_score += (performance as f64 - 5.0) * 5.0;
        }
        
        if let Some(affinity) = doctor.affinity_score {
            quality_score += (affinity as f64 - 5.0) * 3.0;
        }
        
        if let Some(editing) = doctor.editing_score {
            quality_score += (editing as f64 - 5.0) * 3.0;
        }
        
        if let Some(video_quality) = doctor.video_quality_score {
            quality_score += (video_quality as f64 - 5.0) * 4.0;
        }
        
        quality_score.min(100.0).max(0.0)    }
    
    /// 静态方法：计算活跃度评分（预留用于活跃度评分功能扩展）
    #[allow(dead_code)]
    fn calculate_activity_score_static(doctor: &Doctor) -> f64 {
        let mut activity_score = 50.0;
        
        // 基于7天数据计算活跃度
        if let Some(works_7d) = doctor.works_7d {
            activity_score += (works_7d as f64 * 5.0).min(25.0);
        }
        
        if let Some(likes_7d) = doctor.likes_7d {
            activity_score += (likes_7d as f64 / 1000.0).min(15.0);
        }
        
        if let Some(followers_7d) = doctor.followers_7d {
            activity_score += (followers_7d as f64 / 100.0).min(10.0);
        }
        
        activity_score.min(100.0).max(0.0)    }

    /// 计算内容质量指数（预留用于内容质量计算功能扩展）
    #[allow(dead_code)]
    pub fn calculate_content_quality_index(&self, scores: &DoctorScores) -> f64 {
        crate::algorithms::quality::ContentQualityAnalyzer::calculate_content_quality_index(
            scores.performance_score.map(|s| s as f64),
            scores.affinity_score.map(|s| s as f64), 
            scores.editing_score.map(|s| s as f64),
            scores.video_quality_score.map(|s| s as f64),
        )
    }
}

/// 评价摘要结构体（保留用于复杂评估分析功能扩展）
#[derive(Debug, Clone)]
#[allow(dead_code)]
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

    #[test]
    fn test_cost_performance_calculation() {
        let config = AlgorithmConfig::default();
        let algorithm = ScoringAlgorithm::new(config);
        
        let doctor = Doctor {
            id: "TEST001".to_string(),
            name: "测试医生".to_string(),
            title: Some("主治医师".to_string()),
            region: Some("北京".to_string()),
            department: Some("内科".to_string()),
            agency_name: Some("测试机构".to_string()),
            agency_price: Some(10000.0),
            total_followers: 100000,
            total_likes: 50000,
            total_works: 100,
            likes_7d: Some(1000),
            followers_7d: Some(100),
            shares_7d: Some(50),
            comments_7d: Some(200),
            works_7d: Some(5),
            likes_15d: Some(2000),
            followers_15d: Some(200),
            shares_15d: Some(100),
            comments_15d: Some(400),
            works_15d: Some(10),
            likes_30d: Some(3000),
            followers_30d: Some(300),
            shares_30d: Some(150),
            comments_30d: Some(600),
            works_30d: Some(15),
            performance_score: Some(8.0),
            affinity_score: Some(7.5),
            editing_score: Some(8.5),
            video_quality_score: Some(8.0),
            created_at: None,
            updated_at: None,
        };
        
        let metrics = DoctorMetrics {
            id: None,
            doctor_id: "TEST001".to_string(),
            likes_7d: 1000,
            followers_7d: 100,
            shares_7d: 50,
            comments_7d: 200,
            works_7d: 5,
            likes_15d: 2000,
            followers_15d: 200,
            shares_15d: 100,
            comments_15d: 400,
            works_15d: 10,
            likes_30d: 3000,
            followers_30d: 300,
            shares_30d: 150,
            comments_30d: 600,
            works_30d: 15,
            recorded_at: None,
        };
        
        let score = algorithm.calculate_cost_performance_index(&doctor, &metrics);
        assert!(score > 0.0);
        assert!(score <= 100.0);
    }

    #[test]
    fn test_comprehensive_score_calculation() {
        let config = AlgorithmConfig::default();
        let algorithm = ScoringAlgorithm::new(config);
          let weight_config = WeightConfig {
            id: Some(1),
            name: "测试".to_string(),
            description: Some("测试权重配置".to_string()),
            influence_weight: 0.2,
            activity_weight: 0.3,
            quality_weight: 0.1,
            price_weight: 0.4,
            is_default: Some(1),
            created_at: None,
            updated_at: None,
        };
        
        let score = algorithm.calculate_comprehensive_score(80.0, 70.0, 75.0, 85.0, &weight_config);
        assert!(score > 0.0);
        assert!(score <= 100.0);
    }
}
