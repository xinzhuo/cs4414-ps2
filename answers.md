Title: Problem Set 2 Answers
Author: Justin Washington, Xinxhuo Dong

1.
root       302  0.0  0.1   3256   784 ?        Ss   09:01   0:00 /sbin/udevd --daemon
root       504  0.0  0.0   3252   376 ?        S    09:01   0:00 /sbin/udevd --daemon
root       514  0.0  0.0   3252   352 ?        S    09:01   0:00 /sbin/udevd --daemon

On ubuntu, there is a process run by root named udevd that seems to need three processes.  Upon further investigation, I found that udev is a device manager for the linux kernel that handles adding and removing devices and loading firmware.   udevd is a background process that manages the virtual /dev directory.  A new process is added for each device added.

2.
In top, typing O and choosing the column you want to sort can allow you to sort by priority.  I noticed that a process named ksmd always had the highest priority, no matter what other applications I opened.  Through further investigation, I found that ksmd is Kernel SamePae Merging, a background service that scans page addresses to find duplicate pages and merges them to reduce memory usage.  


