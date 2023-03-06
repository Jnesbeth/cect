import yaml
import os
from logging import debug

def create_config(num_messages=1000, report_period=2.0, senders=None):
    """
    Configuration creation for the simulation using yaml
    @param num_messages: limit of messages the producer is supposed to create
    @param report_period: interval of time the monitor processes is to log the progress
    """
    if not senders:
        debug('Using the default senders')
        senders = [
                {'mean_time': 0.001, 'failure_rate': 0.50},
                { 'mean_time': 0.010, 'failure_rate': 0.45 },
                { 'mean_time': 0.015, 'failure_rate': 0.40 },
                { 'mean_time': 0.020, 'failure_rate': 0.35 },
                { 'mean_time': 0.025, 'failure_rate': 0.30 },
                { 'mean_time': 0.030, 'failure_rate': 0.25 },
                { 'mean_time': 0.035, 'failure_rate': 0.20 },
                { 'mean_time': 0.040, 'failure_rate': 0.15 },
                { 'mean_time': 0.045, 'failure_rate': 0.05 },
                { 'mean_time': 0.050, 'failure_rate': 0.01 },
                { 'mean_time': 0.01, 'failure_rate': 0.3 }
            ]
    dict_file = {
        'num_messages': num_messages,
        'report_period': report_period,
        'senders': senders
    }
    pytest_path = os.path.join(os.getcwd(), 'pytest_config.yml')
    with open(pytest_path, 'w') as file:
        yaml.dump(dict_file, file)
    return pytest_path


# Previous code that was using docker containers to run the rust simulation. Had issues with getting the file to mount into the container and run properly.
# client = docker.from_env()
# container = client.containers.run("rust_interview_test", command='cargo run -- pytest_confg.yml',volumes={
#     pytest_path : {
#         'bind': '/pytest_confg.yml',
#         'mode': 'ro',
#     }
# })
# info(container)
# for line in container.logs(stream=True):
#     info(line)