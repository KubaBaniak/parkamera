from crontab import CronTab
import subprocess
import os

cron.remove_all()

cron = CronTab(user=True)

current_file_absolute_path = os.path.dirname(os.path.realpath(__file__))
job = cron.new(command=f'bash {current_file_absolute_path}/scripts/check_slots.sh')

job.minute.every(1)

cron.write()

