### 实现功能

1.完善了任务控制块提供的属性

2.进入系统调用时对此次系统调用进行记录

3.记录任务第一次被调度的时刻和系统调用时刻距离首次调度时刻之间的时间

### 简答作业

1.三个应用程序的报错信息依次为：

```
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
```

版本信息： `RustSBI version 0.4.0-alpha.1, adapting to RISC-V SBI v2.0.0`

2.

​	1.刚进入 `__restore` 时，`a0` 代表了`trap_handler`的返回值，即中断上下文的值

​	2.恢复`sstatus`，`sepc`和`sscratch`寄存器的值，`sstatus`记录了特权切换时CPU的特权级，`sepc`的值是中断结束后处理器要执行的下一条指令的地址，`sscratch`的值是用户栈的栈指针

​    3.跳过`x2`和`x4`的原因是这两个寄存器没有保存的必要，因此也不用回复。不保存`x2`的原因是因为`x2`在函数运行过程中指向内核栈指针`sp`。`x4`是线程寄存器，非特殊原因一般不会用到，所以无需保存

​    4.该指令使得`sp`和`sscratch`的值发生了交换。这条指令执行之前`sp`指向内核栈，`sscratch`指向用户栈。执行之后`sp`指向用户栈，`sscratch`指向内核栈

​    5.状态切换发生在`sret`指令，原因：执行该指令后CPU 会将当前的特权级按照 `sstatus` 的 `SPP` 字段设置为 U 或者 S，从而恢复到异常/中断之前的处理器状态

​    6.`sp`指向内核栈，`sscratch`指向用户栈

​    7.`ecall`指令

### 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   > *暂无交流*

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   > *《你参考的资料说明》*
   >
   > [实现特权级的切换 - rCore-Camp-Guide-2024A 文档](https://learningos.cn/rCore-Camp-Guide-2024A/chapter2/4trap-handling.html)
   >
   > [RISC-V ISA 学习笔记（4）函数调用约定+RV32G列表及对应的汇编伪指令表_riscv调用函数-CSDN博客](https://blog.csdn.net/new_horizon_/article/details/91814223)

\3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

\4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。