-- 创建医生表和其他必需的表结构
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
    likes_7d INTEGER DEFAULT 0,
    followers_7d INTEGER DEFAULT 0,
    shares_7d INTEGER DEFAULT 0,
    comments_7d INTEGER DEFAULT 0,
    works_7d INTEGER DEFAULT 0,
    likes_15d INTEGER DEFAULT 0,
    followers_15d INTEGER DEFAULT 0,
    shares_15d INTEGER DEFAULT 0,
    comments_15d INTEGER DEFAULT 0,
    works_15d INTEGER DEFAULT 0,
    likes_30d INTEGER DEFAULT 0,
    followers_30d INTEGER DEFAULT 0,
    shares_30d INTEGER DEFAULT 0,
    comments_30d INTEGER DEFAULT 0,
    works_30d INTEGER DEFAULT 0,
    performance_score REAL DEFAULT 0,
    affinity_score REAL DEFAULT 0,
    editing_score REAL DEFAULT 0,
    video_quality_score REAL DEFAULT 0,
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

-- 插入默认权重配置
INSERT OR IGNORE INTO weight_configs (id, name, description, influence_weight, activity_weight, quality_weight, price_weight, is_default)
VALUES (1, '默认权重配置', '系统默认的权重配置', 0.3, 0.25, 0.25, 0.2, 1);
