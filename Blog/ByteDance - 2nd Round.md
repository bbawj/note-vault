---
title: "ByteDance - 2nd Round"
date: 2022-11-25
lastmod: 2022-11-25
---
# ByteDance - 2nd Round
I applied to ByteDance for a backend engineering internship role in video infrastructure. Due to a lack of research, I found myself in a system design interview rather than a leetcode style one. My interviewer was an SRE - Software Reliability Engineer, who seemed quite surprised that I knew what an SRE was (thanks to my role at Shopee monitoring the stability of the UAT environment, which is not much unlike an SRE). 
## The Question
*"A poor business man wants to set up a new business, selling cloud storage to customers. However, he only has 4 old servers, each of 1 TB capacity."*

Questions in order:
1. Design the IO flow of such a system, to support basic features of uploading and downloading files given a specific file path.
2. How could we support multiple clients?
3. What if a client wants to upload a file larger than the capacit of a single server ( > 1 TB)?
4. One of the servers failed, causing data loss and large costs to compensate users for the lost/corrupted data. How could we improve the reliability of the 4 servers.
5. The businessman wants to be able to *oversell* his service. With only 4TB of storage, we need to be able to sell 8TB worth to customers. This is on the basis that not all customers will use all the storage they purchased.
## Wow I am bad
Pointers 4 and 5 are most interesting, and stumped me during the interview.

Question 4:
With only 4 servers, and the constraint of being poor, a RAID configuration was not feasible. I think the interviewer expected me to list some redudancy algorithms, but with no knowledge or experience, I couldn't provide any.

Question 5:
This question and the answers from the interviewer, brought into light the technical difficulties which the cloud storage services we use face everyday. One solution is to compress the files uploaded. If we can compress and decompress files on the fly while serving requests to clients, we can make the 4TB of storage go a long way. Pair this up with *virtual uploads*. 

Virtual uploads is based on the assumption that every individual does not *really* have many personal files or data. This means a majority of storage that most people ever really make use of, is for public files - files such as music, or the first season of The Office. Ever wondered what Google Drive is doing during the *Scanning File* portion when you upload a file? Apparently, the local file is being hashed in its entirety and sent to the server to check if Google already has the same file in its store. This means that multiple users will have pointers to the same file stored on the server, and the local file is only "virtually" uploaded.

The final step, and last resort, is to provision more servers. It is hard to provision servers out of thin air, and once bought, they start to add to the cost of the business. Here the interviewer mentioned how one could make use of the constraint on network and bandwith speeds as the time the business will have to provision more servers. 

## Life's tough
If it is not yet evident, I definitely did not come up with any of the responses above. A lot of very smart people have had to come up with such solutions and implement them in the real world. I wonder what other problems lie in plain sight, but have solutions which most will remain oblivious to.