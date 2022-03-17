---
title: centos服务器各种应用安装
date: 2020-06-05 21:43:33
tags:
  - centos
  - setup
---

- java
- mysql
- yum 源更换镜像
- nginx
- redis
- ssh 证书 Let’s Encrypt

---

## java

```bash
wget https://download.oracle.com/otn/java/jdk/8u251-b08/3d5a2bb8f8d4428bbe94aed7ec7ae784/jdk-8u251-linux-x64.tar.gz\?AuthParam\=1591100127_ba63c8b31debe96353a7ee64662e8a2f
```

**下载链接需要重新替换**
http://www.oracle.com/technetwork/java/javase/downloads/jdk8-downloads-2133151.html

### 安装

```bash
## 创建安装目录
mkdir /usr/local/java/
## 解压至安装目
tar -zxvf jdk-8u171-linux-x64.tar.gz -C /usr/local/java/

# 打开文件
vim /etc/profile
# 在末尾添加
export JAVA_HOME=/usr/local/java/jdk1.8.0_251
export JRE_HOME=${JAVA_HOME}/jre
export CLASSPATH=.:${JAVA_HOME}/lib:${JRE_HOME}/lib
export PATH=${JAVA_HOME}/bin:$PATH

#使环境变量生效
source /etc/profile

# 添加软链接
ln -s /usr/local/java/jdk1.8.0_251/bin/java /usr/bin/java
# 检查
java -version
```

---

## mysql

> MySQL 被 Oracle 收购后，CentOS 的镜像仓库中提供的默认的数据库也变为了 MariaDB

### 安装

```bash
# 检查MariaDB
shell> rpm -qa|grep mariadb
mariadb-server-5.5.60-1.el7_5.x86_64
mariadb-5.5.60-1.el7_5.x86_64
mariadb-libs-5.5.60-1.el7_5.x86_64

# 删除mariadb
如果不存在（上面检查结果返回空）则跳过步骤
shell> rpm -e --nodeps mariadb-server
shell> rpm -e --nodeps mariadb
shell> rpm -e --nodeps mariadb-libs

# 0.下载
wget 'https://dev.mysql.com/get/mysql57-community-release-el7-11.noarch.rpm'

# 1.添加Mysql5.7仓库
sudo rpm -ivh https://dev.mysql.com/get/mysql57-community-release-el7-11.noarch.rpm

# 2.确认Mysql仓库成功添加
sudo yum repolist all | grep mysql | grep enabled

# 如果展示像下面,则表示成功添加仓库:
mysql-connectors-community/x86_64  MySQL Connectors Community    enabled:     51
mysql-tools-community/x86_64       MySQL Tools Community         enabled:     63
mysql57-community/x86_64           MySQL 5.7 Community Server    enabled:    267

# 开始安装Mysql5.7
sudo yum -y install mysql-community-server

```

### 启动

```bash
# 启动
sudo systemctl start mysqld

# 设置系统启动时自动启动
sudo systemctl enable mysqld

# 查看启动状态
sudo systemctl status mysqld
```

### 配置

#### 修改密码

```bash
# MySQL第一次启动后会创建超级管理员账号root@localhost，初始密码存储在日志文件中：
shell> sudo grep ‘temporary password’ /var/log/mysqld.log

shell> mysql -uroot -p
# 修改密码
mysql> ALTER USER ‘root’@‘localhost’ IDENTIFIED BY ‘新密码’;
```

#### 允许 root 远程访问

```
mysql> GRANT ALL PRIVILEGES ON *.* TO 'root'@'%' IDENTIFIED BY '123456' WITH GRANT OPTION;
mysql> FLUSH PRIVILEGES;
```

#### 设置编码

编辑/etc/my.cnf，[mysqld]节点增加以下代码：

```bash
[mysqld]
character_set_server=utf8
init-connect=‘SET NAMES utf8’
```

> 参考: [CentOS 安装 MySQL 详解 - 掘金](https://juejin.im/post/5d07cf13f265da1bd522cfb6#heading-24)

---

## yum 源

```bash
ll /etc/yum.repos.d/
# 备份
mkdir /opt/centos-yum.bak
mv /etc/yum.repos.d/* /opt/centos-yum.bak/

#下载aliyun yum repo
# centos7
wget -O /etc/yum.repos.d/CentOS-Base.repo http://mirrors.aliyun.com/repo/Centos-7.repo

# 清缓存
yum clean all
yum makecache
```

---

## nginx

### 安装

```bash
# EPEL 仓库中有 Nginx 的安装包。
sudo yum install epel-release
sudo yum install nginx
```

### 启动

```bash
#设置 Nginx 开机启动：
sudo systemctl enable nginx

#启动 Nginx：
sudo systemctl start nginx
# 通过运行以下命令，来检查 Nginx 的运行状态：
sudo systemctl status nginx
```

---

## redis

### 安装

````bash
#下载fedora的epel仓库
yum install epel-release
# 安装redis数据库
yum install redis ```

### 启动
```bash
# 常见命令介绍
systemctl start redis #启动服务 
systemctl stop redis  #停止服务
systemctl restart redis  #重启服务
systemctl status redis   #查看服务状态
systemctl enable redis   #设置开机自启动
````

### 配置

> 设置 redis 远程连接和密码  
> vi /etc/redis.conf  
> a.找到 protected-mode 设置为 no  
>  protected-mode no  
> b. bind 设置为 0.0.0.0  
>  bind 0.0.0.0  
> c.取消 requirepass 注释,设置密码  
>  requirepass "password"

---

## Let's Encrypt

**需要先停止 nginx 服务**
生成证书验证域名的时候，需要启用 443 端口。

```bash
#安装工具
yum install certbot

# 获取证书， 按流程填写
sudo letsencrypt certonly --standalone
# 证书路径 /etc/letsencrypt/live/

# 自动更新测试
certbot renew --dry-run

# 定时任务查看
crontab -l
# 新建
crontab -e
# 隔 两个月的 凌晨 2:15 执行 更新操作，需要先停止ngxin在启动
15 2 * */2 * certbot renew --pre-hook "sudo systemctl stop nginx" --post-hook "sudo systemctl start nginx"  >> /var/log/certbot-renew.log

```
