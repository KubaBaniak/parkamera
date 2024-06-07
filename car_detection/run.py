from crontab import CronTab
import subprocess
import os

cron = CronTab(user=True)

cron.remove_all()

current_file_absolute_path = os.path.dirname(os.path.realpath(__file__))
command = f'bash {current_file_absolute_path}/scripts/check_slots.sh > {current_file_absolute_path}/scripts/cronLogs.log 2>&1'
job = cron.new(command=command)

job.minute.every(1)

cron.write()

