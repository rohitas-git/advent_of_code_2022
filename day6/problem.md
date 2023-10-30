# Problem Context

Given a malfunctioning communication device
Device needs to lock on to their signal to communicate with them

- The signal is a series of seemingly-random characters that the device receives one at a time.
- To fix the communication system, you need to add a subroutine to the device that detects a start-of-packet marker in the datastream. 
- In the protocol being used by the Elves, the start of a packet is indicated by a sequence of four characters that are all different.

your subroutine needs to identify the first position where the four most recently received characters were all different. Specifically, it needs to report the number of characters from the beginning of the buffer to the end of the first such four-character marker.

## Part 2

A start-of-message marker is just like a start-of-packet marker, except it consists of 14 distinct characters rather than 4.