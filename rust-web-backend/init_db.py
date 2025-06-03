import sqlite3
import os

# 确保数据目录存在
os.makedirs('data', exist_ok=True)

# 连接到数据库（如果不存在会自动创建）
conn = sqlite3.connect('data/doctors.db')
cursor = conn.cursor()

# 读取并执行SQL脚本
with open('init_db.sql', 'r', encoding='utf-8') as f:
    sql_script = f.read()

cursor.executescript(sql_script)
conn.commit()
conn.close()

print("数据库初始化完成!")
