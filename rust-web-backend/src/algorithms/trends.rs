use crate::models::DoctorMetrics;

/// 数据趋势分析算法
pub struct DataTrendAnalyzer;

impl DataTrendAnalyzer {
    /// 计算数据趋势指数
    pub fn calculate_data_trend_index(
        likes_7d: i64, likes_15d: i64, likes_30d: i64,
        followers_7d: i64, followers_15d: i64, followers_30d: i64,
        works_7d: i64, works_15d: i64, works_30d: i64,
    ) -> f64 {
        // 计算各周期的增长率
        let likes_growth = Self::calculate_growth_trend(likes_7d, likes_15d, likes_30d);
        let followers_growth = Self::calculate_growth_trend(followers_7d, followers_15d, followers_30d);
        let works_efficiency = Self::calculate_content_efficiency(
            works_7d, works_15d, works_30d,
            likes_7d, likes_15d, likes_30d
        );
        
        // 加权综合
        let trend_score = likes_growth * 0.4 + followers_growth * 0.4 + works_efficiency * 0.2;
        
        trend_score.min(100.0).max(0.0)
    }

    /// 计算增长趋势
    fn calculate_growth_trend(val_7d: i64, val_15d: i64, val_30d: i64) -> f64 {
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
        }
    }

    /// 计算内容效率
    fn calculate_content_efficiency(
        works_7d: i64, works_15d: i64, works_30d: i64,
        likes_7d: i64, likes_15d: i64, likes_30d: i64
    ) -> f64 {
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
            (avg_efficiency.ln() * 20.0 + 50.0).min(100.0).max(0.0)
        }
    }    /// 计算增长稳定性指数
    pub fn calculate_growth_stability_index(metrics: &DoctorMetrics) -> f64 {
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
            (100.0 / (1.0 + variance.sqrt())).min(100.0)
        }
    }    /// 综合分析医生数据趋势
    pub fn analyze_doctor_trends(metrics: &DoctorMetrics) -> (f64, f64) {
        let trend_index = Self::calculate_data_trend_index(
            metrics.likes_7d as i64, metrics.likes_15d as i64, metrics.likes_30d as i64,
            metrics.followers_7d as i64, metrics.followers_15d as i64, metrics.followers_30d as i64,
            metrics.works_7d as i64, metrics.works_15d as i64, metrics.works_30d as i64,
        );

        let stability_index = Self::calculate_growth_stability_index(metrics);

        (trend_index, stability_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Doctor;    fn create_test_doctor() -> Doctor {
        Doctor {
            id: "TEST001".to_string(),
            name: "测试医生".to_string(),
            title: Some("主治医师".to_string()),
            region: Some("北京".to_string()),
            department: Some("内科".to_string()),
            agency_name: Some("测试机构".to_string()),
            agency_price: Some(5000.0),
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
        }
    }

    fn create_test_metrics() -> DoctorMetrics {
        DoctorMetrics {
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
        }
    }    #[test]
    fn test_data_trend_calculation() {
        let metrics = create_test_metrics();
        let (trend_index, stability_index) = DataTrendAnalyzer::analyze_doctor_trends(&metrics);
        
        assert!(trend_index >= 0.0 && trend_index <= 100.0);
        assert!(stability_index >= 0.0 && stability_index <= 100.0);
    }

    #[test]
    fn test_growth_trend() {
        let growth = DataTrendAnalyzer::calculate_growth_trend(1000, 2000, 3000);
        assert!(growth >= 0.0 && growth <= 100.0);
    }
}
