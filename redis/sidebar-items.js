initSidebarItems({"enum":[["ConnectionAddr","Defines the connection address."]],"fn":[["cmd","Shortcut function to creating a command with a single argument."],["pack_command","Packs a bunch of commands into a request.  This is generally a quite useless function as this functionality is nicely wrapped through the `Cmd` object, but in some cases it can be useful.  The return value of this can then be send to the low level `ConnectionLike` methods."],["parse_redis_url","This function takes a redis URL string and parses it into a URL as used by rust-url.  This is necessary as the default parser does not understand how redis URLs function."],["parse_redis_value","Parses bytes into a redis value."],["pipe","Shortcut for creating a new pipeline."],["transaction","This function simplifies transaction management slightly.  What it does is automatically watching keys and then going into a transaction loop util it succeeds.  Once it goes through the results are returned."]],"struct":[["Client","The client type."],["Cmd","Represents redis commands."],["Connection","Represents a stateful redis TCP connection."],["ConnectionInfo","Holds the connection information that redis should use for connecting."],["Iter","Represents a redis iterator."],["Msg","Represents a pubsub message."],["Parser","The internal redis response parser."],["Pipeline","Represents a redis command pipeline."],["PubSub","Represents a pubsub connection."],["Script","Represents a lua script."],["ScriptInvocation","Represents a prepared script call."]],"trait":[["Commands","Implements common redis commands for connection like objects.  This allows you to send commands straight to a connection or client.  It is also implemented for redis results of clients which makes for very convenient access in some basic cases."],["ConnectionLike","Implements the \"stateless\" part of the connection interface that is used by the different objects in redis-rs.  Primarily it obviously applies to `Connection` object but also some other objects implement the interface (for instance whole clients or certain redis results)."],["IntoConnectionInfo","Converts an object into a connection info struct.  This allows the constructor of the client to accept connection information in a range of different formats."],["PipelineCommands","Implements common redis commands for pipelines.  Unlike the regular commands trait, this returns the pipeline rather than a result directly.  Other than that it works the same however."]]});