-- 创建医生表
CREATE TABLE IF NOT EXISTS doctors (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    title TEXT,
    region TEXT,
    department TEXT,
    agency_name TEXT,
    agency_price REAL,
    total_followers INTEGER DEFAULT 0,
    total_likes INTEGER DEFAULT 0,
    total_works INTEGER DEFAULT 0,
    likes_7d INTEGER,
    followers_7d INTEGER,
    shares_7d INTEGER,
    comments_7d INTEGER,
    works_7d INTEGER,
    likes_15d INTEGER,
    followers_15d INTEGER,
    shares_15d INTEGER,
    comments_15d INTEGER,
    works_15d INTEGER,
    likes_30d INTEGER,
    followers_30d INTEGER,
    shares_30d INTEGER,
    comments_30d INTEGER,
    works_30d INTEGER,
    performance_score REAL,
    affinity_score REAL,
    editing_score REAL,
    video_quality_score REAL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 创建权重配置表
CREATE TABLE IF NOT EXISTS weight_configs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    influence_weight REAL NOT NULL DEFAULT 0.3,
    activity_weight REAL NOT NULL DEFAULT 0.25,
    quality_weight REAL NOT NULL DEFAULT 0.25,
    price_weight REAL NOT NULL DEFAULT 0.2,
    is_default INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 创建医生评分表
CREATE TABLE IF NOT EXISTS doctor_scores (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    doctor_id TEXT NOT NULL,
    weight_config_id INTEGER NOT NULL,
    influence_score REAL NOT NULL DEFAULT 0.0,
    quality_score REAL NOT NULL DEFAULT 0.0,
    activity_score REAL NOT NULL DEFAULT 0.0,
    comprehensive_score REAL NOT NULL DEFAULT 0.0,
    cost_performance_index REAL NOT NULL DEFAULT 0.0,
    ranking INTEGER,
    calculated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (doctor_id) REFERENCES doctors(id) ON DELETE CASCADE,
    FOREIGN KEY (weight_config_id) REFERENCES weight_configs(id) ON DELETE CASCADE
);

-- 插入默认权重配置
INSERT OR IGNORE INTO weight_configs (id, name, description, influence_weight, activity_weight, quality_weight, price_weight, is_default)
VALUES (1, '默认权重配置', '系统默认的权重配置', 0.3, 0.25, 0.25, 0.2, 1);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_doctors_region ON doctors(region);
CREATE INDEX IF NOT EXISTS idx_doctors_department ON doctors(department);
CREATE INDEX IF NOT EXISTS idx_doctor_scores_doctor_id ON doctor_scores(doctor_id);
CREATE INDEX IF NOT EXISTS idx_doctor_scores_weight_config_id ON doctor_scores(weight_config_id);
