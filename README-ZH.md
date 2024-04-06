[English](https://github.com/Somnia1337/EchoUnityArchivist/blob/main/README.md)

# [谐声收藏家](https://github.com/Somnia1337/EchoUnityArchivist)

一个简陋的 email 用户代理，遵循「计算机网络」课程项目 #2 的要求，用 Rust 实现。

“Echo Unity Archivist”(谐声收藏家) 是 “email user agent” 的一个更花哨的名字，它们的首字母相同。

# 课程项目要求

- （已放弃） ~~用户可以通过代理从他们的邮箱下载 email。~~
- 用户可以通过代理发送他们的 email。
- 代理可以访问至少 2 个现实存在的电子邮件服务器。

# 快速上手

运行用户代理：

```shell
cargo run
```

它会请你登录，例如：

```text
You must login before interacting with the server.
SMTP server: smtp.gmail.com (你的目标 SMTP 服务器)
Username: amy2233@gmail.com (你相应的邮箱地址)
SMTP password: jfoiasnelmcpox (类似这样的一串，搜索 “邮箱授权码”)
```

如果一切正常，用户代理将连接到你的服务器，并请你指定一个操作：

```text
> Connected to SMTP server.
> Options:
> 0 Quit
> 1 Send an email
Select an option: 1 (输入你想执行的操作标号)
```

## 发送邮件

输入 “1” 以发送邮件，然后用户代理会请你输入一些细节：

```text
Select an option: 1
To: bob1479@qq.com (收件人的地址)
Subject: Hi! (主题)
Body (press 2 "Enter"s in a row to finish): (输入邮件正文)
(更多)
(的行)
(...)


```

当你编辑完 `Body`(正文) 后，连续按下 “Enter” 键 2 次以告诉用户代理，然后它会请你确认一切无误：

```text
> Seems that you've finished editing,
  if everything looks fine,
  enter "yes" to confirm sending: yes (只有输入 “yes” 才执行发送)
```

如果发送成功，用户代理将打印一条消息：

```text
> Email sent to bob1479@qq.com successfully!
```

## 一个完整示例

```shell
cargo run
```

```text
You must login before interacting with the server.
SMTP server: smtp.gmail.com
Username: amy2233@gmail.com
SMTP password: jfoiasnelmcpox
> Connected to SMTP server.
> Options:
> 0 Quit
> 1 Send an email
Select an option: 1
To: bob1479@qq.com
Subject: Not really sending this time
Body (press 2 "Enter"s in a row to finish): you would not be reading these
because I'm gonna enter "no" to cancel sending


> Seems that you've finished editing,
  if everything looks fine,
  enter "yes" to confirm sending: no
> Sending canceled.
> Options:
> 0 Quit
> 1 Send an email
Select an option: 1
To: bob1479@qq.com
Subject: Ok now it's a real one
Body (press 2 "Enter"s in a row to finish): Hi there!
Goodbye!


> Seems that you've finished editing,
  if everything looks fine,
  enter "yes" to confirm sending: yes
> Email sent to bob1479@qq.com successfully!
> Options:
> 0 Quit
> 1 Send an email
Select an option: 0
> Quitting user agent...
```
