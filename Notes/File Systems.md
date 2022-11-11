# File Systems
## File
A file is an unstructured sequence of bytes. Each byte is individually addressable from the beginning of the file.
### File access
- Sequential: information is processed from the beginning of the file one byte after the other
- Direct access: bytes can be read in any order by referencing the byte number
### Protection
![](https://i.imgur.com/9acuiz6.png)
- Owner: Permissions used by the assigned owner of the file or directory
- Group: Permissions used by members of the group that owns the file or directory
- Other: Permissions used by all users other than the file owner, and members of the group that owns the file or the directory
### Data Structures
#### File Control Block
![400](https://i.imgur.com/xKpBYnL.png)
#### Open File Table
`open()` syscall:
- First searches the system wide OFT to see if it is being used by another process. If it is, per process open file table entry is created pointing to this. 
- If not, the directory structure is searched for the file. FCB is copied to the system wide OFT. Entry is made in per process OFT and a pointer to the entry is returned.
![](https://i.imgur.com/YG1FRQs.png)
The open file tables saves substantial overhead by serving as a cache for the FCB. Data blocks are *not* kept in memory, instead, when the process is closed, the FCB is entry is removed and the updated data is copied back to the disk.
#### File Descriptor
A file descriptor is a non-negative integer which indexes into a per-process file descriptor table which is maintained by the kernel. This in turn indexes into a system-wide open file table. It also indexes into the inode table that describes the actual underlying files. All operations are done on the file descriptor
### Storage allocation
File-Organisation Module: allocates storage space for files, translates logical block addresses to physical block addresses, and manages free disk space.
#### Contiguous
Each file occupies a set of contiguous blocks on the disk.
Advantages:
- Simple as only starting location and length is required
- Supports random access
Disadvantages:
- Finding a hole big enough may result in external fragmentation
- File space is constricted by size of the hole, it might need to be moved to a bigger hole in the future
- If file space is overestimated there will be internal fragmentation
![](https://i.imgur.com/YKfQr9s.png)
#### Linked
Each file is a linked list of disk blocks and the blocks may be scattered anywhere on the disk.
Advantages:
- Simple as only need starting address
- No wasted space and no constraint on file size
Disadvantages:
- No random access
*Assuming 4 bytes is reserved for the pointer to the next block:*
![](https://i.imgur.com/seAqKqD.png)
Why displacement need to + 4? #question
#### Indexed allocation
Each file has an index block which contains all pointers to the allocated blocks. Directory entry contains the block number of the index block. Similar to a [page table](Notes/Memory%20Organisation.md#^b8969e) for memory allocation.  
Advantages:
- Supports random access
- Dynamic storage allocation without external fragmentation
Disadvantages:
- Overhead in keeping index blocks
- Internal fragmentation as the last block that the index is pointing to may not be fully utilised
![](https://i.imgur.com/1sYk7IS.png)
#### inode
An inode is an index block. For each file and directory there is an inode. The inode contains file attributes, 12 pointers to direct blocks (data blocks) and 3 pointers point to indirect blocks (index blocks) with 3 levels of indirection.
![](https://i.imgur.com/0t64Mny.png)
Indirection allows the system to support large file sizes:
![](https://i.imgur.com/zFuqkwv.png)
Using the inode:
![](https://i.imgur.com/XzcIDIN.png)
## Directories
### Structure
A directory can be structured in two ways:
1. each entry contains a file name and other attributes
2. each entry contains a file name and a pointer to another data structure where file attributes can be found
![](https://i.imgur.com/n5wb4w0.png)

![](https://i.imgur.com/qw4qB6W.png)
### Organisation
A tree-like structure:
![](https://i.imgur.com/7FbJ3PU.png)
Path Name
- Absolute Path Name: begins at the root and follows a path down to the specific file, e.g., /spell/mail/prt/first
- Relative Path Name: Defines a path from the current directory, e.g. current directory is: /spell/mail relative path name for the above file is: prt/first
Characteristics:
- Efficient Searching: File can be easily located according to the path name.
- Naming: Files can have the same name under different directories.
- Grouping: files can be grouped logically according to their properties
### Linking
![](https://i.imgur.com/XBiK2Ib.png)
- A hard link points to the data on storage, while a soft link can point to another link which points to information on storage.
- Both linking strategies allow a separate file name to be used for the source file name. This source file name will resolve to the target file data by following the link.
![500](https://i.imgur.com/1kMzyvA.png)
## Disk Space Management
Block size affects both data rate and disk space utilisation
- Big block size: file fits into few blocks resulting in fast to find & transfer blocks, but wastes space if file does not occupy the entire last block
- Small block size: file may consist of many blocks resulting in slow data rate
![](https://i.imgur.com/ozMKoK9.png)
### Managing Free Blocks
There is a need to track which blocks are free in order to allocate disk space to files
#### Bitmap
Each block is represented by 1 bit, 1 (free) and 0 (allocated)
![](https://i.imgur.com/3T1eJaE.png)
Advantage:
- Simple and efficient to find the first free block via bit manipulation. i.e. Find the first non-0 word, and find the first bit 1 in the word.
Disadvantage:
- Takes up additional space as each block requires 1 bit
- Inefficient to look up this bitmap unless the entire map is kept in memory
#### Linked list
![](https://i.imgur.com/habWC4G.png)
The pointer to the next block is stored in the block itself, hence to read the entire list, each block must be read sequentially requiring substantial I/O time.
## Practice Problems
![](https://i.imgur.com/q4C26Gf.png)
a. False. Owner and the group which owner belongs to is able to read.
b. True.
c. False. Using linked file allocation, any free data block can be used.
![](https://i.imgur.com/3BDfuQU.png)
a. The previous links will now point to the data of the new file. To avoid this, dangling links need to be cleaned up.
b. 
Single copy
- Race conditions, mutual exclusion
Multiple copy
- Storage waste
- Inconsistency
![](https://i.imgur.com/VEuDidH.png)
a. 5 disk accesses
1. Load inode of usr
2. Load directory for usr
3. Load inode for ast
4. Load directory for ast
5. Load inode for mbox
b. Seek: no disk reads needed
Current position is 5900: Logical block 5, byte 900.
read(100): 1 disk read by following direct pointer
read(200): 2 disk read by following single indirect pointer
3 disk reads total
c. 
Number of pointers in 1 index block = $1000/2=500$
File size supported = $(6+500) \times 1000=506,000B$
![](https://i.imgur.com/VB9xuM7.png)
File data can be stored across different physical storage blocks. A smaller physical block helps to reduce internal fragmentation as the last block occupied by the file can is only 512B compared to 4KB. Using the larger block size would also help to improve throughput.