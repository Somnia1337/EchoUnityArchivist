[中文](https://github.com/Somnia1337/EchoUnityArchivist/blob/main/README-ZH.md)

# [EchoUnityArchivist](https://github.com/Somnia1337/EchoUnityArchivist)

A sketchy email user agent, following the requirements of project #2 in Computer Networking, implemented in Rust.

"Echo Unity Archivist" is a fancier name for "email user agent", they start with the same letters.

# Course Project Requirements

- (given up) ~~Users can download emails from their email box through the agent.~~
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
Username: amy2233@gmail.com (your corresponding email address)
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
