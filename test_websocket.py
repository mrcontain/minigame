#!/usr/bin/env python3
"""
WebSocket 测试脚本
用于测试 minigame 的 WebSocket 接口

使用方法:
    python3 test_websocket.py --player-id 1 --room-id 1

依赖安装:
    pip3 install websocket-client
"""

import asyncio
import json
import sys
import argparse
from datetime import datetime

try:
    import websocket
except ImportError:
    print("错误: 需要安装 websocket-client 库")
    print("请运行: pip3 install websocket-client")
    sys.exit(1)


class WebSocketClient:
    def __init__(self, url, player_id, room_id):
        self.url = f"{url}?player_id={player_id}&room_id={room_id}"
        self.player_id = player_id
        self.room_id = room_id
        self.ws = None
        
    def timestamp(self):
        return datetime.now().strftime("%H:%M:%S")
    
    def on_message(self, ws, message):
        print(f"[{self.timestamp()}] 📨 收到消息: {message}")
        try:
            data = json.loads(message)
            print(f"[{self.timestamp()}] 📦 解析数据: {json.dumps(data, indent=2, ensure_ascii=False)}")
        except json.JSONDecodeError:
            print(f"[{self.timestamp()}] ⚠️  非 JSON 格式消息")
    
    def on_error(self, ws, error):
        print(f"[{self.timestamp()}] ❌ 错误: {error}")
    
    def on_close(self, ws, close_status_code, close_msg):
        print(f"[{self.timestamp()}] 🔌 连接关闭")
        print(f"    状态码: {close_status_code}")
        print(f"    消息: {close_msg}")
    
    def on_open(self, ws):
        print(f"[{self.timestamp()}] ✅ 连接已建立")
        print(f"    URL: {self.url}")
        print(f"    玩家 ID: {self.player_id}")
        print(f"    房间 ID: {self.room_id}")
        print()
        print("💡 提示:")
        print("  - 输入消息内容并按回车发送")
        print("  - 输入 'quit' 或 'exit' 退出")
        print("  - 输入 'help' 查看帮助")
        print("-" * 50)
        
        # 启动输入线程
        import threading
        input_thread = threading.Thread(target=self.input_handler)
        input_thread.daemon = True
        input_thread.start()
    
    def input_handler(self):
        """处理用户输入"""
        while True:
            try:
                content = input()
                
                if content.lower() in ['quit', 'exit']:
                    print(f"[{self.timestamp()}] 👋 正在退出...")
                    self.ws.close()
                    break
                elif content.lower() == 'help':
                    print()
                    print("📖 帮助信息:")
                    print("  quit/exit - 退出程序")
                    print("  help      - 显示此帮助")
                    print("  其他内容  - 发送消息")
                    print()
                    continue
                elif not content.strip():
                    continue
                
                message = {
                    "player_id": self.player_id,
                    "content": content
                }
                
                self.ws.send(json.dumps(message))
                print(f"[{self.timestamp()}] 📤 已发送: {content}")
                
            except KeyboardInterrupt:
                print(f"\n[{self.timestamp()}] 👋 收到中断信号，正在退出...")
                self.ws.close()
                break
            except Exception as e:
                print(f"[{self.timestamp()}] ❌ 输入错误: {e}")
    
    def connect(self):
        """连接到 WebSocket 服务器"""
        print(f"[{self.timestamp()}] 🔌 正在连接到: {self.url}")
        
        websocket.enableTrace(False)
        self.ws = websocket.WebSocketApp(
            self.url,
            on_open=self.on_open,
            on_message=self.on_message,
            on_error=self.on_error,
            on_close=self.on_close
        )
        
        self.ws.run_forever()


def test_connection_errors():
    """测试各种错误情况"""
    print("=" * 50)
    print("测试连接错误情况")
    print("=" * 50)
    
    test_cases = [
        {
            "name": "缺少 player_id 参数",
            "url": "ws://localhost:7777/ws?room_id=1",
        },
        {
            "name": "缺少 room_id 参数",
            "url": "ws://localhost:7777/ws?player_id=1",
        },
        {
            "name": "player_id 格式错误",
            "url": "ws://localhost:7777/ws?player_id=abc&room_id=1",
        },
        {
            "name": "room_id 格式错误",
            "url": "ws://localhost:7777/ws?player_id=1&room_id=xyz",
        },
    ]
    
    for test in test_cases:
        print(f"\n测试: {test['name']}")
        print(f"URL: {test['url']}")
        try:
            ws = websocket.create_connection(test['url'], timeout=2)
            print("❌ 应该失败但成功了")
            ws.close()
        except Exception as e:
            print(f"✅ 预期失败: {e}")
    
    print("\n" + "=" * 50)


def main():
    parser = argparse.ArgumentParser(description='WebSocket 测试客户端')
    parser.add_argument('--url', default='ws://localhost:7777/ws',
                        help='WebSocket 服务器地址 (默认: ws://localhost:7777/ws)')
    parser.add_argument('--player-id', type=int, required=True,
                        help='玩家 ID')
    parser.add_argument('--room-id', type=int, required=True,
                        help='房间 ID')
    parser.add_argument('--test-errors', action='store_true',
                        help='测试错误情况')
    
    args = parser.parse_args()
    
    if args.test_errors:
        test_connection_errors()
        return
    
    print("=" * 50)
    print("🎮 WebSocket 测试客户端")
    print("=" * 50)
    
    client = WebSocketClient(args.url, args.player_id, args.room_id)
    
    try:
        client.connect()
    except KeyboardInterrupt:
        print("\n程序已退出")
    except Exception as e:
        print(f"连接失败: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()

