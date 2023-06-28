# `blitz-tanks`

🌟 Experimental recommender system for World of Tanks Blitz

![Latest GitHub tag](https://img.shields.io/github/v/tag/eigenein/blitz-tanks)
![GitHub workflow status](https://img.shields.io/github/actions/workflow/status/eigenein/blitz-tanks/check.yaml)

# Developer's notes

## systemd unit

```unit file (systemd)
[Unit]
After = network.target network-online.target
BindsTo = network-online.target

[Service]
Type = simple
User = pi
WorkingDirectory = /home/pi/.blitz-tanks
ExecStart = /home/pi/bin/blitz-tanks web

[Install]
WantedBy = multi-user.target
```

## Keep back MongoDB on Raspberry Pi

```shell
sudo apt-get install mongodb-org-mongos=4.4.18 mongodb-org-tools=4.4.18 mongodb-org-shell=4.4.18 mongodb-org-database-tools-extra=4.4.18 mongodb-org=4.4.18 mongodb-org-server=4.4.18
sudo apt-mark hold mongodb-org-mongos mongodb-org-tools mongodb-org-shell mongodb-org-database-tools-extra mongodb-org mongodb-org-server
```

## Hourly `mongodump`

```text
0 * * * * mongodump --uri="mongodb://localhost/blitzTanks" --gzip --archive=.blitz-tanks/backups/"$(date +\%Y\%m\%d-\%H\%M).tar.gz"
```

## Hourly trainer

```text
0 * * * * BLITZ_TANKS_ENVFILE=.blitz-tanks/.env systemd-cat -t blitz-tanks blitz-tanks trainer fit
```
