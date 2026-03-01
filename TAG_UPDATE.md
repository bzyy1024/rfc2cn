# 标签生成和多标签搜索功能更新说明

## 更新内容

### 1. 标签生成逻辑改进
- ✅ **AI生成新标签**：不再匹配数据库中已存在的标签，而是根据RFC内容生成8-12个新标签
- ✅ **自动保存到数据库**：生成的新标签会自动创建并保存到数据库中
- ✅ **多层次标签**：生成的标签涵盖多个层次：
  - 具体协议/技术名（如：oauth2.0, tls1.3, jwt）
  - 技术领域（如：authentication, authorization, security）
  - 应用场景（如：web-security, api-authentication）
  - 核心概念（如：token, session, certificate）

### 2. 多标签AND搜索功能
- ✅ **多标签AND查询**：支持通过多个标签进行AND逻辑搜索
- ✅ **API端点**：`/api/rfcs/search?tags=tag1,tag2,tag3`
- ✅ **查询参数**：
  - `tags`: 逗号分隔的标签列表（使用AND逻辑）
  - `tag`: 单个标签（保持向后兼容）
  - `q`: 关键词搜索
  - `page` & `per_page`: 分页参数

### 3. 测试结果

#### RFC 9110 生成的标签示例
```
- Protocol Architecture
- Extensibility Mechanisms
- Https
- Protocol Standard
- Stateless Protocol
- Web Protocol
- Application Layer
- HTTP
- Uri Scheme
- Common Terminology
- Distributed Systems
- Api Protocol
```

#### 搜索测试
- **单标签搜索** (`tags=http`)：返回所有包含http标签的RFC
- **两标签AND搜索** (`tags=http,web-protocol`)：返回同时包含两个标签的RFC
- **三标签AND搜索** (`tags=http,web-protocol,protocol-standard`)：返回RFC 9110（正确）
- **不匹配搜索** (`tags=http,oauth`)：返回空结果（正确）

## 代码修改

### 修改的文件

1. **backend/src/services/ai.rs**
   - 更新AI提示词，要求生成8-12个标签
   - 增加标签数量上限到12个

2. **backend/src/bin/cli.rs**
   - 修改 `extract_and_add_tags` 函数，自动创建不存在的新标签
   - 标签名称自动格式化（首字母大写，连字符替换为空格）

3. **backend/src/models/rfc.rs**
   - 在 `SearchRfcQuery` 中添加 `tags` 字段支持多标签查询

4. **backend/src/services/rfc.rs**
   - 更新 `search_rfcs` 函数，添加多标签AND搜索逻辑
   - 使用 PostgreSQL 的 `ANY` 和 `HAVING COUNT(DISTINCT)` 实现AND查询

5. **backend/src/services/tag.rs**
   - 添加 `get_rfcs_by_tags` 函数（用于未来扩展）

## 使用方法

### CLI命令
```bash
# 添加RFC并自动生成标签
cargo run --bin rfc-cli -- add 9110 --auto-tag

# 查看所有标签
cargo run --bin rfc-cli -- tags

# 查看RFC状态和标签
cargo run --bin rfc-cli -- status 9110
```

### API调用
```bash
# 单标签搜索
curl "http://localhost:3000/api/rfcs/search?tags=http"

# 多标签AND搜索
curl "http://localhost:3000/api/rfcs/search?tags=http,web-protocol"

# 三标签AND搜索
curl "http://localhost:3000/api/rfcs/search?tags=http,web-protocol,protocol-standard"
```

## 数据库变化
- 标签表 `tags` 会自动增加新生成的标签记录
- RFC与标签的关联存储在 `rfc_tags` 表中
- 每个RFC可以有多个标签，支持多对多关系

## 注意事项
- 标签生成需要Ollama或OpenAI API服务运行
- 生成的标签使用小写英文，多个单词用连字符连接
- 标签名称会自动格式化为首字母大写的形式用于显示
- 多标签搜索使用AND逻辑，只返回同时包含所有指定标签的RFC
