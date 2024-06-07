from crontab import CronTab
import subprocess
import os

cron = CronTab(user=True)

cron.remove_all()

current_file_absolute_path = os.path.dirname(os.path.realpath(__file__))
job = cron.new(command=f'bash {current_file_absolute_path}/scripts/check_slots.sh')

job.minute.every(1)

cron.write()

