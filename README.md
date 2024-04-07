[中文](https://github.com/Somnia1337/EchoUnityArchivist/blob/main/README-ZH.md)

# [EchoUnityArchivist](https://github.com/Somnia1337/EchoUnityArchivist)

A sketchy email user agent, following the requirements of project #2 in Computer Networking, implemented in Rust.

"Echo Unity Archivist" is a fancier name for "email user agent", they start with the same letters.

# Course Project Requirements

- Users can download emails from their email box through the agent.
- Users can send their emails through the agent.
- The agent can access at least 2 real email servers.

# Getting Started

Run the user agent:

```shell
cargo run
```

It would ask you to login, here's an example:

```text
You must login before interacting with the server.
SMTP server: smtp.gmail.com (your target SMTP server)
Email: amy2233@gmail.com (your corresponding email address)
SMTP password: jfoiasnelmcpox (something like this, google "gmail smtp password" for help)
```

If nothing goes wrong, the user agent would be connected to your server, and would ask you to choose an action:

```text
> Connected to SMTP server.
> Options:
> 0 Quit
> 1 Send an email
Select an option: 1 (enter the number for the action you want)
```

## Send an email

Enter "1" for this action, then the agent would ask you for details: 

```text
Select an option: 1
To: bob1479@qq.com (the receiver's address)
Subject: Hi! (title)
Body (press 2 "Enter"s in a row to finish): (enter the email's body)
(some more)
(lines)
(...)


```

When you've finished editing the `Body` part, press "Enter" 2 times in a row to let the agent know, and it would let you reconfirm if everything is correct:

```text
> Seems that you've finished editing,
  if everything looks fine,
  enter "yes" to confirm sending: yes (only "yes" leads to sending)
```

If sending succeeds, the agent would print a message:

```text
> Email sent to bob1479@qq.com successfully!
```

## A complete example

```shell
cargo run
```

```text
> Logging in is required before interacting with the SMTP/IMAP server.
  Server domain (eg. "qq.com" without header): qq.com
  Email: 2581063732@qq.com
  Password (eg. "jfoaiwnpsej" SMTP/IMAP password): jjdoevmqlzfcdidh
> Connected to smtp.qq.com.
> Connected to imap.qq.com.
> Actions:
  0 Logout & quit
  1 Send an email
  2 Read an email
  Choose one: 1
> Editing email:
  To: 2581063732@qq.com
  Subject: Real Final Test
  Body (press 2 "Enter"s in a row to finish):
  This
  time,
  it's    
  real.


> Seems that you've finished editing,
  if everything looks fine,
  enter "yes" to confirm sending: yes
> Sending...
> Email sent to 2581063732@qq.com successfully.
> Actions:
  0 Logout & quit
  1 Send an email
  2 Read an email
  Choose one: 2
> Inboxes you can choose from:
  [1] INBOX
  [2] Sent Messages
  [3] Drafts
  [4] Deleted Messages
  [5] Junk
  Choose an inbox: 1
> Email fetched:
-------------------------------------
From: 2581063732@qq.com
To: 2581063732@qq.com
Subject: Real Final Test
Date: Sun, 07 Apr 2024 09:09:27 +0000

This
time,
it's
real.

-------------------------------------
> Actions:
  0 Logout & quit
  1 Send an email
  2 Read an email
  Choose one: 0
> Logging out from imap.qq.com...
> Quitting user agent...
```
