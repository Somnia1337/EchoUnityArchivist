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
> Logging in is required before interacting with the SMTP/IMAP server.
  Server domain (eg. "smtp.qq.com"): smtp.gmail.com (your target SMTP server)
  Email address: amy1024@gmail.com (your corresponding email address)
  SMTP/IMAP password (eg. "jfoaiwnpsej"): ioaenfkuhvkusanelx (something like this, google "gmail smtp password" for help)
```

If nothing goes wrong, the user agent would be connected to your server, and would ask you to select an action:

```text
> Connected to smtp.gmail.com.
> Connected to imap.gmail.com.
> Actions:
  0 Logout & quit
  1 Send email
  2 Fetch message
  Select an action: 2 (enter the number for the action you want)
```

## Send email

Enter "1" for this action, the agent would then ask you for details: 

```text
  Select an action: 1
> New draft:
  -------------------------------------
  To: bob2048@gmail.com (the receiver's address)
  Subject: Hi from Amy! (title)
  Body (press 2 `Enter`s in a row to finish):
  Hi Bob!
  Nice 2 c u 2!


  -------------------------------------
```

When you've finished editing the `Body` part, press `Enter` 2 times in a row to let the agent know, and it would let you reconfirm if everything is correct:

```text
> You have finished editing,
  if everything looks fine,
  enter "yes" to confirm sending: yes (only "yes" leads to sending)
```

If sending succeeds, the agent would print a message:

```text
> Email sent to bob2048@gmail.com successfully!
```

## Fetch message

Enter "2" for this action, the agent would then ask you to choose an inbox:

```text
> Fetching inboxes...
  [1] INBOX
  [2] Sent Messages
  [3] Drafts
  [4] Deleted Messages
  [5] Junk
  Select an inbox:
```

After you've selected one, the agent would then check if there's email and fetch the first one if so:

```text
  Select an inbox: 1
> Message fetched:
  -------------------------------------
  From: bob2048@gmail.com
  To: amy1024@gmail.com
  Subject: Hi from Bob!
  Date: Sun, 07 Apr 2024 09:09:27 +0000
  
  Hi Amy!
  Nice 2 c u!
  
  -------------------------------------
```

## A complete example

```shell
cargo run
```

```text
> Logging in is required before interacting with the SMTP/IMAP server.
  Server domain (eg. "smtp.qq.com"): smtp.gmail.com
  Email address: amy1024@gmail.com
  SMTP/IMAP password (eg. "jfoaiwnpsej"): ioaenfkuhvkusanelx
> Connected to smtp.gmail.com.
> Connected to imap.gmail.com.
> Actions:
  0 Logout & quit
  1 Send email
  2 Fetch message
  Select an action: 2
> Fetching inboxes...
  [1] INBOX
  [2] Sent Messages
  [3] Drafts
  [4] Deleted Messages
  [5] Junk
  Select an inbox: 1
> Message fetched:
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
  2 Fetch message
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
  2 Fetch message
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
  2 Fetch message
  Select an action: 0
> Logging out from imap.gmail.com...
> Quitting user agent...
```
