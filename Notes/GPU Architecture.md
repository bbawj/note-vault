---
title: "GPU Architecture"
---
# GPU Architecture
The general purpose CPU is designed for single-threaded code optimised for *low latency.* The GPU allows us to achieve higher throughput in exchange for *higher latency.*

Need to achieve massive data parallelism for computing tasks such as vector processing and Multiplication and Accumulation (MAC) operations in matrices.
![](https://i.imgur.com/48iqtPk.png)
SIMD: Single instruction multiple data
## CUDA
### Architecture
![](https://i.imgur.com/bY9kIUJ.png)
### Programming Model
CUDA works on a heterogeneous programming model that consists of a host and device. Host calls the device to run the program.
- Host: CPU
- Device: GPU
#### Programming Language
![](https://i.imgur.com/NbyCCGb.png)
The source code is split into host (compiled by standard compilers like gcc) and device components (compiled by nvcc).
#### Kernel
![](https://i.imgur.com/8TmMsih.png)

![](https://i.imgur.com/RXDV5yP.png)
#### Threads and Thread Blocks
![](https://i.imgur.com/o3ntZg1.png)
We can access important properties of the kernel:
- Block ID: `blockIdx.x` gives us the ID of the thread block
- Thread ID: `threadIdx.x` gives us the ID of the thread within a thread block
- Dimension: `blockDim.x` gives us the number of threads per block 
The exact thread number can be found using `blockIdx.x* blockDim.x + threadIdx.x`
Multi-dimensionality:
![](https://i.imgur.com/j8TzUOB.png)
#### Synchronisation
![](https://i.imgur.com/t6Xbz4I.png)
### Memory management
![](https://i.imgur.com/mO4Shm7.png)

![](https://i.imgur.com/zXFsfYn.png)
The above code does not take advantage of GPU parallelism in the CUDA core. We can create 1 block with 3 threads to achieve parallelism: `vector_add_cu<<<1,3>>>(d_c, d_a, d_b);`
Use the threadIdx to access the memory:
![](https://i.imgur.com/d70gRtX.png)
> [! Threads vs Blocks]
> The example can also be achieved using 3 blocks each with 1 thread. However, parallel threads have the advantage to directly communicate and synchronise with each other due to shared hardware. Sharing memory between blocks would require *global memory access*
![](https://i.imgur.com/5vxKQ9c.png)
### Example
![](https://i.imgur.com/B5iK4Ky.png)
```c++
//initialize 1 block and 3 threads. We cannot use 3 blocks for this implementation as blocks would nt be able to share the local variable memory
Dot_prod_cu<<<1,3>>>(d_c, d_a, d_b);

__global__ void dot_prod_cu(int *d_c, int *d_a, int *d_b){
	//use __shared__ to allow threads to share data
	__shared__ int tmp[3];
	
	int i = threadIdx.x;
	tmp[i] = d_a[i] * d_b[i];

	//wait for all threads to complete to prevent premature entering into if block
	__syncthreads();
	
	if (i==0){
		int sum = 0;
		for (int j = 0; j < 3; j++)
		sum = sum + tmp[j];
		*d_c = sum;
	}
}
```
### Internal Operations
- Each SM contains multiple SP cores. Each core can only execute 1 thread.
- Each block of threads can be scheduled on any available SM by the runtime system, but 1 block can only exist on 1 SM.
#### Warps
![](https://i.imgur.com/ObcIEOG.png)
![](https://i.imgur.com/ENSz0mN.png)
#### SIMT
Warps enable a unique architecture called Single Instruction Multiple Thread. This means each warp executes only one common instruction for all threads.

Within a single thread, its instructions are
- pipelined to achieve instruction-level parallelism
- issued in order, with no branch prediction and speculative execution
Individual threads in a warp start together, at the same instruction address
- but each has its own instruction address counter and registers
- free to branch and execute independently when the thread diverges, such as due to data-dependent conditional execution and branch.
#### Thread Divergence
Branch statements will result in some threads in a warp wasting their clock cycles. This is because the threads in the warp must all execute the same instruction. For some which satisfy the condition, computation is done else NOP.
![](https://i.imgur.com/D0DNFIi.png)
## Practice Problems
![](https://i.imgur.com/zf3Aapc.png)
```c
__global__ void stencil(int N, int *input, int *output) {
	blockNum = blockIdx.x;
	i = threadIdx.x + blockNum * N;
	int sum = input[i];
	for(int i = 1; i < 3; i++) {
		sum += input[i-i]
		sum += input[i+i]
	}
}
int N = len(input) / BLOCK_SIZE
output = (int *) malloc(N * sizeof(int))
stencil<<<N, BLOCK_SIZE>>>(N, input, output)
```
