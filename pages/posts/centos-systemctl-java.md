---
title: systemd运行java jar应用
date: 2020-06-09 22:24:59
tags:
  - config
  - centos
  - java
---

## systemd 启动 java 应用程序

**test.service**

```conf
# /etc/systemd/system
# 新建test.service， 替换成自己想要的app名字
[Unit]
Description=TestJava
After=network.target

[Service]
Type=forking
ExecStart=/var/www/test/start.sh
ExecStop=/var/www/test/stop.sh
# 除了使用systemctl stop test.service,其他情况退出会重启服务
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

**start.sh**

```bash
#!/bin/bash
echo "start test service"
nohup /usr/bin/java -jar /var/www/test/test.jar --server.port=9090 --spring.profiles.active=test >/dev/null 2>&1 &
echo $! > /var/www/test/app.pid
```

**stop.sh**

```bash
#/bin/sh
PID=$(cat /var/www/test/app.pid)
kill -9 $PID
rm -fr /var/www/test/app.pid
```

## 使用

```bash
# reload
systemctl daemon-reload
systemctl start test.service
systemctl stop test.service
systemctl status test.service
```
