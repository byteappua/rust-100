// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="index.html">Introduction</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="QUICK_START.html"><strong aria-hidden="true">1.</strong> 快速开始指南</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="RUST_100_DAYS_COMPLETE.html"><strong aria-hidden="true">2.</strong> 完整学习指南</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="PROJECT_COMPLETION_STATUS.html"><strong aria-hidden="true">3.</strong> 项目完成状态</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="COMPLETION_SUMMARY.html"><strong aria-hidden="true">4.</strong> 完成总结</a></span></li><li class="chapter-item expanded "><li class="spacer"></li></li><li class="chapter-item expanded "><li class="part-title">第一阶段：基础知识 (Days 1-15)</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">5.</strong> 语法入门</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day01-15/01.Introduction/index.html"><strong aria-hidden="true">5.1.</strong> Day 01: 初识 Rust</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day01-15/02.VariablesAndTypes/index.html"><strong aria-hidden="true">5.2.</strong> Day 02: 变量与数据类型</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day01-15/03.FunctionsAndControlFlow/index.html"><strong aria-hidden="true">5.3.</strong> Day 03: 函数与控制流</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">6.</strong> 所有权机制</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day01-15/04.Ownership/index.html"><strong aria-hidden="true">6.1.</strong> Day 04: 所有权</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day01-15/05.ReferencesAndBorrowing/index.html"><strong aria-hidden="true">6.2.</strong> Day 05: 引用与借用</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day01-15/06.Slices/index.html"><strong aria-hidden="true">6.3.</strong> Day 06: 切片类型</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">7.</strong> 数据结构与组织</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day01-15/07.Structs/index.html"><strong aria-hidden="true">7.1.</strong> Day 07: 结构体</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day01-15/08.Enums/index.html"><strong aria-hidden="true">7.2.</strong> Day 08: 枚举与模式匹配</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day01-15/09.Modules/index.html"><strong aria-hidden="true">7.3.</strong> Day 09: 模块系统</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day01-15/10.CommonCollections/index.html"><strong aria-hidden="true">7.4.</strong> Day 10: 常用集合</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">8.</strong> 抽象与错误处理</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day01-15/11.ErrorHandling/index.html"><strong aria-hidden="true">8.1.</strong> Day 11: 错误处理</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day01-15/12.Generics/index.html"><strong aria-hidden="true">8.2.</strong> Day 12: 泛型</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day01-15/13.Traits/index.html"><strong aria-hidden="true">8.3.</strong> Day 13: Trait</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day01-15/14.Lifetimes/index.html"><strong aria-hidden="true">8.4.</strong> Day 14: 生命周期</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">9.</strong> 工程实践</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day01-15/15.Testing/index.html"><strong aria-hidden="true">9.1.</strong> Day 15: 自动化测试</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">第二阶段：进阶概念 (Days 16-30)</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">10.</strong> 函数式特性</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day16-30/16.Closures/index.html"><strong aria-hidden="true">10.1.</strong> Day 16: 闭包</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day16-30/17.Iterators/index.html"><strong aria-hidden="true">10.2.</strong> Day 17: 迭代器</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">11.</strong> 工程进阶</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day16-30/18.CargoAndCrates/index.html"><strong aria-hidden="true">11.1.</strong> Day 18: Cargo 进阶</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">12.</strong> 智能指针</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day16-30/19.SmartPointers_Box/index.html"><strong aria-hidden="true">12.1.</strong> Day 19: 智能指针 Box</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day16-30/20.SmartPointers_RcRefCell/index.html"><strong aria-hidden="true">12.2.</strong> Day 20: 智能指针 Rc 与 RefCell</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day16-30/21.ReferenceCycles/index.html"><strong aria-hidden="true">12.3.</strong> Day 21: 引用循环与内存泄漏</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">13.</strong> 并发编程</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day16-30/22.Threads/index.html"><strong aria-hidden="true">13.1.</strong> Day 22: 并发 - 线程</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day16-30/23.MessagePassing/index.html"><strong aria-hidden="true">13.2.</strong> Day 23: 并发 - 消息传递</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day16-30/24.SharedState/index.html"><strong aria-hidden="true">13.3.</strong> Day 24: 并发 - 共享状态</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day16-30/25.SendSync/index.html"><strong aria-hidden="true">13.4.</strong> Day 25: 并发 - Send 与 Sync</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">14.</strong> 面向对象与模式</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day16-30/26.OOP_TraitObjects/index.html"><strong aria-hidden="true">14.1.</strong> Day 26: 面向对象特性 - Trait 对象</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day16-30/27.Patterns/index.html"><strong aria-hidden="true">14.2.</strong> Day 27: 模式与匹配</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">15.</strong> 高级特性</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day16-30/28.AdvancedTraits/index.html"><strong aria-hidden="true">15.1.</strong> Day 28: 高级 Trait</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day16-30/29.AdvancedTypes/index.html"><strong aria-hidden="true">15.2.</strong> Day 29: 高级类型</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day16-30/30.AdvancedFnAndMacros/index.html"><strong aria-hidden="true">15.3.</strong> Day 30: 高级函数与宏</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">第三阶段：实战与生态 (Days 31-45)</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">16.</strong> 实用工具与 IO</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day31-45/31.FileIO/index.html"><strong aria-hidden="true">16.1.</strong> Day 31: 文件输入与输出</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day31-45/32.Serialization_Serde/index.html"><strong aria-hidden="true">16.2.</strong> Day 32: 序列化与反序列化 (Serde)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day31-45/33.Minigrep/index.html"><strong aria-hidden="true">16.3.</strong> Day 33: 构建命令行工具 (Minigrep)</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">17.</strong> 网络与异步编程</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day31-45/34.Network_TCP/index.html"><strong aria-hidden="true">17.1.</strong> Day 34: 网络编程基础 (TCP)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day31-45/35.AsyncAwait/index.html"><strong aria-hidden="true">17.2.</strong> Day 35: 异步编程入门 (Async/Await)</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">18.</strong> 底层与高级特性</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day31-45/36.UnsafeRust/index.html"><strong aria-hidden="true">18.1.</strong> Day 36: Unsafe Rust 介绍</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day31-45/37.ProceduralMacros/index.html"><strong aria-hidden="true">18.2.</strong> Day 37: 过程宏 (Procedural Macros)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day31-45/38.FFI/index.html"><strong aria-hidden="true">18.3.</strong> Day 38: FFI - 与 C 语言交互</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">19.</strong> Web 开发生态</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day31-45/39.TokioDeepDive/index.html"><strong aria-hidden="true">19.1.</strong> Day 39: Tokio 运行时深入</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day31-45/40.AsyncWebServer/index.html"><strong aria-hidden="true">19.2.</strong> Day 40: 构建异步 Web 服务器</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day31-45/41.Database_SQLx/index.html"><strong aria-hidden="true">19.3.</strong> Day 41: 数据库交互 (SQLx/Diesel)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day31-45/42.Tracing/index.html"><strong aria-hidden="true">19.4.</strong> Day 42: 日志与监控 (Tracing)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day31-45/43.ErrorHandling/index.html"><strong aria-hidden="true">19.5.</strong> Day 43: 错误处理最佳实践</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day31-45/44.Clap/index.html"><strong aria-hidden="true">19.6.</strong> Day 44: 命令行参数解析 (Clap)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day31-45/45.Reqwest/index.html"><strong aria-hidden="true">19.7.</strong> Day 45: HTTP 客户端 (Reqwest)</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">第四阶段：异步网络项目实战 (Days 46-60)</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">20.</strong> 项目启动与基础</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day46-60/46.ProjectInit/index.html"><strong aria-hidden="true">20.1.</strong> Day 46: 项目初始化与配置</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day46-60/47.RespParsing/index.html"><strong aria-hidden="true">20.2.</strong> Day 47: 协议解析 (Resp)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day46-60/48.CommandArchitecture/index.html"><strong aria-hidden="true">20.3.</strong> Day 48: 命令处理架构</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day46-60/49.StorageEngine/index.html"><strong aria-hidden="true">20.4.</strong> Day 49: 存储引擎实现</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day46-60/50.AsyncNetwork/index.html"><strong aria-hidden="true">20.5.</strong> Day 50: 异步网络层 (Tokio)</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">21.</strong> 功能完善与优化</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day46-60/51.Concurrency/index.html"><strong aria-hidden="true">21.1.</strong> Day 51: 并发控制与锁</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day46-60/52.Persistence/index.html"><strong aria-hidden="true">21.2.</strong> Day 52: 持久化机制 (AOF/RDB)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day46-60/53.PubSub/index.html"><strong aria-hidden="true">21.3.</strong> Day 53: 发布订阅 (Pub/Sub)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day46-60/54.ClientSDK/index.html"><strong aria-hidden="true">21.4.</strong> Day 54: 客户端 SDK 实现</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day46-60/55.Benchmark/index.html"><strong aria-hidden="true">21.5.</strong> Day 55: 性能基准测试</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">22.</strong> 进阶功能</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day46-60/56.ClusterMode/index.html"><strong aria-hidden="true">22.1.</strong> Day 56: 集群模式探索</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day46-60/57.Sentinel/index.html"><strong aria-hidden="true">22.2.</strong> Day 57: 哨兵与高可用</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day46-60/58.TLS/index.html"><strong aria-hidden="true">22.3.</strong> Day 58: 安全性 (TLS)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day46-60/59.Documentation/index.html"><strong aria-hidden="true">22.4.</strong> Day 59: 完善文档与示例</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day46-60/60.CICD/index.html"><strong aria-hidden="true">22.5.</strong> Day 60: 项目发布与 CI/CD</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">第五阶段：Web 开发实战 (Days 61-80)</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day61-80/STAGE5_OVERVIEW.html"><strong aria-hidden="true">23.</strong> 阶段概览</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">24.</strong> Web 框架基础</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day61-80/61.WebOverview/index.html"><strong aria-hidden="true">24.1.</strong> Day 61: Rust Web 开发概览</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day61-80/62.AxumActix/index.html"><strong aria-hidden="true">24.2.</strong> Day 62: Axum/Actix-web 入门</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day61-80/63.RoutingHandlers/index.html"><strong aria-hidden="true">24.3.</strong> Day 63: 路由与请求处理</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day61-80/64.Middleware/index.html"><strong aria-hidden="true">24.4.</strong> Day 64: 中间件开发</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day61-80/65.StateManagement/index.html"><strong aria-hidden="true">24.5.</strong> Day 65: 请求状态管理</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">25.</strong> 数据库与业务逻辑</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day61-80/66.Migrations/index.html"><strong aria-hidden="true">25.1.</strong> Day 66: 数据库迁移 (Migrations)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day61-80/67.CRUD/index.html"><strong aria-hidden="true">25.2.</strong> Day 67: CRUD 操作实战</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day61-80/68.ComplexQueries/index.html"><strong aria-hidden="true">25.3.</strong> Day 68: 复杂查询与事务</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day61-80/69.JWT/index.html"><strong aria-hidden="true">25.4.</strong> Day 69: 用户认证 (JWT)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day61-80/70.RBAC/index.html"><strong aria-hidden="true">25.5.</strong> Day 70: 权限控制 (RBAC)</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">26.</strong> 前端交互与部署</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day61-80/71.Templates/index.html"><strong aria-hidden="true">26.1.</strong> Day 71: 模板引擎 (Askama/Tera)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day61-80/72.WASM/index.html"><strong aria-hidden="true">26.2.</strong> Day 72: WebAssembly (WASM) 简介</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day61-80/73.FrontendIntegration/index.html"><strong aria-hidden="true">26.3.</strong> Day 73: 前端集成实战</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day61-80/74.OpenAPI/index.html"><strong aria-hidden="true">26.4.</strong> Day 74: API 文档生成 (OpenAPI)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day61-80/75.Docker/index.html"><strong aria-hidden="true">26.5.</strong> Day 75: 容器化部署 (Docker)</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">27.</strong> 完整项目实战 (博客系统)</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day61-80/76.BlogDesign/index.html"><strong aria-hidden="true">27.1.</strong> Day 76: 需求分析与设计</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day61-80/77.BlogCore/index.html"><strong aria-hidden="true">27.2.</strong> Day 77: 核心业务实现</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day61-80/78.BlogAPI/index.html"><strong aria-hidden="true">27.3.</strong> Day 78: API 接口开发</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day61-80/79.BlogTesting/index.html"><strong aria-hidden="true">27.4.</strong> Day 79: 测试与优化</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day61-80/80.BlogSummary/index.html"><strong aria-hidden="true">27.5.</strong> Day 80: 项目总结与展示</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">第六阶段：系统编程与拓展 (Days 81-90)</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day81-90/STAGE6_OVERVIEW.html"><strong aria-hidden="true">28.</strong> 阶段概览</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">29.</strong> 系统编程</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day81-90/81.OSConcepts/index.html"><strong aria-hidden="true">29.1.</strong> Day 81: 操作系统概念回顾</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day81-90/82.FileSystem/index.html"><strong aria-hidden="true">29.2.</strong> Day 82: 文件系统操作深入</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day81-90/83.ProcessManagement/index.html"><strong aria-hidden="true">29.3.</strong> Day 83: 进程管理与信号处理</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day81-90/84.RawSockets/index.html"><strong aria-hidden="true">29.4.</strong> Day 84: 原始套接字 (Raw Sockets)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day81-90/85.EmbeddedRust/index.html"><strong aria-hidden="true">29.5.</strong> Day 85: 嵌入式 Rust 简介</a></span></li></ol><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">30.</strong> 性能与工程化</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day81-90/86.Profiling/index.html"><strong aria-hidden="true">30.1.</strong> Day 86: 性能分析工具 (Flamegraph)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day81-90/87.MemoryOptimization/index.html"><strong aria-hidden="true">30.2.</strong> Day 87: 内存优化技巧</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day81-90/88.SIMD/index.html"><strong aria-hidden="true">30.3.</strong> Day 88: SIMD 与底层优化</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day81-90/89.CICD/index.html"><strong aria-hidden="true">30.4.</strong> Day 89: 持续集成流水线</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day81-90/90.CompilerOptimization/index.html"><strong aria-hidden="true">30.5.</strong> Day 90: Rust 编译优化与配置</a></span></li></ol><li class="chapter-item expanded "><li class="part-title">第七阶段：毕业设计 (Days 91-100)</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day91-100/STAGE7_OVERVIEW.html"><strong aria-hidden="true">31.</strong> 阶段概览</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">32.</strong> 综合项目开发</span></span><ol class="section"><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day91-100/91.ProjectDesign/index.html"><strong aria-hidden="true">32.1.</strong> Day 91: 选题与架构设计</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day91-100/92.PoC/index.html"><strong aria-hidden="true">32.2.</strong> Day 92: 原型验证 (PoC)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day91-100/93.CoreDevelopment1/index.html"><strong aria-hidden="true">32.3.</strong> Day 93: 核心模块开发 (一)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day91-100/94.CoreDevelopment2/index.html"><strong aria-hidden="true">32.4.</strong> Day 94: 核心模块开发 (二)</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day91-100/95.Integration/index.html"><strong aria-hidden="true">32.5.</strong> Day 95: 外围模块与集成</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day91-100/96.Performance/index.html"><strong aria-hidden="true">32.6.</strong> Day 96: 性能测试与调优</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day91-100/97.Documentation/index.html"><strong aria-hidden="true">32.7.</strong> Day 97: 文档编写与代码清理</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day91-100/98.Release/index.html"><strong aria-hidden="true">32.8.</strong> Day 98: 发布准备与版本管理</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day91-100/99.Presentation/index.html"><strong aria-hidden="true">32.9.</strong> Day 99: 项目展示与复盘</a></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><a href="Day91-100/100.Rustacean/index.html"><strong aria-hidden="true">32.10.</strong> Day 100: 成为 Rustacean 之路</a></span></li></ol><li class="chapter-item expanded "><li class="spacer"></li></li><li class="chapter-item expanded "><li class="part-title">附录</li></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">33.</strong> 学习资源汇总</span></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">34.</strong> 常见问题 FAQ</span></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">35.</strong> 术语表</span></span></li><li class="chapter-item expanded "><span class="chapter-link-wrapper"><span><strong aria-hidden="true">36.</strong> 贡献指南</span></span></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split('#')[0].split('?')[0];
        if (current_page.endsWith('/')) {
            current_page += 'index.html';
        }
        const links = Array.prototype.slice.call(this.querySelectorAll('a'));
        const l = links.length;
        for (let i = 0; i < l; ++i) {
            const link = links[i];
            const href = link.getAttribute('href');
            if (href && !href.startsWith('#') && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The 'index' page is supposed to alias the first chapter in the book.
            if (link.href === current_page
                || i === 0
                && path_to_root === ''
                && current_page.endsWith('/index.html')) {
                link.classList.add('active');
                let parent = link.parentElement;
                while (parent) {
                    if (parent.tagName === 'LI' && parent.classList.contains('chapter-item')) {
                        parent.classList.add('expanded');
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', e => {
            if (e.target.tagName === 'A') {
                const clientRect = e.target.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                sessionStorage.setItem('sidebar-scroll-offset', clientRect.top - sidebarRect.top);
            }
        }, { passive: true });
        const sidebarScrollOffset = sessionStorage.getItem('sidebar-scroll-offset');
        sessionStorage.removeItem('sidebar-scroll-offset');
        if (sidebarScrollOffset !== null) {
            // preserve sidebar scroll position when navigating via links within sidebar
            const activeSection = this.querySelector('.active');
            if (activeSection) {
                const clientRect = activeSection.getBoundingClientRect();
                const sidebarRect = this.getBoundingClientRect();
                const currentOffset = clientRect.top - sidebarRect.top;
                this.scrollTop += currentOffset - parseFloat(sidebarScrollOffset);
            }
        } else {
            // scroll sidebar to current active section when navigating via
            // 'next/previous chapter' buttons
            const activeSection = document.querySelector('#mdbook-sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        const sidebarAnchorToggles = document.querySelectorAll('.chapter-fold-toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(el => {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define('mdbook-sidebar-scrollbox', MDBookSidebarScrollbox);


// ---------------------------------------------------------------------------
// Support for dynamically adding headers to the sidebar.

(function() {
    // This is used to detect which direction the page has scrolled since the
    // last scroll event.
    let lastKnownScrollPosition = 0;
    // This is the threshold in px from the top of the screen where it will
    // consider a header the "current" header when scrolling down.
    const defaultDownThreshold = 150;
    // Same as defaultDownThreshold, except when scrolling up.
    const defaultUpThreshold = 300;
    // The threshold is a virtual horizontal line on the screen where it
    // considers the "current" header to be above the line. The threshold is
    // modified dynamically to handle headers that are near the bottom of the
    // screen, and to slightly offset the behavior when scrolling up vs down.
    let threshold = defaultDownThreshold;
    // This is used to disable updates while scrolling. This is needed when
    // clicking the header in the sidebar, which triggers a scroll event. It
    // is somewhat finicky to detect when the scroll has finished, so this
    // uses a relatively dumb system of disabling scroll updates for a short
    // time after the click.
    let disableScroll = false;
    // Array of header elements on the page.
    let headers;
    // Array of li elements that are initially collapsed headers in the sidebar.
    // I'm not sure why eslint seems to have a false positive here.
    // eslint-disable-next-line prefer-const
    let headerToggles = [];
    // This is a debugging tool for the threshold which you can enable in the console.
    let thresholdDebug = false;

    // Updates the threshold based on the scroll position.
    function updateThreshold() {
        const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
        const windowHeight = window.innerHeight;
        const documentHeight = document.documentElement.scrollHeight;

        // The number of pixels below the viewport, at most documentHeight.
        // This is used to push the threshold down to the bottom of the page
        // as the user scrolls towards the bottom.
        const pixelsBelow = Math.max(0, documentHeight - (scrollTop + windowHeight));
        // The number of pixels above the viewport, at least defaultDownThreshold.
        // Similar to pixelsBelow, this is used to push the threshold back towards
        // the top when reaching the top of the page.
        const pixelsAbove = Math.max(0, defaultDownThreshold - scrollTop);
        // How much the threshold should be offset once it gets close to the
        // bottom of the page.
        const bottomAdd = Math.max(0, windowHeight - pixelsBelow - defaultDownThreshold);
        let adjustedBottomAdd = bottomAdd;

        // Adjusts bottomAdd for a small document. The calculation above
        // assumes the document is at least twice the windowheight in size. If
        // it is less than that, then bottomAdd needs to be shrunk
        // proportional to the difference in size.
        if (documentHeight < windowHeight * 2) {
            const maxPixelsBelow = documentHeight - windowHeight;
            const t = 1 - pixelsBelow / Math.max(1, maxPixelsBelow);
            const clamp = Math.max(0, Math.min(1, t));
            adjustedBottomAdd *= clamp;
        }

        let scrollingDown = true;
        if (scrollTop < lastKnownScrollPosition) {
            scrollingDown = false;
        }

        if (scrollingDown) {
            // When scrolling down, move the threshold up towards the default
            // downwards threshold position. If near the bottom of the page,
            // adjustedBottomAdd will offset the threshold towards the bottom
            // of the page.
            const amountScrolledDown = scrollTop - lastKnownScrollPosition;
            const adjustedDefault = defaultDownThreshold + adjustedBottomAdd;
            threshold = Math.max(adjustedDefault, threshold - amountScrolledDown);
        } else {
            // When scrolling up, move the threshold down towards the default
            // upwards threshold position. If near the bottom of the page,
            // quickly transition the threshold back up where it normally
            // belongs.
            const amountScrolledUp = lastKnownScrollPosition - scrollTop;
            const adjustedDefault = defaultUpThreshold - pixelsAbove
                + Math.max(0, adjustedBottomAdd - defaultDownThreshold);
            threshold = Math.min(adjustedDefault, threshold + amountScrolledUp);
        }

        if (documentHeight <= windowHeight) {
            threshold = 0;
        }

        if (thresholdDebug) {
            const id = 'mdbook-threshold-debug-data';
            let data = document.getElementById(id);
            if (data === null) {
                data = document.createElement('div');
                data.id = id;
                data.style.cssText = `
                    position: fixed;
                    top: 50px;
                    right: 10px;
                    background-color: 0xeeeeee;
                    z-index: 9999;
                    pointer-events: none;
                `;
                document.body.appendChild(data);
            }
            data.innerHTML = `
                <table>
                  <tr><td>documentHeight</td><td>${documentHeight.toFixed(1)}</td></tr>
                  <tr><td>windowHeight</td><td>${windowHeight.toFixed(1)}</td></tr>
                  <tr><td>scrollTop</td><td>${scrollTop.toFixed(1)}</td></tr>
                  <tr><td>pixelsAbove</td><td>${pixelsAbove.toFixed(1)}</td></tr>
                  <tr><td>pixelsBelow</td><td>${pixelsBelow.toFixed(1)}</td></tr>
                  <tr><td>bottomAdd</td><td>${bottomAdd.toFixed(1)}</td></tr>
                  <tr><td>adjustedBottomAdd</td><td>${adjustedBottomAdd.toFixed(1)}</td></tr>
                  <tr><td>scrollingDown</td><td>${scrollingDown}</td></tr>
                  <tr><td>threshold</td><td>${threshold.toFixed(1)}</td></tr>
                </table>
            `;
            drawDebugLine();
        }

        lastKnownScrollPosition = scrollTop;
    }

    function drawDebugLine() {
        if (!document.body) {
            return;
        }
        const id = 'mdbook-threshold-debug-line';
        const existingLine = document.getElementById(id);
        if (existingLine) {
            existingLine.remove();
        }
        const line = document.createElement('div');
        line.id = id;
        line.style.cssText = `
            position: fixed;
            top: ${threshold}px;
            left: 0;
            width: 100vw;
            height: 2px;
            background-color: red;
            z-index: 9999;
            pointer-events: none;
        `;
        document.body.appendChild(line);
    }

    function mdbookEnableThresholdDebug() {
        thresholdDebug = true;
        updateThreshold();
        drawDebugLine();
    }

    window.mdbookEnableThresholdDebug = mdbookEnableThresholdDebug;

    // Updates which headers in the sidebar should be expanded. If the current
    // header is inside a collapsed group, then it, and all its parents should
    // be expanded.
    function updateHeaderExpanded(currentA) {
        // Add expanded to all header-item li ancestors.
        let current = currentA.parentElement;
        while (current) {
            if (current.tagName === 'LI' && current.classList.contains('header-item')) {
                current.classList.add('expanded');
            }
            current = current.parentElement;
        }
    }

    // Updates which header is marked as the "current" header in the sidebar.
    // This is done with a virtual Y threshold, where headers at or below
    // that line will be considered the current one.
    function updateCurrentHeader() {
        if (!headers || !headers.length) {
            return;
        }

        // Reset the classes, which will be rebuilt below.
        const els = document.getElementsByClassName('current-header');
        for (const el of els) {
            el.classList.remove('current-header');
        }
        for (const toggle of headerToggles) {
            toggle.classList.remove('expanded');
        }

        // Find the last header that is above the threshold.
        let lastHeader = null;
        for (const header of headers) {
            const rect = header.getBoundingClientRect();
            if (rect.top <= threshold) {
                lastHeader = header;
            } else {
                break;
            }
        }
        if (lastHeader === null) {
            lastHeader = headers[0];
            const rect = lastHeader.getBoundingClientRect();
            const windowHeight = window.innerHeight;
            if (rect.top >= windowHeight) {
                return;
            }
        }

        // Get the anchor in the summary.
        const href = '#' + lastHeader.id;
        const a = [...document.querySelectorAll('.header-in-summary')]
            .find(element => element.getAttribute('href') === href);
        if (!a) {
            return;
        }

        a.classList.add('current-header');

        updateHeaderExpanded(a);
    }

    // Updates which header is "current" based on the threshold line.
    function reloadCurrentHeader() {
        if (disableScroll) {
            return;
        }
        updateThreshold();
        updateCurrentHeader();
    }


    // When clicking on a header in the sidebar, this adjusts the threshold so
    // that it is located next to the header. This is so that header becomes
    // "current".
    function headerThresholdClick(event) {
        // See disableScroll description why this is done.
        disableScroll = true;
        setTimeout(() => {
            disableScroll = false;
        }, 100);
        // requestAnimationFrame is used to delay the update of the "current"
        // header until after the scroll is done, and the header is in the new
        // position.
        requestAnimationFrame(() => {
            requestAnimationFrame(() => {
                // Closest is needed because if it has child elements like <code>.
                const a = event.target.closest('a');
                const href = a.getAttribute('href');
                const targetId = href.substring(1);
                const targetElement = document.getElementById(targetId);
                if (targetElement) {
                    threshold = targetElement.getBoundingClientRect().bottom;
                    updateCurrentHeader();
                }
            });
        });
    }

    // Takes the nodes from the given head and copies them over to the
    // destination, along with some filtering.
    function filterHeader(source, dest) {
        const clone = source.cloneNode(true);
        clone.querySelectorAll('mark').forEach(mark => {
            mark.replaceWith(...mark.childNodes);
        });
        dest.append(...clone.childNodes);
    }

    // Scans page for headers and adds them to the sidebar.
    document.addEventListener('DOMContentLoaded', function() {
        const activeSection = document.querySelector('#mdbook-sidebar .active');
        if (activeSection === null) {
            return;
        }

        const main = document.getElementsByTagName('main')[0];
        headers = Array.from(main.querySelectorAll('h2, h3, h4, h5, h6'))
            .filter(h => h.id !== '' && h.children.length && h.children[0].tagName === 'A');

        if (headers.length === 0) {
            return;
        }

        // Build a tree of headers in the sidebar.

        const stack = [];

        const firstLevel = parseInt(headers[0].tagName.charAt(1));
        for (let i = 1; i < firstLevel; i++) {
            const ol = document.createElement('ol');
            ol.classList.add('section');
            if (stack.length > 0) {
                stack[stack.length - 1].ol.appendChild(ol);
            }
            stack.push({level: i + 1, ol: ol});
        }

        // The level where it will start folding deeply nested headers.
        const foldLevel = 3;

        for (let i = 0; i < headers.length; i++) {
            const header = headers[i];
            const level = parseInt(header.tagName.charAt(1));

            const currentLevel = stack[stack.length - 1].level;
            if (level > currentLevel) {
                // Begin nesting to this level.
                for (let nextLevel = currentLevel + 1; nextLevel <= level; nextLevel++) {
                    const ol = document.createElement('ol');
                    ol.classList.add('section');
                    const last = stack[stack.length - 1];
                    const lastChild = last.ol.lastChild;
                    // Handle the case where jumping more than one nesting
                    // level, which doesn't have a list item to place this new
                    // list inside of.
                    if (lastChild) {
                        lastChild.appendChild(ol);
                    } else {
                        last.ol.appendChild(ol);
                    }
                    stack.push({level: nextLevel, ol: ol});
                }
            } else if (level < currentLevel) {
                while (stack.length > 1 && stack[stack.length - 1].level > level) {
                    stack.pop();
                }
            }

            const li = document.createElement('li');
            li.classList.add('header-item');
            li.classList.add('expanded');
            if (level < foldLevel) {
                li.classList.add('expanded');
            }
            const span = document.createElement('span');
            span.classList.add('chapter-link-wrapper');
            const a = document.createElement('a');
            span.appendChild(a);
            a.href = '#' + header.id;
            a.classList.add('header-in-summary');
            filterHeader(header.children[0], a);
            a.addEventListener('click', headerThresholdClick);
            const nextHeader = headers[i + 1];
            if (nextHeader !== undefined) {
                const nextLevel = parseInt(nextHeader.tagName.charAt(1));
                if (nextLevel > level && level >= foldLevel) {
                    const toggle = document.createElement('a');
                    toggle.classList.add('chapter-fold-toggle');
                    toggle.classList.add('header-toggle');
                    toggle.addEventListener('click', () => {
                        li.classList.toggle('expanded');
                    });
                    const toggleDiv = document.createElement('div');
                    toggleDiv.textContent = '❱';
                    toggle.appendChild(toggleDiv);
                    span.appendChild(toggle);
                    headerToggles.push(li);
                }
            }
            li.appendChild(span);

            const currentParent = stack[stack.length - 1];
            currentParent.ol.appendChild(li);
        }

        const onThisPage = document.createElement('div');
        onThisPage.classList.add('on-this-page');
        onThisPage.append(stack[0].ol);
        const activeItemSpan = activeSection.parentElement;
        activeItemSpan.after(onThisPage);
    });

    document.addEventListener('DOMContentLoaded', reloadCurrentHeader);
    document.addEventListener('scroll', reloadCurrentHeader, { passive: true });
})();

