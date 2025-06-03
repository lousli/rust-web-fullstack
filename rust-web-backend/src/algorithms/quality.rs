use crate::models::Doctor;

/// 内容质量分析算法
/// 内容质量分析器（保留用于高级内容质量分析功能扩展）
#[allow(dead_code)]
pub struct ContentQualityAnalyzer;

impl ContentQualityAnalyzer {
    /// 计算内容质量指数（预留用于高级内容质量分析）
    #[allow(dead_code)]
    pub fn calculate_content_quality_index(
        performance_score: Option<f64>,
        affinity_score: Option<f64>,
        editing_score: Option<f64>,
        video_quality_score: Option<f64>,
    ) -> f64 {
        // 收集有效评分
        let mut scores = Vec::new();
        let mut weights = Vec::new();
        
        if let Some(score) = performance_score {
            scores.push(score);
            weights.push(0.3); // 表现力权重30%
        }
        
        if let Some(score) = affinity_score {
            scores.push(score);
            weights.push(0.25); // 亲和力权重25%
        }
        
        if let Some(score) = editing_score {
            scores.push(score);
            weights.push(0.25); // 剪辑水平权重25%
        }
        
        if let Some(score) = video_quality_score {
            scores.push(score);
            weights.push(0.2); // 画面质量权重20%
        }
        
        if scores.is_empty() {
            return 0.0;
        }
        
        // 标准化权重
        let total_weight: f64 = weights.iter().sum();
        let normalized_weights: Vec<f64> = weights.iter().map(|w| w / total_weight).collect();
        
        // 加权平均，转换为0-100分制
        let weighted_sum: f64 = scores.iter()
            .zip(normalized_weights.iter())
            .map(|(score, weight)| score * weight)
            .sum();
        
        weighted_sum * 10.0 // 10分制转100分制
    }    /// 分析医生内容质量的详细评分（预留用于高级内容质量分析）
    #[allow(dead_code)]
    pub fn analyze_content_quality(doctor: &Doctor) -> ContentQualityReport {
        let performance_score = doctor.performance_score.unwrap_or(0.0);
        let affinity_score = doctor.affinity_score.unwrap_or(0.0);
        let editing_score = doctor.editing_score.unwrap_or(0.0);
        let video_quality_score = doctor.video_quality_score.unwrap_or(0.0);

        let overall_index = Self::calculate_content_quality_index(
            doctor.performance_score,
            doctor.affinity_score,
            doctor.editing_score,
            doctor.video_quality_score,
        );

        ContentQualityReport {
            performance_score,
            affinity_score,
            editing_score,
            video_quality_score,
            overall_index,
            strengths: Self::identify_strengths(doctor),
            improvements: Self::identify_improvements(doctor),
        }
    }    /// 识别内容质量优势（预留用于内容优势分析）
    #[allow(dead_code)]
    fn identify_strengths(doctor: &Doctor) -> Vec<String> {
        let mut strengths = Vec::new();
        
        if let Some(score) = doctor.performance_score {
            if score >= 8.0 {
                strengths.push("表现力出色".to_string());
            }
        }
        
        if let Some(score) = doctor.affinity_score {
            if score >= 8.0 {
                strengths.push("亲和力强".to_string());
            }
        }
        
        if let Some(score) = doctor.editing_score {
            if score >= 8.0 {
                strengths.push("剪辑水平高".to_string());
            }
        }
        
        if let Some(score) = doctor.video_quality_score {
            if score >= 8.0 {
                strengths.push("画面质量优秀".to_string());
            }
        }
        
        strengths
    }    /// 识别需要改进的方面（预留用于改进建议分析）
    #[allow(dead_code)]
    fn identify_improvements(doctor: &Doctor) -> Vec<String> {
        let mut improvements = Vec::new();
        
        if let Some(score) = doctor.performance_score {
            if score < 6.0 {
                improvements.push("提升表现力".to_string());
            }
        }
        
        if let Some(score) = doctor.affinity_score {
            if score < 6.0 {
                improvements.push("增强亲和力".to_string());
            }
        }
        
        if let Some(score) = doctor.editing_score {
            if score < 6.0 {
                improvements.push("改善剪辑水平".to_string());
            }
        }
        
        if let Some(score) = doctor.video_quality_score {
            if score < 6.0 {
                improvements.push("提高画面质量".to_string());
            }
        }
        
        improvements
    }    /// 根据内容质量给出投放建议（预留用于投放建议功能）
    #[allow(dead_code)]
    pub fn get_investment_advice(quality_index: f64) -> String {
        match quality_index {
            score if score >= 80.0 => "内容质量优秀，强烈推荐投放".to_string(),
            score if score >= 70.0 => "内容质量良好，推荐投放".to_string(),
            score if score >= 60.0 => "内容质量中等，可考虑投放".to_string(),
            score if score >= 50.0 => "内容质量一般，需谨慎考虑".to_string(),
            _ => "内容质量较差，不建议投放".to_string(),
        }
    }
}

/// 内容质量分析报告
/// 内容质量报告（保留用于内容质量分析功能扩展）
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ContentQualityReport {
    pub performance_score: f64,    // 表现力评分
    pub affinity_score: f64,       // 亲和力评分
    pub editing_score: f64,        // 剪辑水平评分
    pub video_quality_score: f64,  // 画面质量评分
    pub overall_index: f64,        // 综合质量指数
    pub strengths: Vec<String>,    // 优势方面
    pub improvements: Vec<String>, // 改进建议
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Doctor;

    fn create_test_doctor() -> Doctor {        Doctor {
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

    #[test]
    fn test_content_quality_index() {
        let index = ContentQualityAnalyzer::calculate_content_quality_index(
            Some(8.0), Some(7.5), Some(8.5), Some(8.0)
        );
        assert!(index >= 70.0 && index <= 100.0);
    }

    #[test]
    fn test_content_quality_analysis() {
        let doctor = create_test_doctor();
        let report = ContentQualityAnalyzer::analyze_content_quality(&doctor);
        
        assert!(report.overall_index > 0.0);
        assert!(!report.strengths.is_empty());
    }

    #[test]
    fn test_investment_advice() {
        let advice = ContentQualityAnalyzer::get_investment_advice(85.0);
        assert!(advice.contains("优秀"));
        
        let advice2 = ContentQualityAnalyzer::get_investment_advice(45.0);
        assert!(advice2.contains("不建议"));
    }
}
