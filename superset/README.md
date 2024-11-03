# 安装 Superset

## 安装

注意使用 python 3.9 - 3.11 版本。3.12 目前不支持。

```bash
python3.11 -m venv venv
source venv/bin/activate
pip install pip --upgrade
pip install apache-superset
```

## 配置

需要在本地使用 `superset_config.py` 配置文件，更新 `SECRET_KEY`，内部配置数据库连接字符串，并安装 `psycopg2` 库。

```bash
export FLASK_APP=superset
pip install psycopg2
```
