from Util.yaml_creator import create_config
from logging import info, debug
import subprocess
import pytest
import datefinder

@pytest.mark.Smoke
@pytest.mark.parametrize("report_interval", [-1, 1, 2, 30])
def test_report_period(report_interval):
    """
    Testing the update period of the simulation from the monitor report
    Preconditions:
        1. Seconds configured for this test (default 2 seconds)
        2. num_messages configured for this test (default 1000 messages)
        3. 1 Sender: {'mean_time': 0.001, 'failure_rate': 0.50}
    Steps:
        1. Run the simulation
        2. Pull the logs reported from simulation
        3. Verify each MONITOR log that its reported within configured seconds increments
    Expected Results:
        1. The reporting is reported as expected with the configured seconds
    """
    info('Testing with the following report_interval: ' + str(report_interval))
    pytest_path = create_config(report_period=report_interval)
    results = subprocess.run("cargo run -- " + pytest_path, capture_output=True, text=True)
    prev_date = None
    assert not 'error: process didn\'t exit successfully' in results.stderr
    for line in results.stderr.split('\n'):
        debug(line)
        if 'MONITOR' in line:
            matches = list(datefinder.find_dates(line))
            if len(matches) > 0:
                # date returned will be a datetime.datetime object. here we are only using the first match.
                date = matches[0]
                debug(date)
                debug(prev_date)
                if prev_date:
                    assert (date - prev_date).seconds == report_interval
                prev_date = date
            else:
                info('No dates found')

@pytest.mark.Smoke
@pytest.mark.parametrize("num_messages", [-1, 0, 1000, 10000])
def test_successful_messages_monitor(num_messages):
    """
    Testing the succesful messages of the simulation from the monitor report
    Preconditions:
        1. Everything in default mode should be fine
    Steps:
        1. Run the simulation
        2. Pull the logs reported from simulation
        3. Verify the last MONITOR log reports the num_messages configured
    Expected Results:
        1. The reporting is reported as expected with the configured num_messages
    """
    info('Testing with the following num_messages: ' + str(num_messages))
    pytest_path = create_config(num_messages=num_messages)
    results = subprocess.run("cargo run -- " + pytest_path, capture_output=True, text=True)
    assert not 'error: process didn\'t exit successfully' in results.stderr
    assert 'MONITOR sent: ' + str(num_messages) in results.stderr

@pytest.mark.Smoke
@pytest.mark.parametrize("failure_rate", [-1, 0, 0.5, 1.0])
def test_failed_messages_monitor(failure_rate):
    """
    Testing the failed messages of the simulation from the monitor report
    Preconditions:
        1. None. Everything in default mode should be fine
    Steps:
        1. Run the simulation
        2. Pull the logs reported from simulation
        3. Verify the last MONITOR log reports the failed number of messages based on senders failure_rate 
    Expected Results:
        1. The reporting is reported as expected with the configured seconds
    """
    info('Testing with the following failure_rate: ' + str(failure_rate))
    senders = [
         {'mean_time': 0.001, 'failure_rate': failure_rate},
    ]
    pytest_path = create_config(senders=senders)
    results = subprocess.run("cargo run -- " + pytest_path, capture_output=True, text=True)
    assert not 'error: process didn\'t exit successfully' in results.stderr
    if failure_rate is 0:
        assert 'MONITOR sent: ' + str(1000) + ', failed: ' + str(failure_rate) in results.stderr
    else:
        assert 'MONITOR sent: ' + str(1000) + ', failed: ' + str(1000 * failure_rate) in results.stderr

# @pytest.mark.Smoke
@pytest.mark.skip()
def test_mean_time_monitor():
    """
    Testing the failed messages of the simulation from the monitor report
    Preconditions:
        1. None. Everything in default mode should be fine
    Steps:
        1. Run the simulation
        2. Pull the logs reported from simulation
        3. Verify the last MONITOR log reports the failed number of messages based on senders failure_rate 
    Expected Results:
        1. The reporting is reported as expected with the configured seconds
    """
    assert True