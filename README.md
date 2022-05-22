# Clock King

[![run-tests](https://github.com/krvoje/clockking/actions/workflows/main.yaml/badge.svg?branch=master)](https://github.com/krvoje/clockking/actions/workflows/main.yaml)

A console app for jotting down hours tracking for the day. 
Written to learn Rust with [Cursive](https://github.com/gyscos/cursive).

Meant as manual task list, where the results can be transferred to a time tracking app at the end of day.

*Disclaimer:* This is a weekend project that I did for fun, and ended up finding useful at work. 
It's by no means an app with guarantees on anything, but more akin to a script with a TUI I hacked for personal use.
I've had no issues with losing data, but that doesn't mean it's impossible in some corner cases. So, please bear that 
in mind when using the app, and approach it lightheartedly as I did, as a tool to have fun with, that's also useful.
Issues, comments, and suggestions are welcome.

# Functionality

## Clock entries list

This is the main screen, and contains the list of clock entries that can be checked once they are entered to a time tracking app.

Keyboard shortcuts are listed on the screen buttons, spacebar toggles whether an item is clocked or not.

A clock entry contains the following info (might not be contained in the screenshots until I update them):
- `From` - when the task started
- `To`- when the task ended
- `Client` - the client for which the task is (can be empty)
- `Project` - the project for which the task is (can be empty)
- `Description` - a description of the task
- `Duration` - the total (calculated) duration of the task
- `Clocked` - whether the task is clocked in your external time-tracking app

![Clock entries list](screenshots/task_list.png)

## Adding a clock entry

Pressing the `<(A)dd>` adds a new clock entry after the current selection in the table. Clock King tries to be smart and guess the end time of the task. 

![Add new entry](screenshots/add_clock_entry.png)

## Editing a clock entry

Pressing `Enter` opens the edit form for the current selection.

![Edit entry](screenshots/edit_clock_entry.png)

## Deleting a clock entry

Entries can be deleted from the list, and restored from the undo buffer.

![Edit entry](screenshots/delete_entry.png)

## Granularity

There is a time-granularity toggle for how detailed you want to be with your tracking.
- Relaxed (1h)
- Reasonable (30m)
- Detailed (15m)
- Paranoid (5m)
- OCD (1m)
- Scientific (1s)

Changing the granularity automatically adjusts existing entries.

![Granularity](screenshots/granularity.png)

## Record a new entry

By pressing `<Start (r)ecording>` you can submit details for a new entry you want to start recording. The start time
of the entry will be set to the current timestamp (rounded to the selected granularity). When you submit the data,
Clock Kings starts the recording.

![Start recording](screenshots/start_recording.png)

You can continue working while Clock King records your entry in the background.

![Recording](screenshots/recording.png)

By pressing `<Stop (r)ecording>` Clock King stops the recording at the current timestamp, and you can submit the updated
entry details.

![End Recording](screenshots/stop_recording.png)

## File database

The executable creates a `./.clockking/db.json` where the current list is stored. There's an autosave thread, and the data
gets saved when you quit the program normally.

