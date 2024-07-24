# udp-recast

A small utility for redirecting Multicasted UDP messages to a specific host.  An example of this would be when you have an application that needs to receive messages being produced on a different network (ferrying events from a weather station to an app running in a kubernetes cluster, perhaps)

## Usage
`udp-recast -- <port> <target>`
- `<port>` - the port to listen for multicasted messages on 
- `<target>` - the hostname or ip address of the indended recipient.
