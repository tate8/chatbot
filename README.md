# translator

## Description
A English-to-Spanish translation website running on a multithreaded web server.\
Check out [this notebook](https://github.com/tate8/neural-machine-translation) for the ML code.

### Contributor(s)
* Tate Larkin

### Server
I created a multithreaded web server in Rust from scratch to serve the website. The server interacts with the machine learning model to dynamically process user questions and serve up a response. It uses a thread pool to hold threads which it can allocate to a task. Workers accept the code that needs to be executed and run that code on different threads in parallel, and when they complete their task, they return to the thread pool to accept a new task.
<br><br>
<p align="center">
 <img src="https://user-images.githubusercontent.com/70344865/168331541-b2df86bf-7cf0-433a-ba59-048ac61947ba.png"></img>
</p>

### Machine Learning Model
I used a Transformer model as described in the [Attention is All you Need](https://proceedings.neurips.cc/paper/2017/file/3f5ee243547dee91fbd053c1c4a845aa-Paper.pdf) paper. This architecture utilizes only attention mechanisms to track relationships and find patterns in data. It uses self-attention to weight the significance of each part of the input data. Inputs are embedded, positionally encoded, and sent through an encoder and decoder with multiple iterations of Multi-Head Attention layers which runs through attention mechanisms multiple times in parallel. It then uses a final point-wise network for its predictions.
<br>

Scaled Dot Product Attention and Multi-Head Attention diagrams
<p align="middle">
  <img width="300" alt="ScaledDotProductAttn" src="https://user-images.githubusercontent.com/70344865/168853068-6bbf752d-2294-4e03-9f01-55bae063268b.png">
 <img src="https://user-images.githubusercontent.com/70344865/168852860-f858a97f-0eb9-4dfe-8b7f-609a32f1db84.png" width="300"></img>
</p>

<br>

Full Model
<p align="center">
  <img src="https://user-images.githubusercontent.com/70344865/168329344-4978b250-fd0d-4a78-a280-0fb277451f74.png" width="400" height="600"></img>
</p>


### Dataset
Tensorflow English-Spanish dataset

### Website
For the website design, I mimicked a messaging app such as Apple's Messages and other texting software. I used Mobile First design tactics to ensure great quality and responsiveness on any screen size.
