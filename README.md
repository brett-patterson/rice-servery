# rice-servery
An API to interact with the Rice University servery menus.

## Monitor

Additionally, an executable monitor is included that can search through the menus for keywords and alert users if a matching menu item is found.

### Configuration

The monitor is configured through a JSON configuration file, which defaults to "config.json". An example of this file is given as
"config.json.template".

### Running

The monitor is provided as a Rust library with a thin executable wrapper around it. The executable provided only checks the current
menu against the provided rules once. To implement, real-time monitoring, consider running the executable as a CRON job.
