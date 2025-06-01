use crate::models::{Doctor, AccountType};
use super::AlgorithmConfig;

/// 账号分类器
pub struct AccountClassifier {
    config: AlgorithmConfig,
}

impl AccountClassifier {
    /// 创建新的账号分类器
    pub fn new(config: AlgorithmConfig) -> Self {
        Self { config }
    }

    /// 综合分类算法 - 根据文档规范实现
    /// 
    /// 分类依据：
    /// - 头部账号：粉丝数 >= 50万 或 综合影响力指数 >= 80
    /// - 腰部账号：粉丝数 >= 10万 或 综合影响力指数 >= 60
    /// - 尾部账号：其他情况
    pub fn classify_account(&self, doctor: &Doctor) -> AccountType {
        // 计算综合影响力指数
        let influence_index = self.calculate_influence_index(doctor);
        
        // 根据粉丝数和影响力指数综合判断
        if doctor.fans_count >= self.config.head_account_threshold || influence_index >= 80.0 {
            AccountType::Head
        } else if doctor.fans_count >= self.config.middle_account_threshold || influence_index >= 60.0 {
            AccountType::Middle
        } else {
            AccountType::Tail
        }
    }

    /// 计算影响力指数
    /// 
    /// 公式：影响力指数 = (粉丝数权重 × 粉丝数分数 + 播放量权重 × 播放量分数 + 
    ///                   互动权重 × 互动分数) × 职称系数
    pub fn calculate_influence_index(&self, doctor: &Doctor) -> f64 {
        let follower_score = self.normalize_followers(doctor.fans_count);
        let play_score = self.normalize_play_count(doctor.avg_play_count);
        let interaction_score = self.calculate_interaction_score(doctor);
        let title_coefficient = self.get_title_coefficient(&doctor.title);
        
        let base_score = (
            self.config.influence_weight as f64 * follower_score +
            0.3 * play_score +
            0.1 * interaction_score
        ) * title_coefficient;
        
        (base_score * 100.0).min(100.0)
    }

    /// 标准化粉丝数（0-1 区间）
    fn normalize_followers(&self, followers: i32) -> f64 {
        match followers {
            f if f >= 1_000_000 => 1.0,
            f if f >= 500_000 => 0.9,
            f if f >= 200_000 => 0.8,
            f if f >= 100_000 => 0.7,
            f if f >= 50_000 => 0.6,
            f if f >= 20_000 => 0.5,
            f if f >= 10_000 => 0.4,
            f if f >= 5_000 => 0.3,
            f if f >= 1_000 => 0.2,
            _ => 0.1,
        }
    }

    /// 标准化播放量（0-1 区间）
    fn normalize_play_count(&self, play_count: i32) -> f64 {
        match play_count {
            p if p >= 500_000 => 1.0,
            p if p >= 200_000 => 0.9,
            p if p >= 100_000 => 0.8,
            p if p >= 50_000 => 0.7,
            p if p >= 20_000 => 0.6,
            p if p >= 10_000 => 0.5,
            p if p >= 5_000 => 0.4,
            p if p >= 2_000 => 0.3,
            p if p >= 1_000 => 0.2,
            _ => 0.1,
        }
    }

    /// 计算互动分数
    fn calculate_interaction_score(&self, doctor: &Doctor) -> f64 {
        let likes_7d = doctor.likes_7d as f64;
        let comments_7d = doctor.comments_7d as f64;
        let shares_7d = doctor.shares_7d as f64;
        let followers = doctor.fans_count as f64;
        
        if followers <= 0.0 {
            return 0.0;
        }
        
        // 互动率 = (点赞数 + 评论数*2 + 分享数*3) / 粉丝数
        let interaction_rate = (likes_7d + comments_7d * 2.0 + shares_7d * 3.0) / followers;
        
        // 标准化到0-1区间
        (interaction_rate * 1000.0).min(1.0)
    }

    /// 获取职称系数
    fn get_title_coefficient(&self, title: &str) -> f64 {
        match title {
            "主任医师" | "教授" => 1.2,
            "副主任医师" | "副教授" => 1.1,
            "主治医师" => 1.05,
            "住院医师" => 1.0,
            _ => 1.0,
        }
    }

    /// 批量分类
    pub fn classify_batch(&self, doctors: &[Doctor]) -> Vec<(i32, AccountType)> {
        doctors.iter()
            .map(|doctor| (doctor.id, self.classify_account(doctor)))
            .collect()
    }

    /// 获取账号类型的权重系数
    pub fn get_account_type_weight(&self, account_type: &AccountType) -> f64 {
        match account_type {
            AccountType::Head => 1.2,
            AccountType::Middle => 1.0,
            AccountType::Tail => 0.8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Doctor;

    fn create_test_doctor(fans_count: i32, avg_play_count: i32) -> Doctor {
        Doctor {
            id: 1,
            name: "测试医生".to_string(),
            fans_count,
            avg_play_count,
            title: "主治医师".to_string(),
            likes_7d: 1000,
            comments_7d: 100,
            shares_7d: 50,
            ..Default::default()
        }
    }

    #[test]
    fn test_account_classification() {
        let config = AlgorithmConfig::default();
        let classifier = AccountClassifier::new(config);
        
        // 头部账号测试
        let head_doctor = create_test_doctor(600_000, 100_000);
        assert_eq!(classifier.classify_account(&head_doctor), AccountType::Head);
        
        // 腰部账号测试
        let middle_doctor = create_test_doctor(150_000, 30_000);
        assert_eq!(classifier.classify_account(&middle_doctor), AccountType::Middle);
        
        // 尾部账号测试
        let tail_doctor = create_test_doctor(50_000, 5_000);
        assert_eq!(classifier.classify_account(&tail_doctor), AccountType::Tail);
    }

    #[test]
    fn test_influence_index_calculation() {
        let config = AlgorithmConfig::default();
        let classifier = AccountClassifier::new(config);
        
        let doctor = create_test_doctor(200_000, 50_000);
        let influence_index = classifier.calculate_influence_index(&doctor);
        
        assert!(influence_index > 0.0);
        assert!(influence_index <= 100.0);
    }

    #[test]
    fn test_title_coefficient() {
        let config = AlgorithmConfig::default();
        let classifier = AccountClassifier::new(config);
        
        assert_eq!(classifier.get_title_coefficient("主任医师"), 1.2);
        assert_eq!(classifier.get_title_coefficient("主治医师"), 1.05);
        assert_eq!(classifier.get_title_coefficient("住院医师"), 1.0);
    }
}
