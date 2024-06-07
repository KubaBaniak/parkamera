from crontab import CronTab
import subprocess

cron = CronTab(user=True)

job = cron.new(command='./scripts/check_slots.sh')

job.minute.every(1)

cron.write()
