## Step1

replace revm to local crates for debugging

## Step2

record call traces(including internal calls) to extract key info

should focus on evm tracer mod. to be done...


目前打印的transact只是external call的tx信息

## 仔细解读

从forge bin的main文件为入口，以forge test为例
forge test (main.rs)
    rum (cmd/test.rs) 构建confi
        execute_tests (cmd/test.rs)


主要的调用栈在run_loop里实现，它会peek调用栈顶，然后把这个call里面包含的internal call都走完之后才会执行下一个call。
所以internal call其实是包含在这个逻辑里，调用execute的时候只是call/create的CallInputs不同。
所以可以从这里的信息拿到比较全的调用信息，另外，需要进一步查看revm执行的opcode sequence和state change的information。