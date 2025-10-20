-- 如果存在minigame数据库，则不创建，反之则创建
-- CREATE DATABASE IF NOT EXISTS minigame;

-- 创建好友关系的映射表，master_id 和 friend_id
CREATE TABLE IF NOT EXISTS friend_mapping (
    master_id INT NOT NULL,
    friend_id INT NOT NULL,
    PRIMARY KEY (master_id, friend_id)
);
