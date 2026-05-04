# 📜 女书（NvShu）
> **拆开婚姻，重新定义人生协议**
> 一个基于 [Solana](https://solana.com) 区块链的去中心化协议工具，允许任何人自由创建、签署和查询自定义的“婚姻”议定、意定监护、养老互助等多种人生协议。
---

## 💡 核心理念
传统婚姻在法律上是一个 **“打包产品”**：同居、财产、子女、医疗、养老……你只能全部接受，或一个都没有。
**女书把它拆开了。**
我们把人生协议拆成独立的、可定制的模块，让你像自助餐一样自由组合，形成属于自己的专属协议。
> 🍱 传统婚姻 = 固定套餐  
> 🥗 女书协议 = 自助餐
---

## ✨ 当前功能（最小可用版）
| 模块 | 功能 | 使用场景 |
|:---|:---|:---|
| 🏳️‍🌈 “婚姻”议定 | 多人/多性别关系协议 | LGBTQ+伴侣、多元关系 |
| 🏥 意定监护协议 | 授权他人行使医疗/生活决定 | 独居老人、术前授权 |
| 🤝 养老互助协议 | 多人养老权利义务约定 | 抱团养老、社区互助 |
| ✏️ 自定义协议 | 完全自由的条款输入 | 任何人生协议 |

### 功能亮点
- ✅ **多人参与**：支持任意人数的协议，非传统关系友好
- ✅ **多元性别**：六种性别选项，符合国际多元标准
- ✅ **日历有效期**：开始日期 + 结束日期，自动计算
- ✅ **多方签名**：全员签署后自动生效
- ✅ **链上公开查询**：任何人可查看协议详情
- ✅ **AI 法律顾问**：多管辖区法律风险分析（基于 DeepSeek）
- ✅ **女书文化**：以湖南江永女书文字为视觉符号
---

## 🛠️ 技术栈
| 层级 | 技术 |
|:---|:---|
| 区块链 | Solana (Devnet / Localnet) |
| 智能合约 | Anchor (Rust) |
| 前端 | 原生 HTML + JavaScript |
| 链交互 | Solana Web3.js + Anchor SDK |
| AI | DeepSeek Chat API |
| 部署 | 本地测试网 / Solana Devnet |
---

## 📁 项目结构
女书/
├── programs/nvshu/ # Anchor 智能合约
│ └── src/lib.rs # 合约源码 (Rust)
├── target/
│ ├── deploy/nvshu.so # 编译后的合约二进制
│ └── idl/nvshu.json # IDL 接口描述
├── nvshu.html # 前端主页面
└── README.md
---

## 🚀 本地运行
### 前置要求
- [Rust](https://rustup.rs)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation) (v0.30.1)
- Python 3 (用于启动本地服务器)
### 启动步骤
```bash
# 1. 克隆仓库
git clone https://github.com/serenjing23-prog/nvshu.git
cd nvshu
# 2. 启动本地 Solana 测试网（新终端，保持运行）
solana-test-validator
# 3. 部署合约（新终端）
solana config set --url localhost
solana program deploy target/deploy/nvshu.so
# 4. 启动前端服务器
python3 -m http.server 8080
# 5. 打开浏览器
# 访问 http://localhost:8080/nvshu.html
---

🔮 未来模块蓝图
模块	内容设想
💰 财产约定	婚前/婚后财产分别或共有
👶 子女抚养	抚养权、探视权、教育费用
🏥 医疗授权	手术签字、临终关怀决定
📜 遗产规划	遗嘱框架、数字资产继承
🏘️ 社区公约	共同生活群体的权利义务
---

🤝 贡献
本项目为公益开源项目，欢迎提交 Issue 或 PR。
我们特别需要：
UI/UX 设计师
法律专业顾问
多语言翻译
前端/合约开发者
---

📜 许可
MIT License
---

💬 致意
Solana 生态
DeepSeek AI API
湖南江永女书文化传承者
“女书”是世界上唯一的女性文字系统，象征着女性互助与情感契约。本项目以此命名，意在延续这种精神。
