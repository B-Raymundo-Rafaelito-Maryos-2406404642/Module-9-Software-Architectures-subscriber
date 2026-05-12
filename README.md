1. What is amqp?

    AMQP stands for Advanced Message Queuing Protocol. It is an open standard protocol for message-oriented middleware that enables applications to communicate with each other by sending and receiving messages. AMQP provides a way to ensure reliable and secure communication between different systems, regardless of their underlying technology.

2. What does it mean? guest:guest@localhost:5672 , what is the first guest, and what is the second guest, and what is localhost:5672 is for?

    In the context of the connection string "guest:guest@localhost:5672", the first "guest" refers to the username, and the second "guest" refers to the password. "localhost:5672" indicates that the RabbitMQ server is running on the local machine (localhost) and is listening for connections on port 5672, which is the default port for AMQP communication.

Screenshot: 
![Simulation slow subscriber](assets/simulation_slow_subscriber.png)
I ran "cargo run" for 5 times quickly. And, in the screenshot above, the total number of queued messages is 15, which means that the subscriber program has received but has not yet processed 15 messages from the publisher. The subscriber is simulating a slow processing time by sleeping for 1000 milliseconds (1 second) before acknowledging each message. This results in a backlog of messages in the queue, as the subscriber is not able to keep up with the rate at which messages are being published.
