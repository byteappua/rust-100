# Day 10 练习题

## 练习 1: 统计分析工具

文件: `src/exercise1.rs`

给定一个整数列表（Vector），请编写函数分别计算：
1.  **平均数 (Mean)**: 所有数字的总和除以数量。
2.  **中位数 (Median)**: 排序后位于中间的数字。
3.  **众数 (Mode)**: 出现次数最多的数字（使用 HashMap 辅助）。

## 练习 2: 猪拉丁语 (Pig Latin)

文件: `src/exercise2.rs`

编写一个将字符串转换为"猪拉丁语"的程序。规则如下：
1.  如果单词以**元音**开头（a, e, i, o, u），在词尾添加 "-hay"。例如 "apple" -> "apple-hay"。
2.  如果单词以**辅音**开头，将第一个辅音字母移到词尾，并添加 "-ay"。例如 "first" -> "irst-fay"。

输入字符串包含多个单词，以空格分隔。

## 练习 3: 员工管理系统

文件: `src/exercise3.rs`

使用 `HashMap` 和 `Vec` 创建一个简单的文本交互界面，允许用户：
1.  添加员工到部门（例如 "Add Sally to Engineering"）。
2.  列出某个部门的所有员工（例如 "List Engineering"）。
3.  列出公司所有部门的员工（按部门排序）。

注意：为了简化测试，我们可以将交互界面改为通过代码调用函数来模拟。

## ✅ 验证你的答案

```bash
rustc src/exercise1.rs && ./exercise1
rustc src/exercise2.rs && ./exercise2
rustc src/exercise3.rs && ./exercise3
```
