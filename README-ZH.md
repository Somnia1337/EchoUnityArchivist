[English](https://github.com/Somnia1337/EchoUnityArchivist/blob/main/README.md)

# [谐声收藏家](https://github.com/Somnia1337/EchoUnityArchivist)

一个简陋的 email 用户代理，遵循「计算机网络」课程项目 #2 的要求，用 Rust 实现。

“Echo Unity Archivist”(谐声收藏家) 是 “email user agent” 的一个更花哨的名字，它们的首字母相同。

# 课程项目要求

- 用户可以通过代理从他们的邮箱下载 email。
- 用户可以通过代理发送他们的 email。
- 代理可以访问至少 2 个现实存在的电子邮件服务器。

# 快速上手

运行用户代理：

```shell
cargo run
```

它会请你登录，例如：

```text
> Logging in is required before interacting with the SMTP/IMAP server.
  Server domain (eg. "smtp.qq.com"): smtp.gmail.com (你的目标 SMTP 服务器)
  Email: amy1024@gmail.com (你相应的邮箱地址)
  Password (SMTP/IMAP password, eg. "jfoaiwnpsej"): ioaenfkuhvkusanelx (类似这样的一串，搜索 “邮箱授权码” 获取更多帮助)
```

如果一切正常，用户代理将连接到你的服务器，并请你指定一个操作：

```text
> Connected to smtp.gmail.com.
> Connected to imap.gmail.com.
> Actions:
  0 Logout & quit
  1 Send email
  2 Fetch email
  Select an action: 2 (输入你想执行的操作标号)
```

## 发送邮件

输入 “1” 以发送邮件，然后用户代理会请你输入一些细节：

```text
  Select an action: 1
> New draft:
  -------------------------------------
  To: bob2048@gmail.com (收件人的地址)
  Subject: Hi from Amy! (主题)
  Body (press 2 `Enter`s in a row to finish):
  Hi Bob!
  Nice 2 c u 2!


  -------------------------------------
```

当你编辑完 `Body`(正文) 后，连续按下 `Enter` 键 2 次以告诉用户代理，然后它会请你确认一切无误：

```text
> You have finished editing,
  if everything looks fine,
  enter "yes" to confirm sending: yes (只有输入 “yes” 才执行发送)
```

如果发送成功，用户代理将打印一条消息：

```text
> Email sent to bob2048@gmail.com successfully!
```

## 收取邮件

输入 “2” 以收取邮件，然后用户代理会请你选择一个收信箱：

```text
> Fetching inboxes...
  [1] INBOX
  [2] Sent Messages
  [3] Drafts
  [4] Deleted Messages
  [5] Junk
  Select an inbox:
```

选择一个之后，用户代理将检查其中是否有邮件，如果有就收取第一封：

```text
  Select an inbox: 1
> Email fetched:
  -------------------------------------
  From: bob2048@gmail.com
  To: amy1024@gmail.com
  Subject: Hi from Bob!
  Date: Sun, 07 Apr 2024 09:09:27 +0000
  
  Hi Amy!
  Nice 2 c u!
  
  -------------------------------------
```

## 一个完整示例

```shell
cargo run
```

```text
> Logging in is required before interacting with the SMTP/IMAP server.
  Server domain (eg. "smtp.qq.com"): smtp.gmail.com
  Email: amy1024@gmail.com
  Password (SMTP/IMAP password, eg. "jfoaiwnpsej"): ioaenfkuhvkusanelx
> Connected to smtp.gmail.com.
> Connected to imap.gmail.com.
> Actions:
  0 Logout & quit
  1 Send email
  2 Fetch email
  Select an action: 2
> Fetching inboxes...
  [1] INBOX
  [2] Sent Messages
  [3] Drafts
  [4] Deleted Messages
  [5] Junk
  Select an inbox: 1
> Email fetched:
  -------------------------------------
  From: bob2048@gmail.com
  To: amy1024@gmail.com
  Subject: Hi from Bob!
  Date: Sun, 07 Apr 2024 09:09:27 +0000
  
  Hi Amy!
  Nice 2 c u!
  
  -------------------------------------
> Actions:
  0 Logout & quit
  1 Send email
  2 Fetch email
  Select an action: 1
> New draft:
  -------------------------------------
  To: bob2048@gmail.com
  Subject: Hi from Amy!
  Body (press 2 `Enter`s in a row to finish):
  Hi Bob!
  Nise 2 c u 2!
  Oops, I made a typo...


  -------------------------------------
> You have finished editing,
  if everything looks fine,
  enter "yes" to confirm sending: no
> Sending canceled.
> Actions:
  0 Logout & quit
  1 Send email
  2 Fetch email
  Select an action: 1
> New draft:
  -------------------------------------
  To: bob2048@gmail.com
  Subject: Hi from Amy!
  Body (press 2 `Enter`s in a row to finish):
  Hi Bob!
  Nice 2 c u 2!


  -------------------------------------
> You have finished editing,
  if everything looks fine,
  enter "yes" to confirm sending: yes
> Sending...
> Your email has been sent to bob2048@gmail.com.
> Actions:
  0 Logout & quit
  1 Send email
  2 Fetch email
  Select an action: 0
> Logging out from imap.gmail.com...
> Quitting user agent...
```
