# OpenRGB X Pcap
The remake of [`scapy-rgb`](https://github.com/p6nj/scapy-music/blob/main/SacpyRGB.py) in Rust a bit less easy-to-use but FAST (and resource friendly).
Note that the Python version was based on IPv4 addresses and this one uses only raw data.  
Try it on Linux using the script `auto-setup.sh`.
# How to use
You need OpenRGB running and its server listening on default port. Don't worry if something's not right, the errors are very descriptive.  
Runs on Linux (tested on Lubuntu) for now, compiled with the `libpcap` package installed with `apt`.
