# servery-monitor
A program to monitor the food at serveries around Rice University campus.

## Configuration

The monitor is configured through a JSON configuration file, which defaults to "config.json". An example of this file is given as
"config.json.template".

## Running

The monitor is provided as a Rust library with a thin executable wrapper around it. The executable provided only checks the current
menu against the provided rules once. To implement, real-time monitoring, consider running the executable as a CRON job.
