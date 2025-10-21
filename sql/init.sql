-- 如果存在minigame数据库，则不创建，反之则创建
-- CREATE DATABASE IF NOT EXISTS minigame;

-- 创建好友关系的映射表，master_id 和 friend_id
CREATE TABLE IF NOT EXISTS friend_mapping (
    master_id INT NOT NULL,
    friend_id INT NOT NULL,
    PRIMARY KEY (master_id, friend_id)
);
-- 外键约束
ALTER TABLE friend_mapping ADD FOREIGN KEY (master_id) REFERENCES player_info(player_id);
ALTER TABLE friend_mapping ADD FOREIGN KEY (friend_id) REFERENCES player_info(player_id);



-- 创建玩家信息表
CREATE TABLE IF NOT EXISTS player_info (
    player_id INT NOT NULL,
    player_name VARCHAR(255) NOT NULL,
    PRIMARY KEY (player_id)
);