# chatbot

## Description
Q&amp;A Chatbot website with multithreaded web server in Rust

### Contributor(s)
* Tate Larkin

### Server
I created a multithreaded web server in Rust from scratch to serve the website. The server interacts with the machine learning model to dynamically process user questions and serve up a response. It uses a thread pool to hold threads which it can allocate to a task. Workers accept the code that needs to be executed and run that code on different threads in parallel, and when they complete their task, they return to the thread pool to accept a new task.
<br>
![threadPool](https://user-images.githubusercontent.com/70344865/168331541-b2df86bf-7cf0-433a-ba59-048ac61947ba.png)

### Machine Learning Model
I used a Transformer model as described in the [Attention is All you Need](https://proceedings.neurips.cc/paper/2017/file/3f5ee243547dee91fbd053c1c4a845aa-Paper.pdf) paper. This architecture utilizes only attention mechanisms to track relationships and find patterns in data. It uses self-attention to weight the significance of each part of the input data. Inputs are embedded, positionally encoded, and sent through an encoder and decoder with multiple iterations of Multi-Head Attention layers which runs through attention mechanisms multiple times in parallel. It then uses a final point-wise network for its predictions.
![structure](https://user-images.githubusercontent.com/70344865/168329344-4978b250-fd0d-4a78-a280-0fb277451f74.png)

### Dataset
