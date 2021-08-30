### Why ? ###
I read through [this](https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html) article on spinlocks and decided to try replicate the pitfalls. 

**Surpise!**: After a couple of tries, with a relatively small amount of threads (100) the current thread was preempted and the application entered a deadlock.