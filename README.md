# Questions
Suppose we added the following bytecode instructions to our language:
- `SEND_CHANNEL`:
    Pops the channel and a value from the stack and send the value on the channel using a blocking send
- `RECV_CHANNEL`:
    Pops the channel from the stack, receives a value from the channel (this may block), and push the resulting value back onto the stack
- `SPAWN`:
    Pop two functions from the stack and spawn them as concurrent tasks 

Describe in a few sentences how each bytecode instruction could be interpreted, and how your interpreter or language runtime could deal with the blocking nature of the send and the receive instructions.

## Answer

The presence of such instructions implies the need to implement at least a context-switching mechanism between tasks for the interpreter and a logic for choosing how and which task to perform. 

- `SEND_CHANNEL` - In a concurrent programming language, this operation could potentially block if the channel is full. Interpreter would need to handle this potential blocking situation, typically by suspending the execution of the current task until the channel is ready to receive more data. This requires a concurrent or parallel runtime system that supports task suspension and resumption, and communication between tasks via channels.

- `RECV_CHANNEL` - As with `SEND_CHANNEL`, this operation could block if there's no data available in the channel to read. Interpreter needs to handle this situation, typically by suspending the current task until there's data available in the channel. Once the data is available, the task can be resumed and the received data is pushed onto the stack. The mechanism for "waking up" a task depends on the implementation of the scheduler.

- `SPAWN` - This instruction doesn't involve blocking per se, but it does involve creating and managing concurrent tasks. Interpreter would need to provide support for task creation, scheduling, and synchronization. These tasks would then run concurrently with the existing execution context, potentially interacting with it and with each other via shared memory or channels.

In a practical implementation, the runtime system might use concepts like green threads or fibers for lightweight concurrency, along with synchronization primitives like channels, locks, and condition variables to coordinate between tasks. The specific choice of techniques and mechanisms would depend on the needs and constraints of the target execution environment.
