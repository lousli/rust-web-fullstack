use sqlx::{SqlitePool, migrate::MigrateDatabase, Sqlite};
use anyhow::Result;
use crate::config;

pub async fn setup_database() -> Result<SqlitePool> {
    let config = config::get_config();
    let database_url = &config.database.url;
    
    // 如果数据库不存在，创建它
    if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
        println!("正在创建数据库: {}", database_url);
        Sqlite::create_database(database_url).await?;
    }

    let pool = SqlitePool::connect(database_url).await?;
    
    // 创建表结构
    create_tables(&pool).await?;
    
    // 插入默认权重配置
    insert_default_weights(&pool).await?;
    
    Ok(pool)
}

async fn create_tables(pool: &SqlitePool) -> Result<()> {
    // 创建医生信息表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS doctors (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            title TEXT NOT NULL,
            region TEXT NOT NULL,
            department TEXT NOT NULL,
            agency_name TEXT,
            agency_price REAL NOT NULL,
            total_followers INTEGER NOT NULL,
            total_likes INTEGER NOT NULL,
            total_works INTEGER NOT NULL,
            
            -- 7天数据
            likes_7d INTEGER DEFAULT 0,
            followers_7d INTEGER DEFAULT 0,
            shares_7d INTEGER DEFAULT 0,
            comments_7d INTEGER DEFAULT 0,
            works_7d INTEGER DEFAULT 0,
            
            -- 15天数据
            likes_15d INTEGER DEFAULT 0,
            followers_15d INTEGER DEFAULT 0,
            shares_15d INTEGER DEFAULT 0,
            comments_15d INTEGER DEFAULT 0,
            works_15d INTEGER DEFAULT 0,
            
            -- 30天数据
            likes_30d INTEGER DEFAULT 0,
            followers_30d INTEGER DEFAULT 0,
            shares_30d INTEGER DEFAULT 0,
            comments_30d INTEGER DEFAULT 0,
            works_30d INTEGER DEFAULT 0,
            
            -- 人工评分 (0-10分)
            performance_score REAL,
            affinity_score REAL,
            editing_score REAL,
            video_quality_score REAL,
            
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 创建权重配置表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS weight_configs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            
            -- 一级权重 (总和必须为100)
            account_type_weight REAL NOT NULL DEFAULT 25.0,
            cost_effectiveness_weight REAL NOT NULL DEFAULT 30.0,
            data_trend_weight REAL NOT NULL DEFAULT 25.0,
            
            -- 内容质量细分权重
            performance_weight REAL NOT NULL DEFAULT 6.0,
            affinity_weight REAL NOT NULL DEFAULT 5.0,
            editing_weight REAL NOT NULL DEFAULT 5.0,
            video_quality_weight REAL NOT NULL DEFAULT 4.0,
            
            is_default BOOLEAN DEFAULT FALSE,
            created_by TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 创建计算指标表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS calculated_indicators (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            doctor_id TEXT NOT NULL,
            
            -- 账号性质分类
            account_type TEXT NOT NULL,
            account_type_score REAL NOT NULL,
            
            -- 性价比指数
            cost_effectiveness_index REAL NOT NULL,
            
            -- 数据指数
            data_trend_index REAL NOT NULL,
            growth_stability_index REAL NOT NULL,
            
            -- 内容质量指数
            content_quality_index REAL NOT NULL,
            
            -- 综合指数
            comprehensive_index REAL NOT NULL,
            
            -- 计算时间和版本
            calculated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            weight_config_id INTEGER NOT NULL,
            
            FOREIGN KEY (doctor_id) REFERENCES doctors(id),
            FOREIGN KEY (weight_config_id) REFERENCES weight_configs(id),
            UNIQUE(doctor_id, weight_config_id, calculated_at)
        )
        "#,
    )
    .execute(pool)
    .await?;

        // 创建系统配置表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS system_configs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            config_key TEXT UNIQUE NOT NULL,
            config_value TEXT,
            description TEXT,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,    )
    .execute(pool)
    .await?;

    // 创建医疗权重配置表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS medical_weight_configs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            
            -- 五大核心评价指标权重 (总和为100)
            account_influence_weight REAL NOT NULL DEFAULT 22.0,
            cost_effectiveness_weight REAL NOT NULL DEFAULT 35.0,
            content_quality_weight REAL NOT NULL DEFAULT 28.0,
            medical_credibility_weight REAL NOT NULL DEFAULT 10.0,
            roi_prediction_weight REAL NOT NULL DEFAULT 5.0,
            
            -- 配置策略类型
            strategy_type TEXT CHECK(strategy_type IN ('Conservative', 'Aggressive', 'Balanced', 'BrandFocused')),
              -- 配置元数据
            is_default BOOLEAN NOT NULL DEFAULT FALSE,
            created_by TEXT DEFAULT 'system',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 创建医疗计算指标表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS medical_calculated_indicators (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            doctor_id TEXT NOT NULL,
            weight_config_id INTEGER NOT NULL,
            
            -- 五大核心评价指标评分
            account_influence_score REAL NOT NULL,     -- 账号影响力评分
            cost_effectiveness_score REAL NOT NULL,    -- 性价比评分
            content_quality_score REAL NOT NULL,       -- 内容质量评分
            medical_credibility_score REAL NOT NULL,   -- 医疗可信度评分
            roi_prediction_score REAL NOT NULL,        -- ROI预测评分
            
            -- 综合评分
            comprehensive_score REAL NOT NULL,
            
            -- 计算时间
            calculated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            
            FOREIGN KEY (doctor_id) REFERENCES doctors(id),
            FOREIGN KEY (weight_config_id) REFERENCES medical_weight_configs(id),
            UNIQUE(doctor_id, weight_config_id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    println!("数据库表创建成功");
    Ok(())
}

async fn insert_default_weights(pool: &SqlitePool) -> Result<()> {
    // 检查是否已有默认配置
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM weight_configs WHERE is_default = 1")
        .fetch_one(pool)
        .await?;

    if count.0 == 0 {
        // 插入默认权重配置
        sqlx::query(
            r#"
            INSERT INTO weight_configs (
                name, description, 
                account_type_weight, cost_effectiveness_weight, data_trend_weight,
                performance_weight, affinity_weight, editing_weight, video_quality_weight,
                is_default, created_by
            ) VALUES (
                '默认配置', '系统默认的权重配置', 
                25.0, 30.0, 25.0,
                6.0, 5.0, 5.0, 4.0,
                1, 'system'
            )
            "#,
        )
        .execute(pool)
        .await?;

        // 插入成本敏感型配置
        sqlx::query(
            r#"
            INSERT INTO weight_configs (
                name, description,
                account_type_weight, cost_effectiveness_weight, data_trend_weight,
                performance_weight, affinity_weight, editing_weight, video_quality_weight,
                is_default, created_by
            ) VALUES (
                '成本敏感型', '重视性价比的投放配置',
                15.0, 45.0, 25.0,
                5.0, 4.0, 3.0, 3.0,
                0, 'system'
            )
            "#,
        )
        .execute(pool)
        .await?;

        // 插入影响力优先配置
        sqlx::query(
            r#"
            INSERT INTO weight_configs (
                name, description,
                account_type_weight, cost_effectiveness_weight, data_trend_weight,
                performance_weight, affinity_weight, editing_weight, video_quality_weight,
                is_default, created_by
            ) VALUES (
                '影响力优先型', '重视影响力的投放配置',
                40.0, 20.0, 25.0,
                5.0, 4.0, 3.0, 3.0,
                0, 'system'
            )
            "#,
        )
        .execute(pool)
        .await?;

        // 插入内容质量型配置
        sqlx::query(
            r#"
            INSERT INTO weight_configs (
                name, description,
                account_type_weight, cost_effectiveness_weight, data_trend_weight,
                performance_weight, affinity_weight, editing_weight, video_quality_weight,
                is_default, created_by
            ) VALUES (
                '内容质量型', '重视内容质量的投放配置',
                20.0, 25.0, 20.0,
                10.0, 8.0, 9.0, 8.0,
                0, 'system'
            )
            "#,
        )
        .execute(pool)
        .await?;

        // 插入系统配置数据
        let system_configs = vec![
            ("account_head_threshold", "500000", "头部账号粉丝量阈值"),
            ("account_middle_threshold", "100000", "腰部账号粉丝量阈值"),
            ("cost_base_price", "5000", "性价比计算基准价格"),
            ("trend_weight_7d", "0.5", "7天数据权重"),
            ("trend_weight_15d", "0.3", "15天数据权重"),
            ("trend_weight_30d", "0.2", "30天数据权重"),
        ];
        
        for (key, value, desc) in system_configs {
            sqlx::query(
                "INSERT OR IGNORE INTO system_configs (config_key, config_value, description) VALUES (?, ?, ?)"
            )
            .bind(key)
            .bind(value)
            .bind(desc)
            .execute(pool)
            .await?;
        }
        
        println!("默认权重配置创建成功");
    }

    // 初始化医疗权重配置
    insert_default_medical_weights(pool).await?;

    Ok(())
}

async fn insert_default_medical_weights(pool: &SqlitePool) -> Result<()> {
    // 检查是否已有医疗权重配置
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM medical_weight_configs")
        .fetch_one(pool)
        .await?;

    if count.0 == 0 {
        // 插入默认医疗权重配置方案
        let medical_configs = vec![
            (
                "平衡型配置", "各指标权重相对均衡，适合综合考量的投放场景",
                22.0, 35.0, 28.0, 10.0, 5.0, "Balanced", true
            ),
            (
                "保守型配置", "重视性价比和专业可信度，适合新合作医生",
                20.0, 40.0, 25.0, 12.0, 3.0, "Conservative", false
            ),
            (
                "积极型配置", "重视影响力和ROI预测，适合效果导向的投放",
                30.0, 25.0, 30.0, 8.0, 7.0, "Aggressive", false
            ),
            (
                "品牌型配置", "重视内容质量和医疗可信度，适合品牌建设",
                18.0, 20.0, 35.0, 22.0, 5.0, "BrandFocused", false
            ),
        ];

        for (name, desc, account_influence, cost_effectiveness, content_quality, 
             medical_credibility, roi_prediction, strategy, is_default) in medical_configs {
            sqlx::query(
                r#"
                INSERT INTO medical_weight_configs (
                    name, description,
                    account_influence_weight, cost_effectiveness_weight, content_quality_weight,
                    medical_credibility_weight, roi_prediction_weight,
                    strategy_type, is_default, created_by
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 'system')
                "#,
            )
            .bind(name)
            .bind(desc)
            .bind(account_influence)
            .bind(cost_effectiveness)
            .bind(content_quality)
            .bind(medical_credibility)
            .bind(roi_prediction)
            .bind(strategy)
            .bind(is_default)
            .execute(pool)
            .await?;
        }        println!("默认医疗权重配置创建成功");
    }

    Ok(())
}
