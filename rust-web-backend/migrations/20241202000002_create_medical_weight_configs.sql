-- 医疗权重配置表创建脚本
-- 为医生投放评价系统创建五大核心评价指标权重配置表

CREATE TABLE IF NOT EXISTS medical_weight_configs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    
    -- 五大核心评价指标权重 (总和为100)
    account_influence_weight REAL NOT NULL DEFAULT 22.0,      -- 账号影响力权重
    cost_effectiveness_weight REAL NOT NULL DEFAULT 35.0,     -- 性价比权重  
    content_quality_weight REAL NOT NULL DEFAULT 28.0,        -- 内容质量权重
    medical_credibility_weight REAL NOT NULL DEFAULT 10.0,    -- 医疗可信度权重
    roi_prediction_weight REAL NOT NULL DEFAULT 5.0,          -- ROI预测权重
    
    -- 配置策略类型
    strategy_type TEXT CHECK(strategy_type IN ('Conservative', 'Aggressive', 'Balanced', 'BrandFocused')),
    
    -- 配置元数据
    is_default BOOLEAN NOT NULL DEFAULT FALSE,
    created_by TEXT DEFAULT 'system',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    -- 约束条件
    CONSTRAINT check_weight_sum CHECK (
        ABS((account_influence_weight + cost_effectiveness_weight + content_quality_weight + 
             medical_credibility_weight + roi_prediction_weight) - 100.0) < 0.01
    ),
    CONSTRAINT check_weight_range CHECK (
        account_influence_weight >= 0 AND account_influence_weight <= 100 AND
        cost_effectiveness_weight >= 0 AND cost_effectiveness_weight <= 100 AND
        content_quality_weight >= 0 AND content_quality_weight <= 100 AND
        medical_credibility_weight >= 0 AND medical_credibility_weight <= 100 AND
        roi_prediction_weight >= 0 AND roi_prediction_weight <= 100
    )
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_medical_weight_configs_is_default ON medical_weight_configs(is_default);
CREATE INDEX IF NOT EXISTS idx_medical_weight_configs_strategy_type ON medical_weight_configs(strategy_type);
CREATE INDEX IF NOT EXISTS idx_medical_weight_configs_created_at ON medical_weight_configs(created_at);

-- 插入默认配置方案
INSERT OR IGNORE INTO medical_weight_configs (
    id, name, description, 
    account_influence_weight, cost_effectiveness_weight, content_quality_weight,
    medical_credibility_weight, roi_prediction_weight,
    strategy_type, is_default, created_by
) VALUES 
(1, '平衡型配置', '各指标权重相对均衡，适合综合考量的投放场景', 
 22.0, 35.0, 28.0, 10.0, 5.0, 'Balanced', TRUE, 'system'),

(2, '保守型配置', '重视性价比和专业可信度，适合新合作医生', 
 20.0, 40.0, 25.0, 12.0, 3.0, 'Conservative', FALSE, 'system'),

(3, '积极型配置', '重视影响力和ROI预测，适合效果导向的投放', 
 30.0, 25.0, 30.0, 8.0, 7.0, 'Aggressive', FALSE, 'system'),

(4, '品牌型配置', '重视内容质量和医疗可信度，适合品牌建设', 
 18.0, 20.0, 35.0, 22.0, 5.0, 'BrandFocused', FALSE, 'system');

-- 创建更新时间触发器
CREATE TRIGGER IF NOT EXISTS update_medical_weight_configs_updated_at
AFTER UPDATE ON medical_weight_configs
FOR EACH ROW
BEGIN
    UPDATE medical_weight_configs SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- 创建唯一默认配置约束触发器
CREATE TRIGGER IF NOT EXISTS ensure_single_default_medical_weight
BEFORE UPDATE ON medical_weight_configs
FOR EACH ROW
WHEN NEW.is_default = 1 AND OLD.is_default = 0
BEGIN
    UPDATE medical_weight_configs SET is_default = 0 WHERE is_default = 1 AND id != NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS ensure_single_default_medical_weight_insert
BEFORE INSERT ON medical_weight_configs
FOR EACH ROW
WHEN NEW.is_default = 1
BEGIN
    UPDATE medical_weight_configs SET is_default = 0 WHERE is_default = 1;
END;
