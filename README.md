MQTT Visualizer
---

This is a simple CLI utility for visualizing MQTT packets from their payload.

## Usage

To use, run this binary, and then input the hex of each packet you want to
visualize, one packet on each line. It will output the contents of that MQTT
packet to the terminal.

```
> mqtt-visualizer
e000
Data: e000
Disconnect
```
