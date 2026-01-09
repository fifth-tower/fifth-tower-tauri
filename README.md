## 脚本助手
   这是一个类似按键精灵的工具，可以将用户的鼠标、键盘操作编写/录制为脚本；支持脚本测试、发布、执行；支持后台运行脚本。脚本支持窗口缩放。
   所有脚本操作（编写、录制、测试、发布、执行）都是图形化的，用户不需要使用或学习任何脚本语言。

   功能包括：  
   
     - 录制
	   - 选择应用、检查窗口、快捷建冲突检查、快捷键修改
	   - 移动鼠标到位置
	   - 移动鼠标到位置，并点击
	   - 匹配图片并点击
	   - 滚动鼠标
	   - 刮一刮
	   - 输入文字
	   - 按下组合键
	   - 执行外部流程
	   - 录制一个子流程
	 - 编写	
	   - 流程克隆、删除
	   - 步骤修改、复制、粘贴、删除
	   - 成功、失败策略修改
	   - 重新录制流程
	   - 重新录制步骤
	   - 流程继续录制
	 - 测试
       - 流程测试
       - 步骤测试	
	   - 停止测试
       - 执行日志
         - 去看看（查看步骤执行后鼠标位置）	   
	 - 发布
       - 脚本设置
	   - 脚本上传
	 - 下载
       - 收藏、评分、下载 
     - 执行
       - 支持多开，支持后台运行 
	   
	技术架构： leptos+tauri+enigo+windows
	
	  - leptos实现图形化界面
	  - tauri对文件、鼠标、键盘、网络等交互操作进行封装，对网页提供通用接口，提供PC版打包机制
      - enigo负责模拟鼠标、键盘的前台操作
      - windows负责模拟鼠标、键盘的后台操作	  

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## dev

cargo tauri dev

## build

cargo tauri build
