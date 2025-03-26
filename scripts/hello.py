import argparse
import time

def main():
    parser = argparse.ArgumentParser(description='示例 Python 脚本')
    parser.add_argument('--name', default='World', help='要问候的名字')
    args = parser.parse_args()

    print(f"开始执行脚本...")
    time.sleep(1)  # 模拟一些处理时间
    print(f"Hello, {args.name}!")
    print("脚本执行完成。")

if __name__ == '__main__':
    main() 